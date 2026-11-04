use rustc_errors::DiagDecorator;
use rustc_hir::{BinOpKind, Expr, ExprKind, HirId, Node};
use rustc_lint::{LateContext, LateLintPass, LintContext};

use crate::prefer_switch_over_long_if_chain::comparison_subject;

rustc_session::declare_lint! {
    pub(crate) PREFER_OBJECT_LOOKUP,
    Warn,
    "LEG024: prefer matches! for long literal equality-or chains"
}

rustc_session::impl_lint_pass!(PreferObjectLookup => [PREFER_OBJECT_LOOKUP]);

pub(crate) struct PreferObjectLookup;

impl<'tcx> LateLintPass<'tcx> for PreferObjectLookup {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let should_skip = !is_or(expr) || crate::is_macro_expansion(expr) || is_nested_or(cx, expr);
        if should_skip {
            return;
        }
        if chain(cx, expr).is_some_and(|(_, length)| length >= 3) {
            report(cx, expr);
        }
    }
}

fn chain<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> Option<(HirId, usize)> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    if !is_or(expr) {
        return comparison_subject(cx, expr).map(|subject| (subject, 1));
    }
    let ExprKind::Binary(_, left, right) = expr.kind else {
        return None;
    };
    let (subject, left_count) = chain(cx, left)?;
    let (other, right_count) = chain(cx, right)?;
    let length = left_count + right_count;
    (subject == other).then_some((subject, length))
}

fn is_or(expr: &Expr<'_>) -> bool {
    matches!(expr.kind, ExprKind::Binary(op, _, _) if op.node == BinOpKind::Or)
}

fn is_nested_or(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    matches!(cx.tcx.parent_hir_node(expr.hir_id), Node::Expr(parent) if is_or(parent))
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.emit_span_lint(
        PREFER_OBJECT_LOOKUP,
        expr.span.shrink_to_lo(),
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG024 prefer-object-lookup: use `matches!` for this repeated equality chain",
            );
            diag.help("match the same local value against the literal alternatives");
        }),
    );
}
