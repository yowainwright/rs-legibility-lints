use std::ops::ControlFlow;

use rustc_errors::DiagDecorator;
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};

rustc_session::declare_lint! {
    pub(crate) HOIST_IF_OPERATORS,
    Warn,
    "LEG002: hoist operator-heavy if conditions into named booleans"
}

rustc_session::impl_lint_pass!(HoistIfOperators => [HOIST_IF_OPERATORS]);

pub(crate) struct HoistIfOperators {
    pub(crate) max: usize,
}

impl<'tcx> LateLintPass<'tcx> for HoistIfOperators {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let ExprKind::If(condition, _, _) = expr.kind else {
            return;
        };
        let should_skip = expr.span.from_expansion() || has_let_binding(condition);
        if should_skip {
            return;
        }
        let mut operators = BooleanOperators {
            remaining: self.max,
        };
        let exceeds_limit = operators.visit_expr(condition).is_break();
        if exceeds_limit {
            self.report(cx, condition);
        }
    }
}

impl HoistIfOperators {
    fn report(&self, cx: &LateContext<'_>, condition: &Expr<'_>) {
        cx.emit_span_lint(
            HOIST_IF_OPERATORS,
            condition.span,
            DiagDecorator(|diag| {
                diag.primary_message(format!(
                    "LEG002 hoist-if-operators: if condition exceeds {} boolean operators",
                    self.max
                ));
                diag.help(
                    "hoist the condition into a named boolean without changing evaluation order",
                );
            }),
        );
    }
}

struct BooleanOperators {
    remaining: usize,
}

impl<'tcx> Visitor<'tcx> for BooleanOperators {
    type Result = ControlFlow<()>;

    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) -> Self::Result {
        let should_skip = expr.span.from_expansion()
            || matches!(expr.kind, ExprKind::Block(..) | ExprKind::Closure(..));
        if should_skip {
            return ControlFlow::Continue(());
        }
        let is_boolean_operator =
            matches!(expr.kind, ExprKind::Binary(op, ..) if op.node.is_lazy());
        let exceeds_limit = is_boolean_operator && self.remaining == 0;
        if exceeds_limit {
            return ControlFlow::Break(());
        }
        self.remaining -= usize::from(is_boolean_operator);
        intravisit::walk_expr(self, expr)
    }
}

fn has_let_binding(expr: &Expr<'_>) -> bool {
    match expr.peel_drop_temps().kind {
        ExprKind::Let(..) => true,
        ExprKind::Binary(_, left, right) => has_let_binding(left) || has_let_binding(right),
        _ => false,
    }
}
