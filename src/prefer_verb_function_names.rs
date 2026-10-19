use rustc_errors::DiagDecorator;
use rustc_hir::intravisit::FnKind;
use rustc_hir::{Body, FnDecl, Node};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::def_id::LocalDefId;
use rustc_span::{Ident, Span};

rustc_session::declare_lint! {
    pub(crate) PREFER_VERB_FUNCTION_NAMES,
    Allow,
    "LEG052: prefer action-first function names"
}

rustc_session::impl_lint_pass!(PreferVerbFunctionNames => [PREFER_VERB_FUNCTION_NAMES]);

pub(crate) struct PreferVerbFunctionNames;

impl<'tcx> LateLintPass<'tcx> for PreferVerbFunctionNames {
    fn check_fn(
        &mut self,
        cx: &LateContext<'tcx>,
        kind: FnKind<'tcx>,
        _: &'tcx FnDecl<'tcx>,
        body: &'tcx Body<'tcx>,
        span: Span,
        id: LocalDefId,
    ) {
        let level = cx.get_lint_level_spec(PREFER_VERB_FUNCTION_NAMES);
        if level.is_allow() {
            return;
        }
        let (FnKind::ItemFn(name, ..) | FnKind::Method(name, ..)) = kind else {
            return;
        };
        let should_skip = span.from_expansion()
            || !starts_with_generic_noun(name.name.as_str())
            || has_required_name(cx, kind, id);
        if should_skip {
            return;
        }
        let is_accessor = matches!(kind, FnKind::Method(..))
            && crate::no_getter_prefix::returns_own_field(cx, body);
        if !is_accessor {
            report(cx, name);
        }
    }
}

fn starts_with_generic_noun(name: &str) -> bool {
    let Some((first, suffix)) = name.split_once('_') else {
        return false;
    };
    let is_generic = matches!(
        first,
        "config"
            | "data"
            | "info"
            | "item"
            | "object"
            | "result"
            | "settings"
            | "stuff"
            | "thing"
            | "value"
    );
    is_generic && !suffix.is_empty()
}

fn has_required_name(cx: &LateContext<'_>, kind: FnKind<'_>, id: LocalDefId) -> bool {
    let Some(header) = kind.header() else {
        return true;
    };
    let is_foreign = !header.abi.is_rustic_abi();
    let is_exported = cx.tcx.codegen_fn_attrs(id).contains_extern_indicator();
    let is_trait = match cx.tcx.hir_node_by_def_id(id) {
        Node::TraitItem(_) => true,
        Node::ImplItem(item) => item.vis_span().is_none(),
        _ => false,
    };
    is_foreign || is_exported || is_trait
}

fn report(cx: &LateContext<'_>, name: Ident) {
    cx.emit_span_lint(
        PREFER_VERB_FUNCTION_NAMES,
        name.span,
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG052 prefer-verb-function-names: start this function name with an action verb",
            );
            diag.help("name the action, such as `load_data`; update call sites together");
        }),
    );
}
