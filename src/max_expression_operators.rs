use rustc_errors::DiagDecorator;
use rustc_hir::{BinOpKind, Expr, ExprKind, Node, UnOp};
use rustc_lint::{LateContext, LateLintPass, LintContext};

rustc_session::declare_lint! {
    pub(crate) MAX_EXPRESSION_OPERATORS,
    Warn,
    "LEG001: limit logical and comparison operators in an expression"
}

rustc_session::impl_lint_pass!(MaxExpressionOperators => [MAX_EXPRESSION_OPERATORS]);

pub(crate) struct MaxExpressionOperators {
    pub(crate) max: usize,
}

impl<'tcx> LateLintPass<'tcx> for MaxExpressionOperators {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if expr.span.from_expansion() {
            return;
        }
        let parent = cx.tcx.parent_hir_node(expr.hir_id);
        let is_nested = matches!(parent, Node::Expr(parent) if is_operator_expression(parent));
        if is_nested {
            return;
        }
        let exceeds_limit = count_operators(cx, expr, self.max) > self.max;
        if exceeds_limit {
            self.report(cx, expr);
        }
    }
}

impl MaxExpressionOperators {
    fn report(&self, cx: &LateContext<'_>, expr: &Expr<'_>) {
        cx.emit_span_lint(MAX_EXPRESSION_OPERATORS, expr.span, DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG001 max-expression-operators: expression exceeds {} logical/comparison operators",
                self.max
            ));
            diag.help("extract named values while preserving evaluation order");
        }));
    }
}

fn is_operator_expression(expr: &Expr<'_>) -> bool {
    matches!(
        expr.kind,
        ExprKind::Binary(..) | ExprKind::Unary(..) | ExprKind::DropTemps(..)
    )
}

fn count_operators<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>, max: usize) -> usize {
    if expr.span.from_expansion() {
        return 0;
    }
    match expr.kind {
        ExprKind::Binary(op, left, right) => count_binary(cx, op.node, left, right, max),
        ExprKind::Unary(op, inner) => count_unary(cx, expr, op, inner, max),
        ExprKind::Call(callee, arguments) => count_call(cx, callee, arguments, max),
        ExprKind::MethodCall(_, receiver, arguments, _) => count_call(cx, receiver, arguments, max),
        ExprKind::DropTemps(inner) => count_operators(cx, inner, max),
        _ => 0,
    }
}

fn count_binary<'tcx>(
    cx: &LateContext<'tcx>,
    op: BinOpKind,
    left: &'tcx Expr<'tcx>,
    right: &'tcx Expr<'tcx>,
    max: usize,
) -> usize {
    let weight = usize::from(op.is_lazy() || op.is_comparison());
    if weight > max {
        return weight;
    }
    let count = weight.saturating_add(count_operators(cx, left, max - weight));
    if count > max {
        return count;
    }
    count.saturating_add(count_operators(cx, right, max - count))
}

fn count_unary<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
    op: UnOp,
    inner: &'tcx Expr<'tcx>,
    max: usize,
) -> usize {
    let is_logical_not = op == UnOp::Not && cx.typeck_results().expr_ty(expr).is_bool();
    let weight = usize::from(is_logical_not);
    if weight > max {
        return weight;
    }
    weight.saturating_add(count_operators(cx, inner, max - weight))
}

fn count_call<'tcx>(
    cx: &LateContext<'tcx>,
    target: &'tcx Expr<'tcx>,
    arguments: &'tcx [Expr<'tcx>],
    max: usize,
) -> usize {
    let mut count: usize = 0;
    for expression in std::iter::once(target).chain(arguments) {
        let remaining = max - count;
        count = count.saturating_add(count_operators(cx, expression, remaining));
        if count > max {
            return count;
        }
    }
    count
}
