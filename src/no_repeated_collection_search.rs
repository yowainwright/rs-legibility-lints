use rustc_errors::DiagDecorator;
use rustc_hir::def::Res;
use rustc_hir::{Block, Expr, ExprKind, HirId, Stmt, StmtKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::sym;

rustc_session::declare_lint! {
    pub(crate) NO_REPEATED_COLLECTION_SEARCH,
    Warn,
    "LEG057: reuse adjacent searches over unchanged collections"
}

rustc_session::impl_lint_pass!(NoRepeatedCollectionSearch => [NO_REPEATED_COLLECTION_SEARCH]);

pub(crate) struct NoRepeatedCollectionSearch;

impl<'tcx> LateLintPass<'tcx> for NoRepeatedCollectionSearch {
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
    let Some(key) = search_key(cx, first) else {
        return;
    };
    if search_key(cx, second) == Some(key) {
        report(cx, second);
    }
}

pub(crate) fn search_initializer<'tcx>(statement: &Stmt<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let StmtKind::Let(binding) = statement.kind else {
        return None;
    };
    if binding.els.is_some() {
        return None;
    }
    binding.init
}

fn search_key<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> Option<(HirId, HirId)> {
    let ExprKind::MethodCall(_, receiver, [needle], _) = expr.kind else {
        return None;
    };
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let is_supported = crate::no_quadratic_patterns::is_slice_contains(cx, expr)
        && is_native_collection(cx, receiver)
        && has_builtin_equality(cx, needle);
    if !is_supported {
        return None;
    }
    let collection = local_binding(cx, receiver)?;
    let needle = local_binding(cx, needle.peel_borrows())?;
    Some((collection, needle))
}

fn is_native_collection<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    let ty = cx.typeck_results().expr_ty(expr).peel_refs();
    let is_vec = ty
        .ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_diagnostic_item(sym::Vec, definition.did()));
    ty.is_slice() || ty.is_array() || is_vec
}

fn has_builtin_equality<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    let ty = cx.typeck_results().expr_ty(expr).peel_refs();
    ty.is_integral() || ty.is_floating_point() || ty.is_bool() || ty.is_char()
}

pub(crate) fn local_binding(cx: &LateContext<'_>, expr: &Expr<'_>) -> Option<HirId> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let ExprKind::Path(path) = expr.kind else {
        return None;
    };
    let Res::Local(binding) = cx.qpath_res(&path, expr.hir_id) else {
        return None;
    };
    Some(binding)
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    let ExprKind::MethodCall(method, ..) = expr.kind else {
        return;
    };
    cx.tcx.emit_node_span_lint(
        NO_REPEATED_COLLECTION_SEARCH,
        expr.hir_id,
        method.ident.span,
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG057 no-repeated-collection-search: this search repeats the preceding search",
            );
            diag.help("reuse the previous result without changing evaluation order or borrowing");
        }),
    );
}
