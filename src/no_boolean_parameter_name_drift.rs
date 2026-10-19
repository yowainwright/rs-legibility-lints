use rustc_errors::DiagDecorator;
use rustc_hir::attrs::lang_items::LangItem;
use rustc_hir::{BinOpKind, Expr, ExprKind, LetStmt, Node, PatKind, UnOp};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::Ident;

use crate::prefer_positive_condition_names::PREDICATES;

rustc_session::declare_lint! {
    pub(crate) NO_BOOLEAN_PARAMETER_NAME_DRIFT,
    Warn,
    "LEG079: review boolean names that contradict matching inverted expressions"
}

rustc_session::impl_lint_pass!(NoBooleanParameterNameDrift => [NO_BOOLEAN_PARAMETER_NAME_DRIFT]);

pub(crate) struct NoBooleanParameterNameDrift;

impl<'tcx> LateLintPass<'tcx> for NoBooleanParameterNameDrift {
    fn check_local(&mut self, cx: &LateContext<'tcx>, binding: &'tcx LetStmt<'tcx>) {
        let PatKind::Binding(_, id, name, None) = binding.pat.kind else {
            return;
        };
        if binding.span.from_expansion() {
            return;
        }
        let name_text = name.name.as_str();
        let Some(subject) = positive_subject(name_text) else {
            return;
        };
        let is_boolean = cx.typeck_results().node_type(id).peel_refs().is_bool();
        let has_drift = binding
            .init
            .is_some_and(|value| inverts_subject(cx, value, subject));
        let should_report = is_boolean && has_drift;
        if should_report {
            report(cx, name);
        }
    }
}

fn positive_subject(name: &str) -> Option<&str> {
    let (predicate, subject) = name.split_once('_')?;
    let is_valid = PREDICATES.contains(&predicate) && !subject.is_empty();
    let is_negative = subject.starts_with("not_") || subject.starts_with("no_");
    (is_valid && !is_negative).then_some(subject)
}

fn inverts_subject<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>, subject: &str) -> bool {
    if crate::is_macro_expansion(expr) {
        return false;
    }
    match expr.peel_drop_temps().kind {
        ExprKind::Binary(operator, left, right) if operator.node == BinOpKind::Ne => {
            let left_matches = literal_matches(left, subject) && is_string(cx, right);
            let right_matches = literal_matches(right, subject) && is_string(cx, left);
            left_matches || right_matches
        }
        ExprKind::Unary(UnOp::Not, inner) => negates_local(cx, inner, subject),
        _ => false,
    }
}

fn negates_local<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>, subject: &str) -> bool {
    let is_boolean = cx.typeck_results().expr_ty(expr).peel_refs().is_bool();
    if !is_boolean {
        return false;
    }
    let Some(local) = crate::no_repeated_collection_search::local_binding(cx, expr) else {
        return false;
    };
    let Node::Pat(pat) = cx.tcx.hir_node(local) else {
        return false;
    };
    matches!(pat.kind, PatKind::Binding(_, _, name, _) if name.name.as_str() == subject)
}

fn literal_matches(expr: &Expr<'_>, subject: &str) -> bool {
    if crate::is_macro_expansion(expr) {
        return false;
    }
    let ExprKind::Lit(literal) = expr.kind else {
        return false;
    };
    literal
        .node
        .str()
        .is_some_and(|text| text.as_str().eq_ignore_ascii_case(subject))
}

fn is_string<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    if crate::is_macro_expansion(expr) {
        return false;
    }
    let ty = cx.typeck_results().expr_ty(expr).peel_refs();
    let is_owned = ty
        .ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_lang_item(definition.did(), LangItem::String));
    ty.is_str() || is_owned
}

fn report(cx: &LateContext<'_>, name: Ident) {
    let name_text = name.name;
    cx.emit_span_lint(
        NO_BOOLEAN_PARAMETER_NAME_DRIFT,
        name.span,
        DiagDecorator(|diag| {
            diag.primary_message(format!(
                "LEG079 no-boolean-parameter-name-drift: review `{name_text}` against its inverted expression"
            ));
            diag.help("review the name and expression together; preserve intended behavior when updating uses");
        }),
    );
}
