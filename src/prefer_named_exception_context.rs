use rustc_errors::DiagDecorator;
use rustc_hir::{BindingMode, Expr, ExprKind, HirId, PatKind};
use rustc_lint::{LateContext, LateLintPass};
use rustc_span::{Symbol, sym};

use crate::no_repeated_collection_search::local_binding;
use crate::no_repeated_comprehension_filter::contains_macro_call;
use crate::prefer_lowercase_error_strings::message_argument;

rustc_session::declare_lint! {
    pub(crate) PREFER_NAMED_EXCEPTION_CONTEXT,
    Allow,
    "LEG078: add operation context when rewrapping an error's string"
}

rustc_session::impl_lint_pass!(PreferNamedExceptionContext => [PREFER_NAMED_EXCEPTION_CONTEXT]);

pub(crate) struct PreferNamedExceptionContext;

impl<'tcx> LateLintPass<'tcx> for PreferNamedExceptionContext {
    fn check_expr(&mut self, cx: &LateContext<'tcx>, expr: &'tcx Expr<'tcx>) {
        let ExprKind::MethodCall(_, _, [closure], _) = expr.kind else {
            return;
        };
        if !is_result_map_err(cx, expr) {
            return;
        }
        let Some((binding, wrapped)) = closure_body(cx, closure) else {
            return;
        };
        let Some(message) = message_argument(cx, wrapped) else {
            return;
        };
        let should_report =
            stringifies_binding(cx, message, binding) && !contains_macro_call(cx, expr);
        if should_report {
            report(cx, wrapped);
        }
    }
}

fn is_result_map_err(cx: &LateContext<'_>, expr: &Expr<'_>) -> bool {
    if crate::is_macro_expansion(expr) {
        return false;
    }
    let Some(method) = cx.typeck_results().type_dependent_def_id(expr.hir_id) else {
        return false;
    };
    if cx.tcx.item_name(method).as_str() != "map_err" {
        return false;
    }
    let Some(owner) = cx.tcx.impl_of_assoc(method) else {
        return false;
    };
    let ty = cx.tcx.type_of(owner).instantiate_identity().skip_norm_wip();
    ty.ty_adt_def()
        .is_some_and(|definition| cx.tcx.is_diagnostic_item(sym::Result, definition.did()))
}

fn closure_body<'tcx>(
    cx: &LateContext<'tcx>,
    expr: &'tcx Expr<'tcx>,
) -> Option<(HirId, &'tcx Expr<'tcx>)> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    let ExprKind::Closure(closure) = expr.kind else {
        return None;
    };
    let body = cx.tcx.hir_body(closure.body);
    let [parameter] = body.params else {
        return None;
    };
    let PatKind::Binding(BindingMode::NONE, binding, _, None) = parameter.pat.kind else {
        return None;
    };
    Some((binding, expression_body(body.value)?))
}

fn expression_body<'hir>(expr: &'hir Expr<'hir>) -> Option<&'hir Expr<'hir>> {
    if crate::is_macro_expansion(expr) {
        return None;
    }
    match expr.peel_drop_temps().kind {
        ExprKind::Block(block, None) if block.stmts.is_empty() => expression_body(block.expr?),
        ExprKind::Block(..) => None,
        _ => Some(expr),
    }
}

fn stringifies_binding(cx: &LateContext<'_>, expr: &Expr<'_>, binding: HirId) -> bool {
    let ExprKind::MethodCall(_, receiver, [], _) = expr.kind else {
        return false;
    };
    if crate::is_macro_expansion(expr) {
        return false;
    }
    let Some(method) = cx.typeck_results().type_dependent_def_id(expr.hir_id) else {
        return false;
    };
    let method = cx.tcx.trait_item_of(method).unwrap_or(method);
    let is_to_string = cx
        .tcx
        .is_diagnostic_item(Symbol::intern("to_string_method"), method);
    is_to_string && local_binding(cx, receiver) == Some(binding)
}

fn report(cx: &LateContext<'_>, expr: &Expr<'_>) {
    cx.tcx.emit_node_span_lint(
        PREFER_NAMED_EXCEPTION_CONTEXT,
        expr.hir_id,
        expr.span.shrink_to_lo(),
        DiagDecorator(|diag| {
            diag.primary_message("LEG078 prefer-named-exception-context: this error wrapper only repeats the original error message");
            diag.help("add operation context and preserve the original error as a source where appropriate");
        }),
    );
}
