use rustc_errors::DiagDecorator;
use rustc_hir::{BinOpKind, BindingMode, Block, Expr, ExprKind, HirId, PatKind, Stmt, UnOp};
use rustc_lexer::{FrontmatterAllowed, TokenKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::sym;

use crate::max_array_chain_depth::iterator_receiver;
use crate::no_repeated_collection_search::{local_binding, search_initializer};
use crate::no_single_use_renaming_alias::immutable_source;
use crate::prefer_switch_over_long_if_chain::is_literal;

rustc_session::declare_lint! {
    pub(crate) NO_REPEATED_COMPREHENSION_FILTER,
    Allow,
    "LEG077: avoid adjacent identical filters over immutable primitive collections"
}

rustc_session::impl_lint_pass!(NoRepeatedComprehensionFilter => [NO_REPEATED_COMPREHENSION_FILTER]);

pub(crate) struct NoRepeatedComprehensionFilter;

struct Predicate<'hir> {
    operator: BinOpKind,
    literal: &'hir Expr<'hir>,
    item_on_left: bool,
}

impl<'tcx> LateLintPass<'tcx> for NoRepeatedComprehensionFilter {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        for pair in block.stmts.windows(2) {
            check_pair(cx, &pair[0], &pair[1]);
        }
    }
}

fn check_pair<'tcx>(cx: &LateContext<'tcx>, first: &'tcx Stmt<'tcx>, second: &'tcx Stmt<'tcx>) {
    let Some(first) = search_initializer(first) else {
        return;
    };
    let Some(second) = search_initializer(second) else {
        return;
    };
    if same_filters(cx, first, second) {
        report(cx, second);
    }
}

fn same_filters<'tcx>(
    cx: &LateContext<'tcx>,
    first: &'tcx Expr<'tcx>,
    second: &'tcx Expr<'tcx>,
) -> bool {
    let Some((collection, first_predicate)) = filter_key(cx, first) else {
        return false;
    };
    let Some((other, second_predicate)) = filter_key(cx, second) else {
        return false;
    };
    let same_collection = collection == other;
    let same_type = cx.typeck_results().expr_ty(first) == cx.typeck_results().expr_ty(second);
    let same_operator = first_predicate.operator == second_predicate.operator;
    let same_side = first_predicate.item_on_left == second_predicate.item_on_left;
    let same_comparison = same_operator && same_side;
    let same_literal = same_literal(first_predicate.literal, second_predicate.literal);
    same_collection && same_type && same_comparison && same_literal
}

fn filter_key<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Option<(HirId, Predicate<'tcx>)> {
    if !is_vec(cx, expr) {
        return None;
    }
    let filtered = iterator_call(cx, expr, "collect")?;
    let ExprKind::MethodCall(_, _, [closure], _) = filtered.kind else {
        return None;
    };
    let iterated = iterator_call(cx, filtered, "filter")?;
    let collection = slice_iter_receiver(cx, iterated)?;
    let source = immutable_collection(cx, collection)?;
    if contains_macro_call(cx, expr) {
        return None;
    }
    Some((source, predicate(cx, closure)?))
}

pub(crate) fn contains_macro_call(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let Ok(source) = cx.tcx.sess.source_map().span_to_snippet(expr.span) else {
        return true;
    };
    let mut after_bang = false;
    for token in rustc_lexer::tokenize(&source, FrontmatterAllowed::No) {
        match token.kind {
            TokenKind::Whitespace
            | TokenKind::LineComment { .. }
            | TokenKind::BlockComment { .. } => continue,
            TokenKind::OpenParen | TokenKind::OpenBrace | TokenKind::OpenBracket if after_bang => {
                return true;
            }
            kind => after_bang = kind == TokenKind::Bang,
        }
    }
    false
}

fn iterator_call<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
    name: &str,
) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::MethodCall(method, _, _, _) = expr.kind else {
        return None;
    };
    if method.ident.name.as_str() != name {
        return None;
    }
    iterator_receiver(cx, expr)
}

fn is_vec(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    cx.typeck_results()
        .expr_ty(expr)
        .ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_diagnostic_item(sym::Vec, definition.did()))
}

fn slice_iter_receiver<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::MethodCall(method, receiver, [], _) = expr.kind else {
        return None;
    };
    let should_skip = method.ident.name.as_str() != "iter" || crate::is_macro_expansion(expr);
    if should_skip {
        return None;
    }
    let method = cx.typeck_results().type_dependent_def_id(expr.hir_id)?;
    let owner = cx.tcx.impl_of_assoc(method)?;
    let is_inherent = !cx.tcx.impl_is_of_trait(owner);
    let is_slice = cx
        .tcx
        .type_of(owner)
        .instantiate_identity()
        .skip_norm_wip()
        .is_slice();
    (is_inherent && is_slice).then_some(receiver)
}

fn immutable_collection(cx: &LateContext<'_>, expr: &Expr<'_>) -> Option<HirId> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let binding = local_binding(cx, expr.peel_borrows())?;
    immutable_source(cx, binding)?;
    let ty = cx.typeck_results().expr_ty(expr);
    if ty.is_mutable_ptr() {
        return None;
    }
    let ty = ty.peel_refs();
    let is_vec = ty
        .ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_diagnostic_item(sym::Vec, definition.did()));
    let is_collection = ty.is_array() || ty.is_slice() || is_vec;
    is_collection.then_some(binding)
}

fn predicate<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> Option<Predicate<'tcx>> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let ExprKind::Closure(closure) = expr.kind else {
        return None;
    };
    let body = cx.tcx.hir_body(closure.body);
    let [parameter] = body.params else {
        return None;
    };
    let PatKind::Binding(BindingMode::NONE, id, _, None) = parameter.pat.kind else {
        return None;
    };
    literal_comparison(cx, body.value.peel_drop_temps(), id)
}

fn literal_comparison<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
    parameter: HirId,
) -> Option<Predicate<'tcx>> {
    let ExprKind::Binary(operator, left, right) = expr.kind else {
        return None;
    };
    let should_skip = !operator.node.is_comparison() || crate::is_macro_expansion(expr);
    if should_skip {
        return None;
    }
    let (literal, item_on_left) = literal_side(cx, left, right, parameter)?;
    let operator = operator.node;
    Some(Predicate {
        operator,
        literal,
        item_on_left,
    })
}

fn literal_side<'tcx>(
    cx: &LateContext<'tcx>,
    left: &'tcx Expr<'tcx>,
    right: &'tcx Expr<'tcx>,
    parameter: HirId,
) -> Option<(&'tcx Expr<'tcx>, bool)> {
    let item_on_left = is_parameter(cx, left, parameter) && is_literal(right);
    if item_on_left {
        return Some((right, item_on_left));
    }
    let item_on_right = is_parameter(cx, right, parameter) && is_literal(left);
    item_on_right.then_some((left, item_on_left))
}

fn is_parameter(cx: &LateContext<'_>, expr: &Expr<'_>, parameter: HirId) -> bool {
    if crate::is_macro_expansion(expr) {
        return false;
    }
    match expr.kind {
        ExprKind::Path(_) => {
            let ty = cx.typeck_results().expr_ty(expr).peel_refs();
            let is_primitive = ty.is_primitive() || ty.is_str();
            is_primitive && local_binding(cx, expr) == Some(parameter)
        }
        ExprKind::Unary(UnOp::Deref, inner) => is_parameter(cx, inner, parameter),
        _ => false,
    }
}

fn same_literal(first: &Expr<'_>, second: &Expr<'_>) -> bool {
    match (first.kind, second.kind) {
        (ExprKind::Lit(first), ExprKind::Lit(second)) => first.node == second.node,
        (ExprKind::Unary(UnOp::Neg, first), ExprKind::Unary(UnOp::Neg, second)) => {
            same_literal(first, second)
        }
        _ => false,
    }
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.tcx.emit_node_span_lint(
        NO_REPEATED_COMPREHENSION_FILTER,
        expr.hir_id,
        expr.span.shrink_to_lo(),
        DiagDecorator(|diag| {
            diag.primary_message("LEG077 no-repeated-comprehension-filter: this filter repeats the preceding collection and predicate");
            diag.help("consider reusing the result while preserving separate ownership where needed");
        }),
    );
}
