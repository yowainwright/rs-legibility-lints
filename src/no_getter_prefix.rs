use rustc_errors::DiagDecorator;
use rustc_hir::def::Res;
use rustc_hir::{Body, BodyId, Expr, ExprKind, ImplItem, ImplItemKind, PatKind, StmtKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};

rustc_session::declare_lint! {
    pub(crate) NO_GETTER_PREFIX,
    Warn,
    "LEG042: drop get_ from simple field accessors"
}

rustc_session::impl_lint_pass!(NoGetterPrefix => [NO_GETTER_PREFIX]);

pub(crate) struct NoGetterPrefix;

impl<'tcx> LateLintPass<'tcx> for NoGetterPrefix {
    fn check_impl_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx ImplItem<'tcx>) {
        let Some(body) = getter_body(item) else {
            return;
        };
        let name = item.ident.name.as_str();
        let Some(accessor) = name.strip_prefix("get_").filter(|name| !name.is_empty()) else {
            return;
        };
        if returns_own_field(cx, cx.tcx.hir_body(body)) {
            report(cx, item, accessor);
        }
    }
}

fn getter_body(item: &ImplItem<'_>) -> Option<BodyId> {
    let ImplItemKind::Fn(signature, body) = item.kind else {
        return None;
    };
    let has_receiver = signature.decl.implicit_self().has_implicit_self();
    let only_receiver = signature.decl.inputs.len() == 1;
    let is_candidate =
        !item.span.from_expansion() && item.vis_span().is_some() && only_receiver && has_receiver;
    is_candidate.then_some(body)
}

pub(crate) fn returns_own_field<'tcx>(cx: &LateContext<'tcx>, body: &Body<'tcx>) -> bool {
    let Some(value) = returned_value(body) else {
        return false;
    };
    let ExprKind::Field(base, _) = value.peel_borrows().kind else {
        return false;
    };
    let ExprKind::Path(path) = base.kind else {
        return false;
    };
    let [receiver] = body.params else {
        return false;
    };
    let PatKind::Binding(_, binding, _, _) = receiver.pat.kind else {
        return false;
    };
    matches!(cx.qpath_res(&path, base.hir_id), Res::Local(local) if local == binding)
}

fn returned_value<'tcx>(body: &Body<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Block(block, None) = body.value.kind else {
        return None;
    };
    let value = match (block.stmts, block.expr) {
        ([], Some(value)) => value,
        ([statement], None) => match statement.kind {
            StmtKind::Semi(value) => value,
            _ => return None,
        },
        _ => return None,
    };
    let value = match value.kind {
        ExprKind::Ret(Some(value)) => value,
        _ => value,
    };
    (!crate::is_macro_expansion(value)).then_some(value)
}

fn report(cx: &LateContext<'_>, item: &ImplItem<'_>, accessor: &str) {
    let name = item.ident.name;
    cx.emit_span_lint(
        NO_GETTER_PREFIX,
        item.ident.span,
        DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG042 no-getter-prefix: name this accessor `{accessor}` instead of `{name}`"
            ));
            diag.help("rename the method and update its call sites");
        }),
    );
}
