use rustc_errors::DiagDecorator;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{
    BindingMode, Block, Expr, ExprKind, HirId, LetStmt, LoopSource, MatchSource, Pat, PatKind,
    Stmt, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::def_id::DefId;
use rustc_span::sym;

use crate::no_repeated_collection_search::local_binding;

rustc_session::declare_lint! {
    pub(crate) NO_LOOP_APPEND_COMPREHENSION,
    Allow,
    "LEG076: prefer map and collect for simple vector-building loops"
}

rustc_session::impl_lint_pass!(NoLoopAppendComprehension => [NO_LOOP_APPEND_COMPREHENSION]);

pub(crate) struct NoLoopAppendComprehension;

impl<'tcx> LateLintPass<'tcx> for NoLoopAppendComprehension {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        if let Some(loop_expr) = candidate(cx, block) {
            report(cx, loop_expr);
        }
    }
}

fn candidate<'tcx>(cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let (binding, loop_expr, returned) = final_loop(block)?;
    let output = fresh_vector(cx, binding)?;
    if local_binding(cx, returned) != Some(output) {
        return None;
    }
    let (input, pattern, body) = for_parts(loop_expr)?;
    let item = primitive_binding(cx, pattern)?;
    if !is_native_input(cx, input) {
        return None;
    }
    let value = push_value(cx, only_expression(body)?, output)?;
    is_pure_value(cx, value, item).then_some(loop_expr)
}

fn final_loop<'hir>(
    block: &Block<'hir>,
) -> Option<(&'hir LetStmt<'hir>, &'hir Expr<'hir>, &'hir Expr<'hir>)> {
    let (returned, statements) = returned_value(block)?;
    let (last, preceding) = statements.split_last()?;
    let StmtKind::Let(binding) = preceding.last()?.kind else {
        return None;
    };
    Some((binding, statement_expr(last)?, returned))
}

fn returned_value<'hir>(block: &Block<'hir>) -> Option<(&'hir Expr<'hir>, &'hir [Stmt<'hir>])> {
    if let Some(tail) = block.expr {
        return Some((tail, block.stmts));
    }
    let (last, preceding) = block.stmts.split_last()?;
    let ExprKind::Ret(Some(value)) = statement_expr(last)?.kind else {
        return None;
    };
    Some((value, preceding))
}

fn fresh_vector(cx: &LateContext<'_>, binding: &LetStmt<'_>) -> Option<HirId> {
    let PatKind::Binding(BindingMode::MUT, id, _, None) = binding.pat.kind else {
        return None;
    };
    let initializer = binding.init?;
    let is_generated = binding.els.is_some() || crate::is_macro_expansion(initializer);
    if is_generated {
        return None;
    }
    let ExprKind::Call(callee, []) = initializer.kind else {
        return None;
    };
    is_vec_method(cx, associated_function(cx, callee)?, "new").then_some(id)
}

fn associated_function(cx: &LateContext<'_>, expr: &Expr<'_>) -> Option<DefId> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let ExprKind::Path(path) = expr.kind else {
        return None;
    };
    match cx.qpath_res(&path, expr.hir_id) {
        Res::Def(DefKind::AssocFn, method) => Some(method),
        _ => None,
    }
}

fn for_parts<'hir>(
    expr: &'hir Expr<'hir>,
) -> Option<(&'hir Expr<'hir>, &'hir Pat<'hir>, &'hir Block<'hir>)> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let ExprKind::Match(iterator, [arm], MatchSource::ForLoopDesugar) = expr.peel_drop_temps().kind
    else {
        return None;
    };
    let ExprKind::Call(_, [input]) = iterator.kind else {
        return None;
    };
    let ExprKind::Loop(body, None, LoopSource::ForLoop, _) = arm.body.kind else {
        return None;
    };
    let (pattern, body) = iteration_body(only_expression(body)?)?;
    Some((input, pattern, body))
}

fn iteration_body<'hir>(expr: &Expr<'hir>) -> Option<(&'hir Pat<'hir>, &'hir Block<'hir>)> {
    let ExprKind::Match(_, [_, arm], MatchSource::ForLoopDesugar) = expr.kind else {
        return None;
    };
    let PatKind::Struct(_, [field], None) = arm.pat.kind else {
        return None;
    };
    let ExprKind::Block(body, None) = arm.body.kind else {
        return None;
    };
    Some((field.pat, body))
}

fn only_expression<'hir>(block: &Block<'hir>) -> Option<&'hir Expr<'hir>> {
    match (block.stmts, block.expr) {
        ([], Some(expr)) => Some(expr),
        ([statement], None) => statement_expr(statement),
        _ => None,
    }
}

fn statement_expr<'hir>(statement: &Stmt<'hir>) -> Option<&'hir Expr<'hir>> {
    match statement.kind {
        StmtKind::Expr(expr) | StmtKind::Semi(expr) => Some(expr),
        _ => None,
    }
}

fn primitive_binding(cx: &LateContext<'_>, pattern: &Pat<'_>) -> Option<HirId> {
    let PatKind::Binding(BindingMode::NONE, id, _, None) = pattern.kind else {
        return None;
    };
    let ty = cx.typeck_results().node_type(pattern.hir_id);
    let is_primitive = ty.peel_refs().is_primitive();
    let is_copy = cx.type_is_copy_modulo_regions(ty);
    (is_primitive && is_copy).then_some(id)
}

fn is_native_input(cx: &LateContext<'_>, input: &Expr<'_>) -> bool {
    let should_skip =
        crate::is_macro_expansion(input) || local_binding(cx, input.peel_borrows()).is_none();
    if should_skip {
        return false;
    }
    let ty = cx.typeck_results().expr_ty(input).peel_refs();
    let is_vec = ty
        .ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_diagnostic_item(sym::Vec, definition.did()));
    ty.is_array() || ty.is_slice() || is_vec
}

fn push_value<'hir>(
    cx: &LateContext<'hir>,
    expr: &'hir Expr<'hir>,
    output: HirId,
) -> Option<&'hir Expr<'hir>> {
    let ExprKind::MethodCall(_, receiver, [value], _) = expr.kind else {
        return None;
    };
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let method = cx.typeck_results().type_dependent_def_id(expr.hir_id)?;
    let is_output = local_binding(cx, receiver) == Some(output);
    let is_push = is_vec_method(cx, method, "push");
    (is_output && is_push).then_some(value)
}

fn is_vec_method(cx: &LateContext<'_>, method: DefId, name: &str) -> bool {
    let Some(owner) = cx.tcx.impl_of_assoc(method) else {
        return false;
    };
    let is_inherent = !cx.tcx.impl_is_of_trait(owner);
    let is_named = cx.tcx.item_name(method).as_str() == name;
    let ty = cx.tcx.type_of(owner).instantiate_identity().skip_norm_wip();
    let is_vec = ty
        .ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_diagnostic_item(sym::Vec, definition.did()));
    is_inherent && is_named && is_vec
}

fn is_pure_value(cx: &LateContext<'_>, expr: &Expr<'_>, item: HirId) -> bool {
    let is_primitive = cx.typeck_results().expr_ty(expr).peel_refs().is_primitive();
    let should_skip = !is_primitive || crate::is_macro_expansion(expr);
    if should_skip {
        return false;
    }
    match expr.kind {
        ExprKind::Path(_) => local_binding(cx, expr) == Some(item),
        ExprKind::Lit(_) => true,
        ExprKind::Binary(_, left, right) => {
            is_pure_value(cx, left, item) && is_pure_value(cx, right, item)
        }
        ExprKind::Unary(_, inner) | ExprKind::Cast(inner, _) => is_pure_value(cx, inner, item),
        _ => false,
    }
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.tcx.emit_node_span_lint(
        NO_LOOP_APPEND_COMPREHENSION,
        expr.hir_id,
        expr.span.shrink_to_lo(),
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG076 no-loop-append-comprehension: build this vector with `map(...).collect()`",
            );
            diag.help("preserve iteration order and ownership when replacing the push loop");
        }),
    );
}
