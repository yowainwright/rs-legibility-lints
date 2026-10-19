use std::collections::HashSet;

use rustc_errors::DiagDecorator;
use rustc_hir::{CRATE_HIR_ID, HirId, Mod};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_session::lint::{Level, StableLintExpectationId};
use rustc_span::{BytePos, FileName, Span};

rustc_session::declare_lint! {
    pub(crate) REQUIRE_FILENAME_MATCHES_DIRNAME,
    Allow,
    "LEG025: require filenames to match their parent directory"
}

rustc_session::impl_lint_pass!(RequireFilenameMatchesDirname => [REQUIRE_FILENAME_MATCHES_DIRNAME]);

#[derive(Default)]
pub(crate) struct RequireFilenameMatchesDirname {
    root: Option<BytePos>,
    reported: HashSet<(BytePos, Level, Option<StableLintExpectationId>)>,
}

impl<'tcx> LateLintPass<'tcx> for RequireFilenameMatchesDirname {
    fn check_mod(&mut self, cx: &LateContext<'tcx>, module: &'tcx Mod<'tcx>, owner: HirId) {
        let span = module.spans.inner_span;
        if span.from_expansion() {
            return;
        }
        let file = cx.sess().source_map().lookup_source_file(span.lo());
        if owner == CRATE_HIR_ID {
            self.root = Some(file.start_pos);
            return;
        }
        self.check_file(cx, &file.name, span.with_lo(file.start_pos));
    }
}

impl RequireFilenameMatchesDirname {
    fn check_file(&mut self, cx: &LateContext<'_>, filename: &FileName, span: Span) {
        let level = cx.get_lint_level_spec(REQUIRE_FILENAME_MATCHES_DIRNAME);
        let should_skip = self.root == Some(span.lo()) || level.is_allow();
        if should_skip {
            return;
        }
        let Some((stem, parent)) = mismatched_filename(filename) else {
            return;
        };
        let key = (span.lo(), level.level(), level.lint_id());
        if self.reported.insert(key) {
            report(cx, span.shrink_to_lo(), stem, parent);
        }
    }
}

fn mismatched_filename(filename: &FileName) -> Option<(&str, &str)> {
    let FileName::Real(real) = filename else {
        return None;
    };
    let path = real.local_path()?;
    let stem = path.file_name()?.to_str()?.strip_suffix(".rs")?;
    let parent = path.parent()?.file_name()?.to_str()?;
    (!matches_dirname(stem, parent)).then_some((stem, parent))
}

fn matches_dirname(stem: &str, parent: &str) -> bool {
    let is_entry = matches!(stem, "lib" | "main" | "mod") || parent == "src";
    let has_prefix = stem
        .strip_prefix(parent)
        .is_some_and(|suffix| suffix.starts_with('_'));
    is_entry || stem == parent || has_prefix
}

fn report(cx: &LateContext<'_>, span: Span, stem: &str, parent: &str) {
    cx.emit_span_lint(
        REQUIRE_FILENAME_MATCHES_DIRNAME,
        span,
        DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG025 require-filename-matches-dirname: `{stem}.rs` must start with `{parent}`"
            ));
            diag.help(format!(
                "use `{parent}.rs` or an `{parent}_` filename prefix; update module paths"
            ));
        }),
    );
}
