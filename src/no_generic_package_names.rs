use rustc_errors::DiagDecorator;
use rustc_hir::{Item, ItemKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Span;
use rustc_span::def_id::LOCAL_CRATE;

rustc_session::declare_lint! {
    pub(crate) NO_GENERIC_PACKAGE_NAMES,
    Warn,
    "LEG046: avoid catch-all crate and module names"
}

rustc_session::impl_lint_pass!(NoGenericPackageNames => [NO_GENERIC_PACKAGE_NAMES]);

pub(crate) struct NoGenericPackageNames;

impl<'tcx> LateLintPass<'tcx> for NoGenericPackageNames {
    fn check_crate(&mut self, cx: &LateContext<'tcx>) {
        let name = cx.tcx.crate_name(LOCAL_CRATE);
        if is_generic(name.as_str()) {
            let span = cx.tcx.hir_root_module().spans.inner_span.shrink_to_lo();
            report(cx, span, name.as_str());
        }
    }

    fn check_item(&mut self, cx: &LateContext<'tcx>, item: &'tcx Item<'tcx>) {
        let ItemKind::Mod(name, _) = item.kind else {
            return;
        };
        let should_report = !item.span.from_expansion() && is_generic(name.name.as_str());
        if should_report {
            report(cx, name.span, name.name.as_str());
        }
    }
}

fn is_generic(name: &str) -> bool {
    matches!(
        name,
        "base"
            | "common"
            | "helper"
            | "helpers"
            | "lib"
            | "libs"
            | "misc"
            | "shared"
            | "util"
            | "utils"
    )
}

fn report(cx: &LateContext<'_>, span: Span, name: &str) {
    cx.emit_span_lint(
        NO_GENERIC_PACKAGE_NAMES,
        span,
        DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG046 no-generic-package-names: `{name}` is a catch-all name"
            ));
            diag.help("name the crate or module for what it provides");
        }),
    );
}
