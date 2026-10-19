use std::collections::HashSet;

use rustc_errors::DiagDecorator;
use rustc_hir::{HirId, Mod};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::{BytePos, FileName, Span};

rustc_session::declare_lint! {
    pub(crate) NO_MIXED_FILENAME_CASING,
    Warn,
    "LEG026: prefer lowercase Rust filenames with underscores"
}

rustc_session::impl_lint_pass!(NoMixedFilenameCasing => [NO_MIXED_FILENAME_CASING]);

#[derive(Default)]
pub(crate) struct NoMixedFilenameCasing {
    reported: HashSet<BytePos>,
}

impl<'tcx> LateLintPass<'tcx> for NoMixedFilenameCasing {
    fn check_mod(&mut self, cx: &LateContext<'tcx>, module: &'tcx Mod<'tcx>, _: HirId) {
        let span = module.spans.inner_span;
        let should_skip =
            span.from_expansion() || cx.get_lint_level_spec(NO_MIXED_FILENAME_CASING).is_allow();
        if should_skip {
            return;
        }
        let file = cx.sess().source_map().lookup_source_file(span.lo());
        let Some(stem) = filename_stem(&file.name) else {
            return;
        };
        let should_report = has_mixed_casing(stem) && self.reported.insert(file.start_pos);
        if should_report {
            let start = span.with_lo(file.start_pos).shrink_to_lo();
            report(cx, start, stem);
        }
    }
}

fn filename_stem(filename: &FileName) -> Option<&str> {
    let FileName::Real(real) = filename else {
        return None;
    };
    real.local_path()?
        .file_name()?
        .to_str()?
        .strip_suffix(".rs")
}

fn has_mixed_casing(stem: &str) -> bool {
    stem.chars()
        .any(|character| character.is_uppercase() || character == '-')
}

fn report(cx: &LateContext<'_>, span: Span, stem: &str) {
    cx.emit_span_lint(
        NO_MIXED_FILENAME_CASING,
        span,
        DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG026 no-mixed-filename-casing: `{stem}.rs` is not snake_case"
            ));
            diag.help(
                "rename the file using lowercase letters and underscores; update module paths",
            );
        }),
    );
}

#[test]
fn rejects_uppercase_and_hyphens() {
    for stem in [
        "API_client",
        "ApiClient",
        "API",
        "api-client",
        "api_Client",
        "Équipe",
    ] {
        assert!(has_mixed_casing(stem), "{stem}");
    }
}

#[test]
fn accepts_lowercase_and_uncased_names() {
    for stem in [
        "api_client",
        "lib",
        "main",
        "mod",
        "api2_client",
        "_private",
        "équipe",
        "客户",
    ] {
        assert!(!has_mixed_casing(stem), "{stem}");
    }
}
