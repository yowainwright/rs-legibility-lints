use rustc_errors::DiagDecorator;
use rustc_hir::{Pat, PatKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Ident;

use crate::prefer_positive_condition_names::PREDICATES;

rustc_session::declare_lint! {
    pub(crate) PREFER_BOOLEAN_PREFIXES,
    Allow,
    "LEG053: prefer predicate or state prefixes for boolean bindings"
}

rustc_session::impl_lint_pass!(PreferBooleanPrefixes => [PREFER_BOOLEAN_PREFIXES]);

const EXCEPTIONS: &[&str] = &[
    "debug", "disabled", "done", "empty", "enabled", "exists", "found", "invalid", "ok", "ready",
    "valid",
];

pub(crate) struct PreferBooleanPrefixes;

impl<'tcx> LateLintPass<'tcx> for PreferBooleanPrefixes {
    fn check_pat(&mut self, cx: &LateContext<'tcx>, pat: &'tcx Pat<'tcx>) {
        let level = cx.get_lint_level_spec(PREFER_BOOLEAN_PREFIXES);
        if level.is_allow() {
            return;
        }
        let PatKind::Binding(_, id, name, _) = pat.kind else {
            return;
        };
        let should_skip =
            pat.span.from_expansion() || pat.hir_id != id || is_allowed_name(name.name.as_str());
        if should_skip {
            return;
        }
        let is_boolean = cx.typeck_results().node_type(id).peel_refs().is_bool();
        if is_boolean {
            report(cx, name);
        }
    }
}

fn is_allowed_name(name: &str) -> bool {
    let is_exempt = name.starts_with('_') || name == "self";
    if is_exempt {
        return true;
    }
    let Some((predicate, subject)) = name.split_once('_') else {
        return EXCEPTIONS.contains(&name);
    };
    PREDICATES.contains(&predicate) && !subject.is_empty()
}

fn report(cx: &LateContext<'_>, name: Ident) {
    let name_text = name.name;
    cx.emit_span_lint(
        PREFER_BOOLEAN_PREFIXES,
        name.span,
        DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG053 prefer-boolean-prefixes: give boolean binding `{name_text}` a predicate or state prefix"
            ));
            diag.help("use a name such as `is_active`, `has_owner`, or `can_send`; update uses together");
        }),
    );
}
