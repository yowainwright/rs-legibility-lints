use rustc_errors::DiagDecorator;
use rustc_hir::{Expr, ExprKind, Node};
use rustc_lint::{LateContext, LateLintPass, LintContext};

rustc_session::declare_lint! {
    pub(crate) NO_DEEP_SELECTOR_CHAIN,
    Warn,
    "LEG031: limit consecutive field and index accesses"
}

rustc_session::impl_lint_pass!(NoDeepSelectorChain => [NO_DEEP_SELECTOR_CHAIN]);

pub(crate) struct NoDeepSelectorChain {
    pub(crate) max: usize,
}

impl<'tcx> LateLintPass<'tcx> for NoDeepSelectorChain {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let should_skip = access_base(expr).is_none() || is_nested_access(cx, expr);
        if should_skip {
            return;
        }
        if chain_exceeds_limit(expr, self.max) {
            self.report(cx, expr);
        }
    }
}

impl NoDeepSelectorChain {
    fn report(&self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        cx.emit_span_lint(
            NO_DEEP_SELECTOR_CHAIN,
            expr.span,
            DiagDecorator(|diag| {
                diag.primary_message(format!(
                    "LEG031 no-deep-selector-chain: selector/index chain exceeds {} accesses",
                    self.max
                ));
                diag.help(
                    "extract a named reference while preserving evaluation order and borrowing",
                );
            }),
        );
    }
}

fn chain_exceeds_limit<'tcx>(mut expr: &'tcx Expr<'tcx>, max: usize) -> bool {
    let mut depth = 0;
    while let Some(base) = access_base(expr) {
        depth += 1;
        if depth > max {
            return true;
        }
        expr = base;
    }
    false
}

fn access_base<'tcx>(expr: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let (ExprKind::Field(base, _) | ExprKind::Index(base, _, _)) = expr.kind else {
        return None;
    };
    let is_source = !crate::is_macro_expansion(expr);
    is_source.then_some(base)
}

fn is_nested_access<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    let Node::Expr(parent) = cx.tcx.parent_hir_node(expr.hir_id) else {
        return false;
    };
    access_base(parent).is_some_and(|base| base.hir_id == expr.hir_id)
}
