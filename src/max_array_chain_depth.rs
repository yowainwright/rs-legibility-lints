use rustc_errors::DiagDecorator;
use rustc_hir::{Expr, ExprKind, Node};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::sym;

rustc_session::declare_lint! {
    pub(crate) MAX_ARRAY_CHAIN_DEPTH,
    Warn,
    "LEG011: limit consecutive Iterator method calls"
}

rustc_session::impl_lint_pass!(MaxArrayChainDepth => [MAX_ARRAY_CHAIN_DEPTH]);

pub(crate) struct MaxArrayChainDepth {
    pub(crate) max: usize,
}

impl<'tcx> LateLintPass<'tcx> for MaxArrayChainDepth {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let is_method = matches!(expr.kind, ExprKind::MethodCall(..));
        let should_skip = !is_method || is_nested_chain(cx, expr);
        if should_skip {
            return;
        }
        if chain_exceeds_limit(cx, expr, self.max) {
            self.report(cx, expr);
        }
    }
}

impl MaxArrayChainDepth {
    fn report(&self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        let ExprKind::MethodCall(method, ..) = expr.kind else {
            return;
        };
        cx.emit_span_lint(
            MAX_ARRAY_CHAIN_DEPTH,
            method.ident.span,
            DiagDecorator(|diag| {
                diag.primary_message(format!(
                    "LEG011 max-array-chain-depth: iterator chain exceeds {} method calls",
                    self.max
                ));
                diag.help(
                    "split the chain into named iterators without collecting intermediate results",
                );
            }),
        );
    }
}

fn chain_exceeds_limit<'tcx>(
    cx: &LateContext<'tcx>,
    mut expr: &'tcx Expr<'tcx>,
    max: usize,
) -> bool {
    let mut depth = 0;
    while let Some(receiver) = iterator_receiver(cx, expr) {
        depth += 1;
        if depth > max {
            return true;
        }
        expr = receiver;
    }
    false
}

pub(crate) fn iterator_receiver<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::MethodCall(_, receiver, _, _) = expr.kind else {
        return None;
    };
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let method = cx.typeck_results().type_dependent_def_id(expr.hir_id)?;
    let method = cx.tcx.trait_item_of(method).unwrap_or(method);
    let trait_id = cx.tcx.trait_of_assoc(method)?;
    cx.tcx
        .is_diagnostic_item(sym::Iterator, trait_id)
        .then_some(receiver)
}

fn is_nested_chain<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    let Node::Expr(parent) = cx.tcx.parent_hir_node(expr.hir_id) else {
        return false;
    };
    iterator_receiver(cx, parent).is_some_and(|receiver| receiver.hir_id == expr.hir_id)
}
