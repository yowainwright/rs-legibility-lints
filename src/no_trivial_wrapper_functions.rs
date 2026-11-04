use rustc_errors::DiagDecorator;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::intravisit::FnKind;
use rustc_hir::{
    BindingMode, Body, Constness, Expr, ExprKind, FnDecl, Node, Param, PatKind, StmtKind,
};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Span;
use rustc_span::def_id::{DefId, LocalDefId};

rustc_session::declare_lint! {
    pub(crate) NO_TRIVIAL_WRAPPER_FUNCTIONS,
    Warn,
    "LEG008: avoid private functions that only forward their parameters"
}

rustc_session::impl_lint_pass!(NoTrivialWrapperFunctions => [NO_TRIVIAL_WRAPPER_FUNCTIONS]);

pub(crate) struct NoTrivialWrapperFunctions;

impl<'tcx> LateLintPass<'tcx> for NoTrivialWrapperFunctions {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        id: LocalDefId,
    ) {
        let should_skip =
            span.from_expansion() || body.params.is_empty() || !is_candidate(cx, kind, id);
        if should_skip {
            return;
        }
        let Some(call) = single_expression(body) else {
            return;
        };
        if is_forwarding_call(cx, call, body.params, id) {
            report(cx, id, span);
        }
    }
}

fn is_candidate(cx: &LateContext<'_>, kind: FnKind<'_>, id: LocalDefId) -> bool {
    let Some(header) = kind.header() else {
        return false;
    };
    let is_const = matches!(header.constness, Constness::Const { .. });
    let is_special = header.is_async() || header.is_unsafe() || is_const;
    let is_boundary = is_special || !header.abi.is_rustic_abi();
    if is_boundary {
        return false;
    }
    if cx.tcx.codegen_fn_attrs(id).contains_extern_indicator() {
        return false;
    }
    match cx.tcx.hir_node_by_def_id(id) {
        Node::Item(item) => item.vis_span.is_empty(),
        Node::ImplItem(item) => item.vis_span().is_some_and(Span::is_empty),
        _ => false,
    }
}

fn single_expression<'tcx>(body: &Body<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Block(block, None) = body.value.kind else {
        return None;
    };
    let expr = match (block.stmts, block.expr) {
        ([], Some(expr)) => expr,
        ([stmt], None) => match stmt.kind {
            StmtKind::Semi(expr) | StmtKind::Expr(expr) => expr,
            _ => return None,
        },
        _ => return None,
    };
    match expr.kind {
        ExprKind::Ret(Some(value)) => Some(value),
        _ => Some(expr),
    }
}

fn is_forwarding_call<'tcx>(
    cx: &LateContext<'tcx>,
    call: &Expr<'tcx>,
    params: &[Param<'tcx>],
    id: LocalDefId,
) -> bool {
    let Some(callee) = called_function(cx, call) else {
        return false;
    };
    let should_skip = callee == id.to_def_id()
        || crate::is_macro_expansion(call)
        || !preserves_output(cx, call, id);
    if should_skip {
        return false;
    }
    match call.kind {
        ExprKind::Call(_, args) => matches_arguments(cx, params, args.iter()),
        ExprKind::MethodCall(_, receiver, args, _) => {
            matches_arguments(cx, params, std::iter::once(receiver).chain(args.iter()))
        }
        _ => false,
    }
}

fn preserves_output<'tcx>(cx: &LateContext<'tcx>, call: &Expr<'tcx>, id: LocalDefId) -> bool {
    let signature = cx.tcx.fn_sig(id).instantiate_identity().skip_norm_wip();
    let output = cx
        .tcx
        .instantiate_bound_regions_with_erased(signature)
        .output();
    let actual = cx.typeck_results().expr_ty(call);
    cx.tcx.erase_and_anonymize_regions(actual) == cx.tcx.erase_and_anonymize_regions(output)
}

fn called_function(cx: &LateContext<'_>, call: &Expr<'_>) -> Option<DefId> {
    match call.kind {
        ExprKind::Call(callee, _) => {
            let ExprKind::Path(path) = callee.kind else {
                return None;
            };
            match cx.qpath_res(&path, callee.hir_id) {
                Res::Def(DefKind::Fn | DefKind::AssocFn, id) => Some(id),
                _ => None,
            }
        }
        ExprKind::MethodCall(..) => cx.typeck_results().type_dependent_def_id(call.hir_id),
        _ => None,
    }
}

fn matches_arguments<'tcx>(
    cx: &LateContext<'tcx>,
    params: &[Param<'tcx>],
    mut args: impl Iterator<Item = &'tcx Expr<'tcx>>,
) -> bool {
    let matches = params
        .iter()
        .all(|param| args.next().is_some_and(|arg| is_parameter(cx, param, arg)));
    matches && args.next().is_none()
}

fn is_parameter(cx: &LateContext<'_>, param: &Param<'_>, arg: &Expr<'_>) -> bool {
    let PatKind::Binding(BindingMode::NONE | BindingMode::MUT, id, _, None) = param.pat.kind else {
        return false;
    };
    let ExprKind::Path(path) = arg.kind else {
        return false;
    };
    let is_binding =
        matches!(cx.qpath_res(&path, arg.hir_id), Res::Local(binding) if binding == id);
    let original = cx.typeck_results().expr_ty(arg);
    let adjusted = cx.typeck_results().expr_ty_adjusted(arg);
    let is_same_type = cx.tcx.erase_and_anonymize_regions(original)
        == cx.tcx.erase_and_anonymize_regions(adjusted);
    is_binding && is_same_type
}

fn report(cx: &LateContext<'_>, id: LocalDefId, span: Span) {
    cx.emit_span_lint(
        NO_TRIVIAL_WRAPPER_FUNCTIONS,
        cx.tcx.def_ident_span(id).unwrap_or(span),
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG008 no-trivial-wrapper-functions: function only forwards its parameters",
            );
            diag.help("consider calling the underlying function or method directly");
        }),
    );
}
