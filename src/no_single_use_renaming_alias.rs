use rustc_errors::DiagDecorator;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{
    BindingMode, Block, Expr, ExprKind, HirId, LetStmt, Mutability, Node, PatKind, Stmt, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Ident;

use crate::no_repeated_collection_search::local_binding;

rustc_session::declare_lint! {
    pub(crate) NO_SINGLE_USE_RENAMING_ALIAS,
    Warn,
    "LEG058: avoid single-use Copy aliases before a final function call"
}

rustc_session::impl_lint_pass!(NoSingleUseRenamingAlias => [NO_SINGLE_USE_RENAMING_ALIAS]);

pub(crate) struct NoSingleUseRenamingAlias;

impl<'tcx> LateLintPass<'tcx> for NoSingleUseRenamingAlias {
    fn check_block(&mut self, cx: &LateContext<'tcx>, block: &'tcx Block<'tcx>) {
        let Some((statement, call)) = final_pair(block) else {
            return;
        };
        let Some(alias) = plain_alias(statement) else {
            return;
        };
        let Some(source) = copy_source(cx, alias) else {
            return;
        };
        if passes_alias(cx, call, alias.pat.hir_id) {
            report(cx, alias, source);
        }
    }
}

fn final_pair<'tcx>(block: &Block<'tcx>) -> Option<(&'tcx Stmt<'tcx>, &'tcx Expr<'tcx>)> {
    let (last, preceding) = block.stmts.split_last()?;
    if let Some(tail) = block.expr {
        return Some((last, tail));
    }
    let (StmtKind::Semi(call) | StmtKind::Expr(call)) = last.kind else {
        return None;
    };
    Some((preceding.last()?, call))
}

fn plain_alias<'tcx>(statement: &Stmt<'tcx>) -> Option<&'tcx LetStmt<'tcx>> {
    let StmtKind::Let(binding) = statement.kind else {
        return None;
    };
    let PatKind::Binding(BindingMode::NONE, _, name, None) = binding.pat.kind else {
        return None;
    };
    let has_annotations = binding.ty.is_some() || binding.els.is_some();
    let should_skip =
        has_annotations || binding.span.from_expansion() || name.name.as_str().starts_with('_');
    (!should_skip).then_some(binding)
}

fn copy_source(cx: &LateContext<'_>, alias: &LetStmt<'_>) -> Option<Ident> {
    let initializer = alias.init?;
    let source = immutable_source(cx, local_binding(cx, initializer)?)?;
    let PatKind::Binding(_, _, name, _) = alias.pat.kind else {
        return None;
    };
    if name.name == source.name {
        return None;
    }
    let ty = cx.typeck_results().node_type(alias.pat.hir_id);
    let is_same_type = ty == cx.typeck_results().expr_ty(initializer);
    let is_copy = cx.type_is_copy_modulo_regions(ty);
    (is_same_type && is_copy).then_some(source)
}

pub(crate) fn immutable_source(cx: &LateContext<'_>, id: HirId) -> Option<Ident> {
    let Node::Pat(pat) = cx.tcx.hir_node(id) else {
        return None;
    };
    let PatKind::Binding(BindingMode(_, Mutability::Not), _, name, _) = pat.kind else {
        return None;
    };
    Some(name)
}

fn passes_alias(cx: &LateContext<'_>, call: &Expr<'_>, alias: HirId) -> bool {
    let ExprKind::Call(callee, [argument]) = call.kind else {
        return false;
    };
    let is_generated = crate::is_macro_expansion(call) || crate::is_macro_expansion(callee);
    if is_generated {
        return false;
    }
    let ExprKind::Path(path) = callee.kind else {
        return false;
    };
    let is_function = matches!(
        cx.qpath_res(&path, callee.hir_id),
        Res::Def(DefKind::Fn | DefKind::AssocFn, _)
    );
    let is_alias = local_binding(cx, argument) == Some(alias);
    is_function && is_alias
}

fn report(cx: &LateContext<'_>, alias: &LetStmt<'_>, source: Ident) {
    let PatKind::Binding(_, _, name, _) = alias.pat.kind else {
        return;
    };
    cx.tcx.emit_node_span_lint(
        NO_SINGLE_USE_RENAMING_ALIAS,
        alias.pat.hir_id,
        name.span,
        DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG058 no-single-use-renaming-alias: `{}` only renames `{}` for one use",
                name.name, source.name
            ));
            diag.help("pass the original binding directly and remove this alias");
        }),
    );
}
