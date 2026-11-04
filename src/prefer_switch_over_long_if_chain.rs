use rustc_errors::DiagDecorator;
use rustc_hir::def::Res;
use rustc_hir::{BinOpKind, Expr, ExprKind, HirId, Node, UnOp};
use rustc_lint::{LateContext, LateLintPass, LintContext};

rustc_session::declare_lint! {
    pub(crate) PREFER_SWITCH_OVER_LONG_IF_CHAIN,
    Warn,
    "LEG034: prefer match for repeated comparisons of one local binding"
}

rustc_session::impl_lint_pass!(PreferSwitchOverLongIfChain => [PREFER_SWITCH_OVER_LONG_IF_CHAIN]);

pub(crate) struct PreferSwitchOverLongIfChain {
    pub(crate) min: usize,
}

impl<'tcx> LateLintPass<'tcx> for PreferSwitchOverLongIfChain {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let should_skip = !matches!(expr.kind, ExprKind::If(..))
            || crate::is_macro_expansion(expr)
            || is_else_if(cx, expr);
        if should_skip {
            return;
        }
        if chain_length(cx, expr).is_some_and(|length| length >= self.min) {
            report(cx, expr);
        }
    }
}

fn chain_length<'tcx>(cx: &LateContext<'tcx>, mut expr: &'tcx Expr<'tcx>) -> Option<usize> {
    let ExprKind::If(condition, _, _) = expr.kind else {
        return None;
    };
    let subject = comparison_subject(cx, condition)?;
    let mut length = 0;
    while let ExprKind::If(condition, _, otherwise) = expr.kind {
        if comparison_subject(cx, condition) != Some(subject) {
            return None;
        }
        length += 1;
        let Some(next) = otherwise else {
            break;
        };
        expr = next;
    }
    Some(length)
}

pub(crate) fn comparison_subject<'tcx>(
    cx: &LateContext<'tcx>,
    condition: &'tcx Expr<'tcx>,
) -> Option<HirId> {
    let ExprKind::Binary(operator, left, right) = condition.peel_drop_temps().kind else {
        return None;
    };
    let should_skip = operator.node != BinOpKind::Eq || crate::is_macro_expansion(condition);
    if should_skip {
        return None;
    }
    let subject = if is_literal(right) {
        left
    } else if is_literal(left) {
        right
    } else {
        return None;
    };
    local_subject(cx, subject)
}

fn local_subject<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> Option<HirId> {
    let ExprKind::Path(ref path) = expr.kind else {
        return None;
    };
    let Res::Local(binding) = cx.qpath_res(path, expr.hir_id) else {
        return None;
    };
    let ty = cx.typeck_results().expr_ty(expr).peel_refs();
    let supports_patterns = ty.is_integral() || ty.is_char() || ty.is_bool() || ty.is_str();
    let is_source = !crate::is_macro_expansion(expr);
    (supports_patterns && is_source).then_some(binding)
}

pub(crate) fn is_literal(expr: &Expr<'_>) -> bool {
    if crate::is_macro_expansion(expr) {
        return false;
    }
    match expr.kind {
        ExprKind::Lit(_) => true,
        ExprKind::Unary(UnOp::Neg, inner) => {
            matches!(inner.kind, ExprKind::Lit(_)) && !crate::is_macro_expansion(inner)
        }
        _ => false,
    }
}

fn is_else_if(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let Node::Expr(parent) = cx.tcx.parent_hir_node(expr.hir_id) else {
        return false;
    };
    matches!(parent.kind, ExprKind::If(_, _, Some(other)) if other.hir_id == expr.hir_id)
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.emit_span_lint(
        PREFER_SWITCH_OVER_LONG_IF_CHAIN,
        expr.span.shrink_to_lo(),
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG034 prefer-switch-over-long-if-chain: use `match` for this repeated comparison chain",
            );
            diag.help("match on the same value while preserving branch order and behavior");
        }),
    );
}
