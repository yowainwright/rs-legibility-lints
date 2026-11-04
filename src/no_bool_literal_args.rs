use rustc_errors::DiagDecorator;
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};

use crate::no_repeated_comprehension_filter::contains_macro_call;

rustc_session::declare_lint! {
    pub(crate) NO_BOOL_LITERAL_ARGS,
    Warn,
    "LEG035: name boolean literals passed as arguments"
}

rustc_session::impl_lint_pass!(NoBoolLiteralArgs => [NO_BOOL_LITERAL_ARGS]);

pub(crate) struct NoBoolLiteralArgs;

impl<'tcx> LateLintPass<'tcx> for NoBoolLiteralArgs {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let (ExprKind::Call(_, arguments) | ExprKind::MethodCall(_, _, arguments, _)) = expr.kind
        else {
            return;
        };
        if crate::is_macro_expansion(expr) {
            return;
        }
        let mut literals = arguments
            .iter()
            .filter(|argument| is_boolean_literal(cx, argument))
            .peekable();
        let should_skip = literals.peek().is_none() || contains_macro_call(cx, expr);
        if should_skip {
            return;
        }
        for argument in literals {
            report(cx, argument);
        }
    }
}

fn is_boolean_literal<'tcx>(cx: &LateContext<'tcx>, argument: &'tcx Expr<'tcx>) -> bool {
    let ExprKind::Lit(_) = argument.kind else {
        return false;
    };
    cx.typeck_results().expr_ty(argument).is_bool() && !crate::is_macro_expansion(argument)
}

fn report(cx: &LateContext<'_>, argument: &Expr<'_>) {
    cx.emit_span_lint(
        NO_BOOL_LITERAL_ARGS,
        argument.span,
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG035 no-bool-literal-args: use a named value instead of a boolean literal argument",
            );
            diag.help("give this value a name that explains the argument's meaning");
        }),
    );
}
