use rustc_errors::DiagDecorator;
use rustc_hir::def::{DefKind, Res};
use rustc_hir::{Expr, ExprKind};
use rustc_lint::{LateContext, LateLintPass, LintContext};
use rustc_span::sym;

use crate::prefer_switch_over_long_if_chain::is_literal;

rustc_session::declare_lint! {
    pub(crate) NO_SMALL_COLLECTION_CONVERSION,
    Warn,
    "LEG059: use a small literal array directly for one immediate lookup"
}

rustc_session::impl_lint_pass!(NoSmallCollectionConversion => [NO_SMALL_COLLECTION_CONVERSION]);

pub(crate) struct NoSmallCollectionConversion;

impl<'tcx> LateLintPass<'tcx> for NoSmallCollectionConversion {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        if is_small_lookup(cx, expr) {
            report(cx, expr);
        }
    }
}

fn is_small_lookup<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> bool {
    let ExprKind::MethodCall(_, receiver, [needle], _) = expr.kind else {
        return false;
    };
    let should_skip = crate::is_macro_expansion(expr)
        || !is_hashset_contains(cx, expr)
        || !has_primitive_values(cx, needle);
    if should_skip {
        return false;
    }
    let Some(array) = from_argument(cx, receiver) else {
        return false;
    };
    is_small_literal_array(cx, array)
}

fn is_hashset_contains(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let Some(method) = cx.typeck_results().type_dependent_def_id(expr.hir_id) else {
        return false;
    };
    let Some(owner) = cx.tcx.impl_of_assoc(method) else {
        return false;
    };
    let is_inherent = !cx.tcx.impl_is_of_trait(owner);
    let is_hashset = cx
        .tcx
        .type_of(owner)
        .instantiate_identity()
        .skip_norm_wip()
        .ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_diagnostic_item(sym::HashSet, definition.did()));
    is_inherent && is_hashset && cx.tcx.item_name(method).as_str() == "contains"
}

fn from_argument<'tcx>(cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) -> Option<&'tcx Expr<'tcx>> {
    let ExprKind::Call(callee, [argument]) = expr.kind else {
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
    let method = cx.tcx.trait_item_of(method).unwrap_or(method);
    let trait_id = cx.tcx.trait_of_assoc(method)?;
    cx.tcx
        .is_diagnostic_item(sym::From, trait_id)
        .then_some(argument)
}

fn is_small_literal_array(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let ExprKind::Array(values) = expr.kind else {
        return false;
    };
    let is_source = !crate::is_macro_expansion(expr);
    is_source && values.len() < 3 && has_primitive_values(cx, expr) && values.iter().all(is_literal)
}

fn has_primitive_values(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    let ty = cx.typeck_results().expr_ty(expr);
    let ty = if matches!(expr.kind, ExprKind::Array(_)) {
        ty.builtin_index().unwrap_or(ty)
    } else {
        ty
    };
    let ty = ty.peel_refs();
    ty.is_primitive() || ty.is_str()
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.emit_span_lint(
        NO_SMALL_COLLECTION_CONVERSION,
        expr.span.shrink_to_lo(),
        DiagDecorator(|diag| {
            diag.primary_message(
                "LEG059 no-small-collection-conversion: use the small array directly for this lookup",
            );
            diag.help("use array `.contains()` while preserving evaluation order and borrowing");
        }),
    );
}
