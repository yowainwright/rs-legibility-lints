use rustc_errors::DiagDecorator;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Symbol;
use rustc_span::def_id::DefId;

use crate::no_repeated_comprehension_filter::contains_macro_call;

rustc_session::declare_lint! {
    pub(crate) PREFER_LOWERCASE_ERROR_STRINGS,
    Warn,
    "LEG050: prefer lowercase error messages without trailing punctuation"
}

rustc_session::impl_lint_pass!(PreferLowercaseErrorStrings => [PREFER_LOWERCASE_ERROR_STRINGS]);

pub(crate) struct PreferLowercaseErrorStrings;

impl<'tcx> LateLintPass<'tcx> for PreferLowercaseErrorStrings {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let Some(argument) = message_argument(cx, expr) else {
            return;
        };
        let ExprKind::Lit(literal) = argument.kind else {
            return;
        };
        let should_skip = crate::is_macro_expansion(argument) || contains_macro_call(cx, expr);
        if should_skip {
            return;
        }
        let Some(text) = literal.node.str() else {
            return;
        };
        if let Some(issue) = message_issue(text.as_str()) {
            report(cx, argument, issue);
        }
    }
}

pub(crate) fn message_argument<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Call(callee, arguments) = expr.kind else {
        return None;
    };
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let ExprKind::Path(path) = callee.kind else {
        return None;
    };
    let Res::Def(DefKind::AssocFn, method) = cx.qpath_res(&path, callee.hir_id) else {
        return None;
    };
    arguments.get(message_index(cx, method)?)
}

fn message_index(cx: &LateContext<'_>, method: DefId) -> Option<usize> {
    let name = cx.tcx.item_name(method);
    if !matches!(name.as_str(), "new" | "other") {
        return None;
    }
    let constructor = cx.tcx.get_diagnostic_item(Symbol::intern("io_error_new"))?;
    if method == constructor {
        return Some(1);
    }
    let owner = cx.tcx.impl_of_assoc(constructor)?;
    let same_owner = cx.tcx.impl_of_assoc(method) == Some(owner);
    let is_other = name.as_str() == "other";
    (same_owner && is_other).then_some(0)
}

fn message_issue(text: &str) -> Option<&'static str> {
    let word = text
        .split_whitespace()
        .next()
        .unwrap_or_default()
        .trim_matches(['.', '!', ':', ',', ';']);
    if is_capitalized_word(word) {
        return Some("start error messages with a lowercase word");
    }
    text.ends_with(['.', '!', ':'])
        .then_some("omit trailing `.`, `!`, or `:` from error messages")
}

fn is_capitalized_word(word: &str) -> bool {
    let mut characters = word.chars();
    if !characters.next().is_some_and(char::is_uppercase) {
        return false;
    }
    let mut has_lowercase = false;
    for character in characters {
        if character.is_uppercase() {
            return false;
        }
        has_lowercase |= character.is_lowercase();
    }
    has_lowercase
}

fn report(cx: &LateContext<'_>, argument: &Expr<'_>, issue: &str) {
    cx.emit_span_lint(
        PREFER_LOWERCASE_ERROR_STRINGS,
        argument.span,
        DiagDecorator(|diag| {
            diag.primary_message(format!("LEG050 prefer-lowercase-error-strings: {issue}"));
            diag.help("preserve identifiers and acronyms when editing the message");
        }),
    );
}
