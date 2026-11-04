use rustc_errors::DiagDecorator;
use rustc_hir::{Pat, PatKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Ident;

rustc_session::declare_lint! {
    pub(crate) PREFER_POSITIVE_CONDITION_NAMES,
    Warn,
    "LEG007: prefer positive boolean binding names"
}

rustc_session::impl_lint_pass!(PreferPositiveConditionNames => [PREFER_POSITIVE_CONDITION_NAMES]);

pub(crate) const PREDICATES: &[&str] = &[
    "is", "are", "was", "were", "has", "have", "had", "can", "could", "should", "will", "would",
    "did", "does",
];

pub(crate) struct PreferPositiveConditionNames;

impl<'tcx> LateLintPass<'tcx> for PreferPositiveConditionNames {
    fn check_pat(&mut self, cx: &LateContext<'tcx>, pat: &'tcx Pat<'tcx>) {
        let PatKind::Binding(_, id, ident, _) = pat.kind else {
            return;
        };
        let name = ident.name.as_str();
        let should_skip = pat.span.from_expansion() || pat.hir_id != id || !is_negative_name(name);
        if should_skip {
            return;
        }
        let is_boolean = cx.typeck_results().node_type(id).peel_refs().is_bool();
        if is_boolean {
            self.report(cx, ident);
        }
    }
}

impl PreferPositiveConditionNames {
    fn report(&self, cx: &LateContext<'_>, ident: Ident) {
        let name = ident.name;
        let message =
            format!("LEG007 prefer-positive-condition-names: negative boolean name `{name}`");
        cx.emit_span_lint(
            PREFER_POSITIVE_CONDITION_NAMES,
            ident.span,
            DiagDecorator(|diag| {
                diag.primary_message(message);
                diag.help(
                    "use a positive boolean name and negate at use sites, preserving behavior",
                );
            }),
        );
    }
}

fn is_negative_name(name: &str) -> bool {
    let Some((predicate, rest)) = name.split_once('_') else {
        return false;
    };
    if !PREDICATES.contains(&predicate) {
        return false;
    }
    rest.strip_prefix("not_")
        .or_else(|| rest.strip_prefix("no_"))
        .is_some_and(|subject| !subject.is_empty())
}
