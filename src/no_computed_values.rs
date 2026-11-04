use rustc_errors::DiagDecorator;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{BinOpKind, Block, Body, Expr, ExprKind, FnDecl};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::Span;
use rustc_span::def_id::LocalDefId;

rustc_session::declare_lint! {
    pub(crate) NO_COMPUTED_VALUES,
    Warn,
    "LEG012: limit returned calculations to one arithmetic operator"
}

rustc_session::impl_lint_pass!(NoComputedValues => [NO_COMPUTED_VALUES]);

pub(crate) struct NoComputedValues;

impl<'tcx> LateLintPass<'tcx> for NoComputedValues {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        _: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        _: Span,
        _: LocalDefId,
    ) {
        check_tail(cx, body.value);
    }

    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if crate::is_macro_expansion(expr) {
            return;
        }
        if let ExprKind::Ret(Some(value)) = expr.kind {
            check_tail(cx, value);
        }
    }
}

fn check_tail(cx: &LateContext<'_>, expr: &Expr<'_>) {
    if crate::is_macro_expansion(expr) {
        return;
    }
    match expr.kind {
        ExprKind::Block(block, _) => check_block(cx, block),
        ExprKind::If(_, then, otherwise) => check_branches(cx, then, otherwise),
        ExprKind::Match(_, arms, _) => {
            for arm in arms {
                check_tail(cx, arm.body);
            }
        }
        ExprKind::DropTemps(inner) => check_tail(cx, inner),
        _ => check_value(cx, expr),
    }
}

fn check_block(cx: &LateContext<'_>, block: &Block<'_>) {
    if let Some(tail) = block.expr {
        check_tail(cx, tail);
    }
}

fn check_branches(cx: &LateContext<'_>, then: &Expr<'_>, otherwise: Option<&Expr<'_>>) {
    check_tail(cx, then);
    if let Some(otherwise) = otherwise {
        check_tail(cx, otherwise);
    }
}

fn check_value(cx: &LateContext<'_>, expr: &Expr<'_>) {
    let Some((left, right)) = arithmetic_children(expr) else {
        return;
    };
    let exceeds_limit = arithmetic_children(left).is_some() || arithmetic_children(right).is_some();
    if exceeds_limit {
        report(cx, expr);
    }
}

fn arithmetic_children<'hir>(
    expr: &'hir Expr<'hir>,
) -> Option<(&'hir Expr<'hir>, &'hir Expr<'hir>)> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    match expr.kind {
        ExprKind::Binary(op, left, right) if is_arithmetic(op.node) => Some((left, right)),
        ExprKind::Unary(_, inner)
        | ExprKind::AddrOf(_, _, inner)
        | ExprKind::Cast(inner, _)
        | ExprKind::Type(inner, _)
        | ExprKind::DropTemps(inner) => arithmetic_children(inner),
        _ => None,
    }
}

fn is_arithmetic(operator: BinOpKind) -> bool {
    matches!(
        operator,
        BinOpKind::Add | BinOpKind::Sub | BinOpKind::Mul | BinOpKind::Div | BinOpKind::Rem
    )
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.tcx.emit_node_span_lint(
        NO_COMPUTED_VALUES,
        expr.hir_id,
        expr.span,
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG012 no-computed-values: returned calculation exceeds one arithmetic operator",
            );
            diag.help("extract a named intermediate value while preserving evaluation order and ownership");
        }),
    );
}
