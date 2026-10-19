use rustc_errors::DiagDecorator;
use rustc_hir::intravisit::{self, Visitor};
use rustc_hir::{Body, Expr, ExprKind, MatchSource, Node};
use rustc_lint::{LateContext, LateLintPass};

rustc_session::declare_lint! {
    pub(crate) MAX_CONTROL_FLOW_DEPTH,
    Warn,
    "LEG003: limit nested control-flow depth"
}

rustc_session::impl_lint_pass!(MaxControlFlowDepth => [MAX_CONTROL_FLOW_DEPTH]);

pub(crate) struct MaxControlFlowDepth {
    pub(crate) max: usize,
}

impl<'tcx> LateLintPass<'tcx> for MaxControlFlowDepth {
    fn check_body(&mut self, cx: &LateContext<'tcx>, body: &Body<'tcx>) {
        DepthVisitor {
            cx,
            depth: 0,
            max: self.max,
        }
        .visit_expr(body.value);
    }
}

struct DepthVisitor<'a, 'tcx> {
    cx: &'a LateContext<'tcx>,
    depth: usize,
    max: usize,
}

impl<'tcx> Visitor<'tcx> for DepthVisitor<'_, 'tcx> {
    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) {
        if crate::is_macro_expansion(expr) {
            return;
        }
        let weight = control_flow_weight(self.cx, expr);
        self.depth += weight;
        let exceeds_limit = weight > 0 && self.depth > self.max;
        if exceeds_limit {
            self.report(expr);
        }
        intravisit::walk_expr(self, expr);
        self.depth -= weight;
    }
}

impl DepthVisitor<'_, '_> {
    fn report(&self, expr: &Expr<'_>) {
        self.cx.tcx.emit_node_span_lint(
            MAX_CONTROL_FLOW_DEPTH,
            expr.hir_id,
            expr.span.shrink_to_lo(),
            DiagDecorator(|diag| {
                diag.primary_message(format!(
                    "LEG003 max-control-flow-depth: control-flow depth {} exceeds {}",
                    self.depth, self.max
                ));
                diag.help("return early or extract a helper to reduce nesting");
            }),
        );
    }
}

fn control_flow_weight(cx: &LateContext<'_>, expr: &Expr<'_>) -> usize {
    match expr.kind {
        ExprKind::If(..) if !expr.span.from_expansion() && !is_else_if(cx, expr) => 1,
        ExprKind::Loop(..) => usize::from(crate::is_source_loop(expr)),
        ExprKind::Match(_, _, MatchSource::Normal | MatchSource::Postfix) => 1,
        _ => 0,
    }
}

fn is_else_if(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let Node::Expr(parent) = cx.tcx.parent_hir_node(expr.hir_id) else {
        return false;
    };
    matches!(parent.kind, ExprKind::If(_, _, Some(other)) if other.hir_id == expr.hir_id)
}
