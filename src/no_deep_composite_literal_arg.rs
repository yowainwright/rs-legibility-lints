use rustc_errors::DiagDecorator;
use rustc_hir::{Expr, ExprKind, StructTailExpr};
use rustc_lint::{LateContext, LateLintPass, LintContext};

rustc_session::declare_lint! {
    pub(crate) NO_DEEP_COMPOSITE_LITERAL_ARG,
    Warn,
    "LEG037: limit literal nesting in call arguments"
}

rustc_session::impl_lint_pass!(NoDeepCompositeLiteralArg => [NO_DEEP_COMPOSITE_LITERAL_ARG]);

pub(crate) struct NoDeepCompositeLiteralArg {
    pub(crate) max: usize,
}

impl<'tcx> LateLintPass<'tcx> for NoDeepCompositeLiteralArg {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let (ExprKind::Call(_, arguments) | ExprKind::MethodCall(_, _, arguments, _)) = expr.kind
        else {
            return;
        };
        if crate::is_macro_expansion(expr) {
            return;
        }
        for argument in arguments
            .iter()
            .filter(|argument| exceeds_limit(argument, self.max))
        {
            self.report(cx, argument);
        }
    }
}

impl NoDeepCompositeLiteralArg {
    fn report(&self, cx: &LateContext<'_>, argument: &Expr<'_>) {
        let unit = if self.max == 1 { "level" } else { "levels" };
        cx.emit_span_lint(
            NO_DEEP_COMPOSITE_LITERAL_ARG,
            argument.span,
            DiagDecorator(|diag| {
                diag.primary_message(format!(
                    "LEG037 no-deep-composite-literal-arg: literal nesting exceeds {} {unit}",
                    self.max
                ));
                diag.help(
                    "extract a named value while preserving construction order and ownership",
                );
            }),
        );
    }
}

fn exceeds_limit(expr: &Expr<'_>, max: usize) -> bool {
    if crate::is_macro_expansion(expr) {
        return false;
    }
    match expr.kind {
        ExprKind::AddrOf(_, _, inner) => exceeds_limit(inner, max),
        ExprKind::Struct(..) | ExprKind::Array(_) | ExprKind::Tup(_) | ExprKind::Repeat(..) => {
            max == 0 || children_exceed_limit(expr, max - 1)
        }
        _ => false,
    }
}

fn children_exceed_limit(expr: &Expr<'_>, max: usize) -> bool {
    match expr.kind {
        ExprKind::Struct(_, fields, base) => {
            fields.iter().any(|field| exceeds_limit(field.expr, max))
                || matches!(base, StructTailExpr::Base(base) if exceeds_limit(base, max))
        }
        ExprKind::Array(values) | ExprKind::Tup(values) => {
            values.iter().any(|value| exceeds_limit(value, max))
        }
        ExprKind::Repeat(value, _) => exceeds_limit(value, max),
        _ => false,
    }
}
