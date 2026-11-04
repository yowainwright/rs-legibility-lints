#![feature(rustc_private)]

#[cfg(test)]
extern crate rustc_driver as _;
extern crate rustc_errors;
extern crate rustc_hir;
extern crate rustc_lexer;
extern crate rustc_lint;
extern crate rustc_session;
extern crate rustc_span;

mod comment_policy;
mod comment_rules;
mod hoist_if_operators;
mod max_array_chain_depth;
mod max_control_flow_depth;
mod max_expression_operators;
mod no_bool_literal_args;
mod no_boolean_parameter_name_drift;
mod no_computed_values;
mod no_deep_composite_literal_arg;
mod no_deep_selector_chain;
mod no_generic_package_names;
mod no_getter_prefix;
mod no_loop_append_comprehension;
mod no_mixed_filename_casing;
mod no_quadratic_patterns;
mod no_repeated_collection_search;
mod no_repeated_comprehension_filter;
mod no_single_use_renaming_alias;
mod no_small_collection_conversion;
mod no_trivial_wrapper_functions;
mod prefer_boolean_prefixes;
mod prefer_lowercase_error_strings;
mod prefer_named_exception_context;
mod prefer_object_lookup;
mod prefer_positive_condition_names;
mod prefer_switch_over_long_if_chain;
mod prefer_verb_function_names;
mod require_filename_matches_dirname;

use comment_policy::CommentPolicy;
use comment_rules::{
    CommentRules, NO_AUTOMATED_COMMENT_ATTRIBUTION, NO_STACKED_COMMENTS, NO_UNMATCHED_COMMENTS,
    PREFER_LINE_COMMENTS,
};
use hoist_if_operators::{HOIST_IF_OPERATORS, HoistIfOperators};
use max_array_chain_depth::{MAX_ARRAY_CHAIN_DEPTH, MaxArrayChainDepth};
use max_control_flow_depth::{MAX_CONTROL_FLOW_DEPTH, MaxControlFlowDepth};
use max_expression_operators::{MAX_EXPRESSION_OPERATORS, MaxExpressionOperators};
use no_bool_literal_args::{NO_BOOL_LITERAL_ARGS, NoBoolLiteralArgs};
use no_boolean_parameter_name_drift::{
    NO_BOOLEAN_PARAMETER_NAME_DRIFT, NoBooleanParameterNameDrift,
};
use no_computed_values::{NO_COMPUTED_VALUES, NoComputedValues};
use no_deep_composite_literal_arg::{NO_DEEP_COMPOSITE_LITERAL_ARG, NoDeepCompositeLiteralArg};
use no_deep_selector_chain::{NO_DEEP_SELECTOR_CHAIN, NoDeepSelectorChain};
use no_generic_package_names::{NO_GENERIC_PACKAGE_NAMES, NoGenericPackageNames};
use no_getter_prefix::{NO_GETTER_PREFIX, NoGetterPrefix};
use no_loop_append_comprehension::{NO_LOOP_APPEND_COMPREHENSION, NoLoopAppendComprehension};
use no_mixed_filename_casing::{NO_MIXED_FILENAME_CASING, NoMixedFilenameCasing};
use no_quadratic_patterns::{NO_QUADRATIC_PATTERNS, NoQuadraticPatterns};
use no_repeated_collection_search::{NO_REPEATED_COLLECTION_SEARCH, NoRepeatedCollectionSearch};
use no_repeated_comprehension_filter::{
    NO_REPEATED_COMPREHENSION_FILTER, NoRepeatedComprehensionFilter,
};
use no_single_use_renaming_alias::{NO_SINGLE_USE_RENAMING_ALIAS, NoSingleUseRenamingAlias};
use no_small_collection_conversion::{NO_SMALL_COLLECTION_CONVERSION, NoSmallCollectionConversion};
use no_trivial_wrapper_functions::{NO_TRIVIAL_WRAPPER_FUNCTIONS, NoTrivialWrapperFunctions};
use prefer_boolean_prefixes::{PREFER_BOOLEAN_PREFIXES, PreferBooleanPrefixes};
use prefer_lowercase_error_strings::{PREFER_LOWERCASE_ERROR_STRINGS, PreferLowercaseErrorStrings};
use prefer_named_exception_context::{PREFER_NAMED_EXCEPTION_CONTEXT, PreferNamedExceptionContext};
use prefer_object_lookup::{PREFER_OBJECT_LOOKUP, PreferObjectLookup};
use prefer_positive_condition_names::{
    PREFER_POSITIVE_CONDITION_NAMES, PreferPositiveConditionNames,
};
use prefer_switch_over_long_if_chain::{
    PREFER_SWITCH_OVER_LONG_IF_CHAIN, PreferSwitchOverLongIfChain,
};
use prefer_verb_function_names::{PREFER_VERB_FUNCTION_NAMES, PreferVerbFunctionNames};
use require_filename_matches_dirname::{
    REQUIRE_FILENAME_MATCHES_DIRNAME, RequireFilenameMatchesDirname,
};

#[cfg(not(test))]
dylint_linting::dylint_library!();

#[derive(serde::Deserialize)]
#[serde(default, deny_unknown_fields, rename_all = "kebab-case")]
struct Config {
    comment_matchers: Vec<String>,
    comment_prefix_identifiers: Vec<String>,
    comment_suffix_identifiers: Vec<String>,
    max_array_chain_depth: usize,
    max_composite_literal_arg_depth: usize,
    max_control_flow_depth: usize,
    max_expression_operators: usize,
    max_if_operators: usize,
    max_selector_chain_depth: usize,
    min_switch_chain_length: usize,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            comment_matchers: Vec::new(),
            comment_prefix_identifiers: Vec::new(),
            comment_suffix_identifiers: Vec::new(),
            max_array_chain_depth: 2,
            max_composite_literal_arg_depth: 1,
            max_control_flow_depth: 3,
            max_expression_operators: 4,
            max_if_operators: 0,
            max_selector_chain_depth: 3,
            min_switch_chain_length: 3,
        }
    }
}

#[allow(unsafe_code, reason = "Dylint requires a named registration export")]
#[expect(
    clippy::no_mangle_with_rust_abi,
    reason = "Dylint requires the Rust ABI for register_lints"
)]
#[unsafe(no_mangle)]
pub fn register_lints(session: &rustc_session::Session, store: &mut rustc_lint::LintStore) {
    dylint_linting::init_config(session);
    let config = dylint_linting::config_or_default::<Config>("rs-legibility-lints");
    register_rules(store);
    register_comment_pass(store, &config);
    register_passes(store, config);
}

fn register_comment_pass(store: &mut rustc_lint::LintStore, config: &Config) {
    let matchers = config.comment_matchers.clone();
    let prefixes = config.comment_prefix_identifiers.clone();
    let suffixes = config.comment_suffix_identifiers.clone();
    store.register_late_lint_pass(Box::new(move |tcx| {
        let policy = CommentPolicy::new(&matchers, &prefixes, &suffixes).unwrap_or_else(|error| {
            tcx.dcx()
                .err(format!("invalid comment-matchers in dylint.toml: {error}"));
            CommentPolicy::default()
        });
        Box::new(CommentRules::new(policy))
    }));
}

fn register_rules(store: &mut rustc_lint::LintStore) {
    register_threshold_rules(store);
    register_optional_rules(store);
    store.register_lints(&[
        NO_AUTOMATED_COMMENT_ATTRIBUTION,
        PREFER_LINE_COMMENTS,
        NO_BOOL_LITERAL_ARGS,
        NO_BOOLEAN_PARAMETER_NAME_DRIFT,
        NO_COMPUTED_VALUES,
        NO_GENERIC_PACKAGE_NAMES,
        NO_GETTER_PREFIX,
        NO_MIXED_FILENAME_CASING,
        NO_QUADRATIC_PATTERNS,
        NO_REPEATED_COLLECTION_SEARCH,
        NO_SINGLE_USE_RENAMING_ALIAS,
        NO_TRIVIAL_WRAPPER_FUNCTIONS,
        PREFER_LOWERCASE_ERROR_STRINGS,
        PREFER_POSITIVE_CONDITION_NAMES,
    ]);
}

fn register_optional_rules(store: &mut rustc_lint::LintStore) {
    store.register_lints(&[
        NO_REPEATED_COMPREHENSION_FILTER,
        NO_LOOP_APPEND_COMPREHENSION,
        NO_STACKED_COMMENTS,
        NO_UNMATCHED_COMMENTS,
        PREFER_BOOLEAN_PREFIXES,
        PREFER_NAMED_EXCEPTION_CONTEXT,
        PREFER_VERB_FUNCTION_NAMES,
        REQUIRE_FILENAME_MATCHES_DIRNAME,
    ]);
}

fn register_threshold_rules(store: &mut rustc_lint::LintStore) {
    store.register_lints(&[
        MAX_ARRAY_CHAIN_DEPTH,
        MAX_CONTROL_FLOW_DEPTH,
        MAX_EXPRESSION_OPERATORS,
        HOIST_IF_OPERATORS,
        NO_DEEP_COMPOSITE_LITERAL_ARG,
        NO_DEEP_SELECTOR_CHAIN,
        NO_SMALL_COLLECTION_CONVERSION,
        PREFER_SWITCH_OVER_LONG_IF_CHAIN,
        PREFER_OBJECT_LOOKUP,
    ]);
}

fn register_passes(store: &mut rustc_lint::LintStore, config: Config) {
    register_stateless_passes(store);
    let max = config.max_array_chain_depth;
    store.register_late_lint_pass(Box::new(move |_| Box::new(MaxArrayChainDepth { max })));
    let max = config.max_composite_literal_arg_depth;
    store.register_late_lint_pass(Box::new(move |_| {
        Box::new(NoDeepCompositeLiteralArg { max })
    }));
    let max = config.max_control_flow_depth;
    store.register_late_lint_pass(Box::new(move |_| Box::new(MaxControlFlowDepth { max })));
    let max = config.max_expression_operators;
    store.register_late_lint_pass(Box::new(move |_| Box::new(MaxExpressionOperators { max })));
    let max = config.max_selector_chain_depth;
    store.register_late_lint_pass(Box::new(move |_| Box::new(NoDeepSelectorChain { max })));
    let max = config.max_if_operators;
    store.register_late_lint_pass(Box::new(move |_| Box::new(HoistIfOperators { max })));
    let min = config.min_switch_chain_length;
    store.register_late_lint_pass(Box::new(move |_| {
        Box::new(PreferSwitchOverLongIfChain { min })
    }));
}

fn register_stateless_passes(store: &mut rustc_lint::LintStore) {
    register_optional_passes(store);
    store.register_late_lint_pass(Box::new(|_| Box::new(NoSmallCollectionConversion)));
    store.register_late_lint_pass(Box::new(|_| Box::new(PreferObjectLookup)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoComputedValues)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoBooleanParameterNameDrift)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoSingleUseRenamingAlias)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoRepeatedCollectionSearch)));
    store.register_late_lint_pass(Box::new(|_| Box::new(PreferLowercaseErrorStrings)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoGenericPackageNames)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoGetterPrefix)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoBoolLiteralArgs)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoMixedFilenameCasing::default())));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoTrivialWrapperFunctions)));
    store.register_late_lint_pass(Box::new(|_| Box::new(PreferPositiveConditionNames)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoQuadraticPatterns)));
}

fn register_optional_passes(store: &mut rustc_lint::LintStore) {
    store.register_late_lint_pass(Box::new(|_| Box::new(PreferNamedExceptionContext)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoRepeatedComprehensionFilter)));
    store.register_late_lint_pass(Box::new(|_| Box::new(NoLoopAppendComprehension)));
    store.register_late_lint_pass(Box::new(|_| {
        Box::new(RequireFilenameMatchesDirname::default())
    }));
    store.register_late_lint_pass(Box::new(|_| Box::new(PreferBooleanPrefixes)));
    store.register_late_lint_pass(Box::new(|_| Box::new(PreferVerbFunctionNames)));
}

fn is_macro_expansion(expr: &rustc_hir::Expr<'_>) -> bool {
    expr.span
        .macro_backtrace()
        .any(|expansion| matches!(expansion.kind, rustc_span::ExpnKind::Macro(..)))
}

fn is_source_loop(expr: &rustc_hir::Expr<'_>) -> bool {
    let rustc_hir::ExprKind::Loop(_, _, source, _) = expr.kind else {
        return false;
    };
    source != rustc_hir::LoopSource::Loop || !expr.span.from_expansion()
}

#[test]
fn default_limits() {
    dylint_testing::ui::Test::src_base(env!("CARGO_CRATE_NAME"), "src/tests/default")
        .rustc_flags(["--edition=2024"])
        .run();
}

#[test]
fn invalid_comment_policy() {
    dylint_testing::ui::Test::src_base(
        env!("CARGO_CRATE_NAME"),
        "src/tests/invalid_comment_policy",
    )
    .dylint_toml("[rs-legibility-lints]\ncomment-matchers = ['[']\n")
    .run();
}

#[test]
fn configured_limits() {
    dylint_testing::ui::Test::src_base(env!("CARGO_CRATE_NAME"), "src/tests/configured")
        .rustc_flags(["--edition=2024"])
        .dylint_toml(concat!(
            "[rs-legibility-lints]\n",
            "max-array-chain-depth = 1\n",
            "max-composite-literal-arg-depth = 2\n",
            "max-control-flow-depth = 1\n",
            "max-expression-operators = 1\n",
            "max-if-operators = 1\n",
            "max-selector-chain-depth = 1\n",
            "min-switch-chain-length = 2\n",
            "comment-matchers = ['\\bENG-[0-9]+\\b']\n",
            "comment-prefix-identifiers = ['', '  ', ' WHY ']\n",
            "comment-suffix-identifiers = ['', '  ', ' REVIEWED ']\n",
        ))
        .run();
}
