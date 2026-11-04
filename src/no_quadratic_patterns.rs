use rustc_errors::DiagDecorator;
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Body, Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass};

rustc_session::declare_lint! {
    pub(crate) NO_QUADRATIC_PATTERNS,
    Warn,
    "LEG005: flag nested loops and linear collection searches in loops"
}

rustc_session::impl_lint_pass!(NoQuadraticPatterns => [NO_QUADRATIC_PATTERNS]);

pub(crate) struct NoQuadraticPatterns;

impl<'tcx> LateLintPass<'tcx> for NoQuadraticPatterns {
    fn check_body(&mut self, cx: &LateContext<'tcx>, body: &Body<'tcx>) {
        LoopVisitor {
            cx,
            inside_loop: false,
        }
        .visit_expr(body.value);
    }
}

struct LoopVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    inside_loop: bool,
}

impl<'tcx> Visitor<'tcx> for LoopVisitor<'_, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if crate::is_macro_expansion(expr) {
            return;
        }
        if crate::is_source_loop(expr) {
            self.visit_loop(expr);
            return;
        }
        let is_repeated_search = self.inside_loop && is_slice_contains(self.cx, expr);
        if is_repeated_search {
            self.report(expr, "linear collection search inside a loop");
        }
        intravisit::walk_expr(self, expr);
    }
}

impl<'tcx> LoopVisitor<'_, 'tcx> {
    fn visit_loop(&mut self, expr: &'tcx Expr<'tcx>) {
        if self.inside_loop {
            self.report(expr, "nested loop detected");
            return;
        }
        self.inside_loop = true;
        intravisit::walk_expr(self, expr);
        self.inside_loop = false;
    }

    fn report(&self, expr: &Expr<'_>, message: &str) {
        self.cx.tcx.emit_node_span_lint(
            NO_QUADRATIC_PATTERNS,
            expr.hir_id,
            expr.span.shrink_to_lo(),
            DiagDecorator(|diag| {
                diag.primary_message(format!("LEG005 no-quadratic-patterns: {message}"));
                diag.help("consider an index or a single traversal while preserving behavior");
            }),
        );
    }
}

pub(crate) fn is_slice_contains(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let ExprKind::MethodCall(method, receiver, _, _) = expr.kind else {
        return false;
    };
    if method.ident.name.as_str() != "contains" {
        return false;
    }
    let receiver_type = cx.typeck_results().expr_ty_adjusted(receiver).peel_refs();
    if !receiver_type.is_slice() {
        return false;
    }
    let Some(method_id) = cx.typeck_results().type_dependent_def_id(expr.hir_id) else {
        return false;
    };
    cx.tcx
        .impl_of_assoc(method_id)
        .is_some_and(|impl_id| !cx.tcx.impl_is_of_trait(impl_id))
}
