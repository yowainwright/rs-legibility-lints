# rs-legibility-lints

Rust legibility checks using Clippy and [Dylint](https://github.com/trailofbits/dylint).

## Setup

Install [rustup](https://rustup.rs/) and [fs-lint](https://github.com/yowainwright/fs-lint), with Cargo on PATH.

```sh
make setup
```

Tools stay in `.tools/`; builds and cached drivers stay in `target/`.

---

## Usage

Run checks from this repository:

```sh
make lint-rust        # Enabled Clippy checks
make lint-legibility  # All default custom checks
make lint            # Both, plus file-path checks
```

To enable another Clippy check from the rule index:

```sh
export PATH="$PWD/.tools/bin:$PATH"
cargo clippy --locked --all-targets -- -W clippy::bool_comparison
```

Enabled Clippy checks are in [Cargo.toml](Cargo.toml), with thresholds in [clippy.toml](clippy.toml):
19 body code lines, nesting depth 3, and 5 arguments. Native Clippy counting and exemptions apply.

Custom limits can be set in [dylint.toml](dylint.toml); these are the defaults:

```toml
[rs-legibility-lints]
max-array-chain-depth = 2
max-composite-literal-arg-depth = 1
max-expression-operators = 4
max-if-operators = 0
max-control-flow-depth = 3
max-selector-chain-depth = 3
min-switch-chain-length = 3
comment-matchers = []
comment-prefix-identifiers = []
comment-suffix-identifiers = []
```

Custom rules default to `warn`; opt-in rules LEG025, LEG039, LEG052, LEG053, LEG061, LEG076, LEG077, and LEG078 default to `allow`.
Set [Rust lint levels](https://doc.rust-lang.org/reference/attributes/diagnostics.html#lint-check-attributes)
in the crate root, or use `#[...]` on an individual item:

```rust
#![warn(no_bool_literal_args)]
#![deny(no_getter_prefix)]
#![allow(no_generic_package_names, reason = "legacy module names")]
```

`warn` reports a warning; `deny` reports an error; `allow` disables the check.
Clippy uses the same levels, with names such as `clippy::too_many_lines`.

This repository also sets `build.warnings = "deny"` in [.cargo/config.toml](.cargo/config.toml),
so warnings fail validation. For warnings without failing the build:

```sh
CARGO_BUILD_WARNINGS=warn make lint-legibility
```

The Cargo override preserves explicit `deny` errors. Severity uses Rust lint attributes; `dylint.toml` sets rule thresholds.
These commands check this repository; another project needs its own lint configuration.

---

## Rule coverage

The [shared catalog](https://github.com/yowainwright/legibility/blob/main/docs/rules.md) has 62 entries:
**31 implemented** Dylint checks, **7 Clippy** mappings,
**8 partial Clippy** mappings, and **16 not applicable** to Rust. Every entry has a Rust assessment.

The 31 custom checks include eight opt-in checks.
Clippy mappings follow native Rust behavior; they do not claim parity with other language plugins.

---

## Rules

Removed lines are don'ts. Added lines are dos. Examples are focused snippets, not automatic fixes.

| Code | Rule | Summary |
| --- | --- | --- |
| [`LEG001`](#leg001-max-expression-operators) | `max-expression-operators` | **Implemented.** Limit logical and comparison operators to four per expression. |
| [`LEG002`](#leg002-hoist-if-operators) | `hoist-if-operators` | **Implemented.** Name `if` conditions containing `&&` or `\|\|`. |
| [`LEG003`](#leg003-max-control-flow-depth) | `max-control-flow-depth` | **Implemented.** Limit nested control flow to three levels. |
| [`LEG005`](#leg005-no-quadratic-patterns) | `no-quadratic-patterns` | **Implemented.** Flag nested loops and slice membership searches inside loops. |
| [`LEG006`](#leg006-no-redundant-boolean-logic) | `no-redundant-boolean-logic` | **Clippy.** `bool_comparison`, `nonminimal_bool`: Remove redundant boolean logic. |
| [`LEG007`](#leg007-prefer-positive-condition-names) | `prefer-positive-condition-names` | **Implemented.** Prefer positive boolean binding names. |
| [`LEG008`](#leg008-no-trivial-wrapper-functions) | `no-trivial-wrapper-functions` | **Implemented.** Avoid private functions that only forward their parameters. |
| [`LEG009`](#leg009-prefer-early-return) | `prefer-early-return` | **Clippy.** `redundant_else`: Remove `else` after a branch that exits. |
| [`LEG010`](#leg010-prefer-guard-clauses) | `prefer-guard-clauses` | **Clippy (partial).** `manual_let_else`, `question_mark`: Prefer pattern guards with `let...else` or `?`. |
| [`LEG011`](#leg011-max-array-chain-depth) | `max-array-chain-depth` | **Implemented.** Limit consecutive `Iterator` method calls to two. |
| [`LEG012`](#leg012-no-computed-values) | `no-computed-values` | **Implemented.** Limit returned calculations to one arithmetic operator. |
| [`LEG024`](#leg024-prefer-object-lookup) | `prefer-object-lookup` | **Implemented.** Prefer `matches!` for three or more literal equality checks on one local value. |
| [`LEG025`](#leg025-require-filename-matches-dirname) | `require-filename-matches-dirname` | **Implemented.** Opt-in. Match filenames to their parent directory. |
| [`LEG026`](#leg026-no-mixed-filename-casing) | `no-mixed-filename-casing` | **Implemented.** Flag uppercase letters and hyphens in Rust filenames. |
| [`LEG031`](#leg031-no-deep-selector-chain) | `no-deep-selector-chain` | **Implemented.** Limit consecutive field and index accesses to three. |
| [`LEG034`](#leg034-prefer-switch-over-long-if-chain) | `prefer-switch-over-long-if-chain` | **Implemented.** Prefer `match` for three or more literal comparisons of one local binding. |
| [`LEG035`](#leg035-no-bool-literal-args) | `no-bool-literal-args` | **Implemented.** Name literal boolean arguments. |
| [`LEG036`](#leg036-no-complex-if-init) | `no-complex-if-init` | **Clippy (partial).** `blocks_in_conditions`: Move statement blocks out of conditions. |
| [`LEG037`](#leg037-no-deep-composite-literal-arg) | `no-deep-composite-literal-arg` | **Implemented.** Limit literal nesting in call arguments to one level. |
| [`LEG038`](#leg038-max-function-lines) | `max-function-lines` | **Clippy.** `too_many_lines`: Limit functions to 19 body code lines. |
| [`LEG039`](#leg039-no-unmatched-comments) | `no-unmatched-comments` | **Implemented.** Opt-in. Require comments to match a configured policy. |
| [`LEG040`](#leg040-no-automated-comment-attribution) | `no-automated-comment-attribution` | **Implemented.** Flag explicit automated authorship signatures in ordinary comments. |
| [`LEG041`](#leg041-prefer-line-comments) | `prefer-line-comments` | **Implemented.** Prefer line comments for standalone ordinary block comments. |
| [`LEG042`](#leg042-no-getter-prefix) | `no-getter-prefix` | **Implemented.** Drop `get_` from simple field accessor names. |
| [`LEG043`](#leg043-no-underscore-names) | `no-underscore-names` | **Not applicable.** Rust uses snake_case identifiers. |
| [`LEG044`](#leg044-prefer-initialism-casing) | `prefer-initialism-casing` | **Not applicable.** Rust uses names such as `Uuid` and `is_xid_start`. |
| [`LEG045`](#leg045-no-package-name-stutter) | `no-package-name-stutter` | **Clippy (partial).** `module_name_repetitions`: Avoid repeating a module name in public item names. |
| [`LEG046`](#leg046-no-generic-package-names) | `no-generic-package-names` | **Implemented.** Avoid catch-all module and crate names. |
| [`LEG047`](#leg047-prefer-range-loop) | `prefer-range-loop` | **Clippy.** `needless_range_loop`: Iterate over values when an index only accesses a collection. |
| [`LEG048`](#leg048-no-redundant-break) | `no-redundant-break` | **Not applicable.** Rust `match` arms need no trailing `break`. |
| [`LEG049`](#leg049-no-naked-returns) | `no-naked-returns` | **Not applicable.** Rust has no Go-style named result parameters. |
| [`LEG050`](#leg050-prefer-lowercase-error-strings) | `prefer-lowercase-error-strings` | **Implemented.** Prefer lowercase error messages without trailing `.`, `!`, or `:`. |
| [`LEG051`](#leg051-max-function-params) | `max-function-params` | **Clippy.** `too_many_arguments`: Limit functions to five arguments. |
| [`LEG052`](#leg052-prefer-verb-function-names) | `prefer-verb-function-names` | **Implemented.** Opt-in. Prefer action names for functions. |
| [`LEG053`](#leg053-prefer-boolean-prefixes) | `prefer-boolean-prefixes` | **Implemented.** Opt-in. Prefer predicate or state prefixes for boolean bindings. |
| [`LEG054`](#leg054-no-complex-ternaries) | `no-complex-ternaries` | **Not applicable.** Rust uses `if` expressions rather than ternary syntax. |
| [`LEG055`](#leg055-no-hidden-side-effects) | `no-hidden-side-effects` | **Clippy (partial).** `mixed_read_write_in_expression`: Separate reads and writes within one expression. |
| [`LEG056`](#leg056-no-identity-array-callback) | `no-identity-array-callback` | **Clippy (partial).** `map_identity`: Remove identity iterator maps. |
| [`LEG057`](#leg057-no-repeated-collection-search) | `no-repeated-collection-search` | **Implemented.** Reuse adjacent identical slice membership searches. |
| [`LEG058`](#leg058-no-single-use-renaming-alias) | `no-single-use-renaming-alias` | **Implemented.** Flag single-use `Copy` aliases before a final function call. |
| [`LEG059`](#leg059-no-small-collection-conversion) | `no-small-collection-conversion` | **Implemented.** Use small literal arrays directly for one immediate membership check. |
| [`LEG060`](#leg060-no-standalone-array-mutations) | `no-standalone-array-mutations` | **Not applicable.** Owned `Vec::push` and `extend` are idiomatic Rust. |
| [`LEG061`](#leg061-no-stacked-comments) | `no-stacked-comments` | **Implemented.** Opt-in. Flag consecutive standalone ordinary line comments. |
| [`LEG062`](#leg062-prefer-flat-map) | `prefer-flat-map` | **Clippy.** `map_flatten`: Replace `map(...).flatten()` with `flat_map(...)`. |
| [`LEG063`](#leg063-no-direct-bin-smoke) | `no-direct-bin-smoke` | **Not applicable.** Cargo binary integration tests are valid; installation checks belong in release/CI. |
| [`LEG064`](#leg064-no-redundant-nullish-fallback) | `no-redundant-nullish-fallback` | **Not applicable.** Rust has no JavaScript nullish coalescing. |
| [`LEG065`](#leg065-require-jsdoc-multiline-comments) | `require-jsdoc-multiline-comments` | **Not applicable.** Rust uses rustdoc rather than JSDoc. |
| [`LEG066`](#leg066-no-unnecessary-async) | `no-unnecessary-async` | **Clippy (partial).** `unused_async`, `redundant_async_block`: Remove unnecessary async functions and blocks. |
| [`LEG067`](#leg067-no-unnecessary-block-callback) | `no-unnecessary-block-callback` | **Clippy (partial).** `needless_return`: Remove redundant `return` in closures. |
| [`LEG068`](#leg068-prefer-concat-object-assign) | `prefer-concat-object-assign` | **Not applicable.** JavaScript spread and `Object.assign` policy does not transfer. |
| [`LEG069`](#leg069-require-executable-shebang) | `require-executable-shebang` | **Not applicable.** Cargo-built binaries need no source shebang. |
| [`LEG071`](#leg071-no-unnecessary-lambda) | `no-unnecessary-lambda` | **Clippy.** `redundant_closure`, `redundant_closure_for_method_calls`: Remove forwarding closures. |
| [`LEG072`](#leg072-prefer-flat-comprehension) | `prefer-flat-comprehension` | **Not applicable.** Iterator flattening is covered by LEG062. |
| [`LEG073`](#leg073-no-redundant-none-fallback) | `no-redundant-none-fallback` | **Clippy (partial).** `needless_match`: Remove identity `Option` matches. |
| [`LEG074`](#leg074-no-identity-comprehension) | `no-identity-comprehension` | **Not applicable.** Identity iterator mapping is covered by LEG056. |
| [`LEG075`](#leg075-prefer-comprehension-over-map-filter) | `prefer-comprehension-over-map-filter` | **Not applicable.** Iterator `map` and `filter` are idiomatic Rust. |
| [`LEG076`](#leg076-no-loop-append-comprehension) | `no-loop-append-comprehension` | **Implemented.** Opt-in. Prefer `map(...).collect()` for simple vector-building loops. |
| [`LEG077`](#leg077-no-repeated-comprehension-filter) | `no-repeated-comprehension-filter` | **Implemented. Opt-in.** Flag adjacent identical filters over an immutable primitive collection. |
| [`LEG078`](#leg078-prefer-named-exception-context) | `prefer-named-exception-context` | **Implemented. Opt-in.** Add operation context when rewrapping an error's string in `Result::map_err`. |
| [`LEG079`](#leg079-no-boolean-parameter-name-drift) | `no-boolean-parameter-name-drift` | **Implemented.** Review positive boolean names assigned matching-subject inversions. |
| [`LEG081`](#leg081-prefer-functions) | `prefer-functions` | **Not applicable.** Rust binaries enter through `main`. |
| [`LEG082`](#leg082-use-defaults-in-functions) | `use-defaults-in-functions` | **Not applicable.** Rust functions have typed parameters, not Shell expansions. |

<a id="leg001-max-expression-operators"></a>

### `LEG001 max-expression-operators`

Calls contribute to the count; blocks, closures, and async bodies are separate.
Reports once per complete expression, including nested calls. Arithmetic and macro-generated operators are excluded.

#### do / don't

```diff
- values[0] && values[1] && values[2] && values[3] && values[4] && values[5]
+ let first_group = values[0] && values[1] && values[2];
+ first_group && values[3] && values[4] && values[5]
```

<a id="leg002-hoist-if-operators"></a>

### `LEG002 hoist-if-operators`

Comparisons and `!` do not count. Pattern bindings and let chains are excluded.

#### do / don't

```diff
- if is_ready && is_allowed { proceed(); }
+ let can_proceed = is_ready && is_allowed;
+ if can_proceed { proceed(); }
```

<a id="leg003-max-control-flow-depth"></a>

### `LEG003 max-control-flow-depth`

Count `if`, loops, and `match`. An `else if` stays at the same level; separate bodies reset depth.

#### do / don't

```diff
- if is_ready {
-     if is_allowed {
-         if has_credit {
-             if can_send { send(); }
-         }
-     }
- }
+ if !is_ready { return; }
+ if is_allowed {
+     if has_credit {
+         if can_send { send(); }
+     }
+ }
```

<a id="leg005-no-quadratic-patterns"></a>

### `LEG005 no-quadratic-patterns`

Recognizes slice `.contains`, including arrays and Vecs; set lookups stay quiet.
This is a pattern heuristic, not proof of runtime complexity.

#### do / don't

```diff
+ let available_ids: std::collections::HashSet<_> = available.iter().copied().collect();
  for id in selected {
-     if available.contains(id) { visit(id); }
+     if available_ids.contains(id) { visit(id); }
  }
```

<a id="leg006-no-redundant-boolean-logic"></a>

### `LEG006 no-redundant-boolean-logic`

#### do / don't

```diff
- value == true
+ value
```

<a id="leg007-prefer-positive-condition-names"></a>

### `LEG007 prefer-positive-condition-names`

Checks typed boolean bindings, including parameters; excludes fields, constants, and macros.

#### do / don't

```diff
- let is_not_ready = status != "ready";
- if !is_not_ready { proceed(); }
+ let is_ready = status == "ready";
+ if is_ready { proceed(); }
```

<a id="leg008-no-trivial-wrapper-functions"></a>

### `LEG008 no-trivial-wrapper-functions`

Checks private functions and inherent methods forwarding every parameter in order.
Explicit visibility, trait methods, async/const/unsafe/extern boundaries, and generated code are excluded.
Argument adjustments that change types or ownership are excluded.

#### do / don't

```diff
- fn clean_name(name: &str) -> &str { name.trim() }
- let normalized = clean_name(input);
+ let normalized = input.trim();
```

<a id="leg009-prefer-early-return"></a>

### `LEG009 prefer-early-return`

#### do / don't

```diff
  if is_ready { return 1; }
- else { return 0; }
+ return 0;
```

<a id="leg010-prefer-guard-clauses"></a>

### `LEG010 prefer-guard-clauses`

Clippy covers specific guard forms rather than every large enclosing `if`.

#### do / don't

```diff
- let number = if let Some(number) = value { number } else { return 0; };
+ let Some(number) = value else { return 0; };
```

<a id="leg011-max-array-chain-depth"></a>

### `LEG011 max-array-chain-depth`

Counts resolved `Iterator` methods, including terminal calls such as `any()` and `collect()`.
Iterator creation (`iter()`/`into_iter()`), unrelated builders, and macro-generated code are excluded.

#### do / don't

```diff
- let has_large = values.iter().filter(|v| **v > 0).map(|v| *v * 2).any(|v| v > 100);
+ let positive = values.iter().filter(|v| **v > 0);
+ let mut doubled = positive.map(|v| *v * 2);
+ let has_large = doubled.any(|v| v > 100);
```

<a id="leg012-no-computed-values"></a>

### `LEG012 no-computed-values`

Allows one binary arithmetic operator (`+`, `-`, `*`, `/`, `%`) per returned calculation.
Checks explicit returns and function, closure, and async tail values, including `if`/`match` arms.
Parentheses, casts, borrows, and unary operators preserve the chain without adding to the count.
Calls, composite values, and other binary operators break the chain. Initializers, loop-break values,
and macro expansions are excluded. No automatic fix.

#### do / don't

```diff
- return subtotal + tax + shipping;
+ let taxed_subtotal = subtotal + tax;
+ return taxed_subtotal + shipping;
```

<a id="leg024-prefer-object-lookup"></a>

### `LEG024 prefer-object-lookup`

Flags three or more `==` comparisons joined by `||`, all testing the same local binding against literals.
Supports integers, characters, booleans, and borrowed strings; reversed comparisons and parentheses are accepted.
Two-comparison chains, fields, calls, constants, floats, owned strings, custom equality, and macro expansions are excluded.
Reports once per complete qualifying chain. No automatic fix.

#### do / don't

```diff
- status == "ready" || status == "queued" || status == "retrying"
+ matches!(status, "ready" | "queued" | "retrying")
```

<a id="leg025-require-filename-matches-dirname"></a>

### `LEG025 require-filename-matches-dirname`

Checks compiled local module filenames: the stem must equal the parent directory or start with its name plus `_`.
Matching is case-sensitive. Preserves the crate entry file, files directly under `src/`, and `lib.rs`, `main.rs`, and `mod.rs`.
Unused and cfg-disabled files are excluded. No automatic rename.

```rust
#![warn(require_filename_matches_dirname)]
```

#### do / don't

```diff
- src/orders/service.rs
+ src/orders/orders_service.rs
```

<a id="leg026-no-mixed-filename-casing"></a>

### `LEG026 no-mixed-filename-casing`

Checks compiled local `.rs` module filenames, once per file and effective lint level.
Separate lint expectations are preserved. Update module paths when renaming.
Unused and cfg-disabled files are outside this compiler lint's scope.

#### do / don't

```diff
- src/API_client.rs
+ src/api_client.rs
```

<a id="leg031-no-deep-selector-chain"></a>

### `LEG031 no-deep-selector-chain`

Counts fields, tuple fields, and indexing. Calls break chains; macro-generated accesses are excluded.
Use references when extracting intermediate values to preserve ownership.

#### do / don't

```diff
- config.user.profile.settings.email.enabled
+ let profile = &config.user.profile;
+ let email = &profile.settings.email;
+ email.enabled
```

<a id="leg034-prefer-switch-over-long-if-chain"></a>

### `LEG034 prefer-switch-over-long-if-chain`

Counts `==` branches comparing the same local binding with integer, character, boolean, or string literals.
The final `else` does not count. Calls, fields, floats, overloaded equality, and macro-generated comparisons are excluded.

#### do / don't

```diff
- let label = if status == 1 { "new" } else if status == 2 { "active" } else if status == 3 { "closed" } else { "other" };
+ let label = match status {
+     1 => "new",
+     2 => "active",
+     3 => "closed",
+     _ => "other",
+ };
```

<a id="leg035-no-bool-literal-args"></a>

### `LEG035 no-bool-literal-args`

Flags each direct `true` or `false` argument in function, method, and constructor calls.
Named values, method receivers, and calls containing macro uses are excluded. No automatic fix.

#### do / don't

```diff
- save(true);
+ let should_overwrite = true;
+ save(should_overwrite);
```

<a id="leg036-no-complex-if-init"></a>

### `LEG036 no-complex-if-init`

Uses Clippy's `blocks_in_conditions`, enabled by `make lint-rust`.
Flags statement blocks in ordinary `if`/`match` conditions; ordinary `if let` and let-chains remain allowed.
This is partial coverage: it does not enforce the Go/C++ initializer operator limit.

#### do / don't

```diff
- if {
-     let user = load_user();
-     user.is_some()
- } {
+ let has_user = load_user().is_some();
+ if has_user {
```

<a id="leg037-no-deep-composite-literal-arg"></a>

### `LEG037 no-deep-composite-literal-arg`

Counts struct and enum-variant literals, arrays (including repeats), and tuples. Borrows do not add depth.
Calls and blocks break nesting; macro expansions and method receivers are excluded. No automatic fix.

#### do / don't

```diff
- save(Config { http: HttpConfig { timeout: 10 } });
+ let http = HttpConfig { timeout: 10 };
+ save(Config { http });
```

<a id="leg038-max-function-lines"></a>

### `LEG038 max-function-lines`

Blank/comment-only lines and outer braces do not count. Extract a focused helper above the limit.

<a id="leg039-no-unmatched-comments"></a>

### `LEG039 no-unmatched-comments`

Checks each ordinary line or block comment against case-insensitive regexes, prefixes, or suffixes.
Any match allows the comment. Matching trims whitespace and block-comment stars;
prefixes and suffixes require a word boundary. Empty identifiers are ignored; invalid regexes fail compilation.
Enabling the rule with empty lists rejects ordinary comments.

Preserves rustdoc, strings, SPDX/copyright headers, generation markers, and `//@ ` / `//~` compiler-test directives.
Other license or tool comments need an allowed matcher. No automatic fix.

```rust
#![warn(no_unmatched_comments)]
```

```toml
[rs-legibility-lints]
comment-matchers = ['\bENG-[0-9]+\b']
comment-prefix-identifiers = ["WHY"]
comment-suffix-identifiers = ["REVIEWED"]
```

#### do / don't

```diff
- // Retry the request.
+ // WHY: The provider rejects requests while its rate limit resets.
```

<a id="leg040-no-automated-comment-attribution"></a>

### `LEG040 no-automated-comment-attribution`

Flags `@author` signatures and phrases such as `Generated by Codex`, `Written by an AI`,
and `Copilot-generated` in ordinary line and block comments.
Matches whole words without case sensitivity: `ai`, `chatgpt`, `claude`, `codex`, `copilot`,
`gemini`, `gpt`, `llm`, and `openai`, with `authored`, `created`, `generated`, `produced`, or `written`.
Preserves rustdoc, strings, `@generated` markers, and `Code generated ... DO NOT EDIT.` headers.
No automatic fix.

#### do / don't

```diff
- // Generated by Codex. Keep provider retry order.
+ // Keep provider retry order.
```

<a id="leg041-prefer-line-comments"></a>

### `LEG041 prefer-line-comments`

Flags ordinary block comments that occupy their own lines, including single-line `/* ... */` comments.
Preserves rustdoc, strings, comments sharing a line with code, and comments inside expressions.
No automatic fix.

#### do / don't

```diff
- /* Preserve provider retry order. */
+ // Preserve provider retry order.
```

<a id="leg042-no-getter-prefix"></a>

### `LEG042 no-getter-prefix`

Flags inherent methods with only a `self` receiver that directly return a field, optionally borrowed.
Indexed lookups, computed results, trait methods, and macro-generated code are excluded. No automatic rename.

#### do / don't

```diff
- fn get_owner(&self) -> &str { &self.owner }
+ fn owner(&self) -> &str { &self.owner }
```

<a id="leg043-no-underscore-names"></a>

### `LEG043 no-underscore-names`

Preserve Rust's `snake_case` naming convention.

<a id="leg044-prefer-initialism-casing"></a>

### `LEG044 prefer-initialism-casing`

Preserve Rust-style initialisms such as `Uuid` rather than requiring Go-style capitalization.

<a id="leg045-no-package-name-stutter"></a>

### `LEG045 no-package-name-stutter`

Clippy checks public items; broader crate-wide coverage remains unverified.

#### do / don't

```diff
  pub mod user {
-     pub struct UserRecord;
+     pub struct Record;
  }
```

<a id="leg046-no-generic-package-names"></a>

### `LEG046 no-generic-package-names`

Flags exact crate or module names: `base`, `common`, `helper`, `helpers`, `lib`, `libs`, `misc`, `shared`, `util`, and `utils`.
Macro-generated modules are excluded. Name the module for its contents; this example contains retry behavior.

#### do / don't

```diff
- mod util;
+ mod retry;
```

<a id="leg047-prefer-range-loop"></a>

### `LEG047 prefer-range-loop`

This example uses a slice of `Copy` values.

#### do / don't

```diff
- for index in 0..values.len() { visit(values[index]); }
+ for &value in values { visit(value); }
```

<a id="leg048-no-redundant-break"></a>

### `LEG048 no-redundant-break`

A loop's `break` remains meaningful; `match` arms have no switch-style trailing `break`.

<a id="leg049-no-naked-returns"></a>

### `LEG049 no-naked-returns`

A unit-returning `return;` can be a valid guard.

<a id="leg050-prefer-lowercase-error-strings"></a>

### `LEG050 prefer-lowercase-error-strings`

Checks direct string literals in `std::io::Error::new` and `Error::other`, including import and type aliases.
Preserves initialisms and names with internal capitals, such as `HTTP` and `TLSConfig`.
Dynamic messages, unrelated constructors, and calls containing macro uses are excluded. No automatic fix.

#### do / don't

```diff
- std::io::Error::other("Invalid user.")
+ std::io::Error::other("invalid user")
```

<a id="leg051-max-function-params"></a>

### `LEG051 max-function-params`

Group related inputs into a meaningful type and update callers.
Receivers count; native ABI and trait-implementation exemptions apply.

#### do / don't

```diff
- fn connect(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) {}
+ fn connect(options: ConnectionOptions) {}
```

<a id="leg052-prefer-verb-function-names"></a>

### `LEG052 prefer-verb-function-names`

Opt-in with `#![warn(prefer_verb_function_names)]`.
Flags `config_`, `data_`, `info_`, `item_`, `object_`, `result_`, `settings_`, `stuff_`, `thing_`, and `value_` with a nonempty suffix.
Bare nouns, simple field accessors, trait methods, macro expansions, and foreign/exported functions are excluded.
Choose the verb from the function's behavior; this example loads data. No automatic rename.

#### do / don't

```diff
- fn data_fn() {
+ fn load_data() {
```

<a id="leg053-prefer-boolean-prefixes"></a>

### `LEG053 prefer-boolean-prefixes`

Opt-in with `#![warn(prefer_boolean_prefixes)]`. Checks boolean bindings, including parameters, aliases, references, and patterns.
Prefixes require `_` and a subject: `is`, `are`, `was`, `were`, `has`, `have`, `had`, `can`, `could`, `should`, `will`, `would`, `did`, and `does`.
Allows `debug`, `disabled`, `done`, `empty`, `enabled`, `exists`, `found`, `invalid`, `ok`, `ready`, and `valid`.
Fields, constants, statics, `self`, names starting with `_`, and macro-generated bindings are excluded. No automatic rename.

#### do / don't

```diff
- let active: bool = user.enabled;
- if active { proceed(); }
+ let is_active: bool = user.enabled;
+ if is_active { proceed(); }
```

<a id="leg054-no-complex-ternaries"></a>

### `LEG054 no-complex-ternaries`

Apply LEG001–LEG003 to complex value-producing `if` expressions.

<a id="leg055-no-hidden-side-effects"></a>

### `LEG055 no-hidden-side-effects`

Preserve evaluation order and values. Broader callback mutation needs a Rust definition.

#### do / don't

```diff
- (value, { value += 1; value })
+ let previous = value;
+ value += 1;
+ (previous, value)
```

<a id="leg056-no-identity-array-callback"></a>

### `LEG056 no-identity-array-callback`

Constant-true filters still need assessment.

#### do / don't

```diff
- values.iter().copied().map(|value| value).collect::<Vec<_>>()
+ values.iter().copied().collect::<Vec<_>>()
```

<a id="leg057-no-repeated-collection-search"></a>

### `LEG057 no-repeated-collection-search`

Flags adjacent `let` initializers calling slice `contains` on the same local collection and needle bindings.
Supports slices, arrays, and `Vec` with integer, floating-point, boolean, or character equality, including references.
Intervening statements, custom equality/dereferencing, other search methods, and macro expansions are excluded. No automatic fix.

#### do / don't

```diff
- let is_visible = ids.contains(&id);
- let is_enabled = ids.contains(&id);
+ let is_present = ids.contains(&id);
+ let is_visible = is_present;
+ let is_enabled = is_present;
```

<a id="leg058-no-single-use-renaming-alias"></a>

### `LEG058 no-single-use-renaming-alias`

Checks immutable, unannotated `Copy` aliases passed directly to a block's final single-argument function call, including tail calls.
Source bindings must also be immutable. Moves, borrow expressions, callbacks, method calls, intervening statements, and macro expansions are excluded.
No automatic fix.

#### do / don't

```diff
- let user_data = user;
- show(user_data);
+ show(user);
```

<a id="leg059-no-small-collection-conversion"></a>

### `LEG059 no-small-collection-conversion`

Flags resolved `HashSet::from([...]).contains(...)` calls with fewer than three literal primitive items.
Supports integers, booleans, characters, and string literals, including empty arrays and negative integers.
Named inputs, arrays with three or more items, custom keys/query types, other collections/construction forms,
and macro expansions are excluded. No automatic fix.

#### do / don't

```diff
- HashSet::from(["done", "failed"]).contains(&status)
+ ["done", "failed"].contains(&status)
```

<a id="leg060-no-standalone-array-mutations"></a>

### `LEG060 no-standalone-array-mutations`

Keep ordinary owned collection mutation; assess hidden effects under LEG055.

<a id="leg061-no-stacked-comments"></a>

### `LEG061 no-stacked-comments`

Flags the second and each subsequent consecutive standalone ordinary `//` comment.
Blank lines, code, other comment forms, rustdoc, and recognized headers break the stack.
Preserves SPDX/copyright headers, generation markers, and `//@ ` / `//~` compiler-test directives.
Matching LEG039's configured policy does not exempt ordinary comments. No automatic fix.

```rust
#![warn(no_stacked_comments)]
```

#### do / don't

```diff
- // Retry failed requests.
- // Preserve their order.
+ // Retry failed requests in order.
```

<a id="leg062-prefer-flat-map"></a>

### `LEG062 prefer-flat-map`

#### do / don't

```diff
- values.into_iter().map(expand).flatten().collect::<Vec<_>>()
+ values.into_iter().flat_map(expand).collect::<Vec<_>>()
```

<a id="leg063-no-direct-bin-smoke"></a>

### `LEG063 no-direct-bin-smoke`

Not applicable as a Rust source lint. Cargo integration tests may launch the built binary using
`CARGO_BIN_EXE_<name>`; see [Cargo's integration-test documentation](https://doc.rust-lang.org/cargo/reference/cargo-targets.html#integration-tests).
Installed-command smoke checks belong in release/CI validation.

<a id="leg064-no-redundant-nullish-fallback"></a>

### `LEG064 no-redundant-nullish-fallback`

Rust's `?` propagates `Option` or `Result`; see LEG073 for identity `Option` fallbacks.

<a id="leg065-require-jsdoc-multiline-comments"></a>

### `LEG065 require-jsdoc-multiline-comments`

Use rustdoc for documentation without turning every ordinary comment into documentation.

<a id="leg066-no-unnecessary-async"></a>

### `LEG066 no-unnecessary-async`

Removing `async` changes the return type; update callers and preserve API contracts.

#### do / don't

```diff
- async fn value() -> u8 { 1 }
+ fn value() -> u8 { 1 }
```

<a id="leg067-no-unnecessary-block-callback"></a>

### `LEG067 no-unnecessary-block-callback`

This removes redundant returns, without requiring removal of every closure block.

#### do / don't

```diff
- let value = || { return 1; };
+ let value = || { 1 };
```

<a id="leg068-prefer-concat-object-assign"></a>

### `LEG068 prefer-concat-object-assign`

Rust struct updates and collection ownership need their own policies.

<a id="leg069-require-executable-shebang"></a>

### `LEG069 require-executable-shebang`

Cargo builds executable binaries; Rust entry-point files need no interpreter shebang.

<a id="leg071-no-unnecessary-lambda"></a>

### `LEG071 no-unnecessary-lambda`

#### do / don't

```diff
- values.into_iter().map(|value| parse(value))
+ values.into_iter().map(parse)
```

<a id="leg072-prefer-flat-comprehension"></a>

### `LEG072 prefer-flat-comprehension`

Use LEG062 for iterator map/flatten chains.

<a id="leg073-no-redundant-none-fallback"></a>

### `LEG073 no-redundant-none-fallback`

Other fallback forms still need assessment.

#### do / don't

```diff
- match value { Some(number) => Some(number), None => None }
+ value
```

<a id="leg074-no-identity-comprehension"></a>

### `LEG074 no-identity-comprehension`

Use LEG056 for identity iterator maps.

<a id="leg075-prefer-comprehension-over-map-filter"></a>

### `LEG075 prefer-comprehension-over-map-filter`

Keep idiomatic iterator `map` and `filter`.

<a id="leg076-no-loop-append-comprehension"></a>

### `LEG076 no-loop-append-comprehension`

Checks a fresh `Vec::new()`, a `for` loop containing only `Vec::push`, and an immediate tail or explicit return.
Requires a local `Vec`, array, or slice; a simple `Copy` primitive binding; and a transformation using that binding,
literals, builtin operators, or casts. Calls within transformations, captured values, extra statements, branching,
early exits, preallocated output vectors, other iterators/types, and macro expansions are excluded. No automatic fix.

```rust
#![warn(no_loop_append_comprehension)]
```

#### do / don't

```diff
- let mut doubled = Vec::new();
- for value in values {
-     doubled.push(value * 2);
- }
- doubled
+ values.into_iter().map(|value| value * 2).collect()
```

<a id="leg077-no-repeated-comprehension-filter"></a>

### `LEG077 no-repeated-comprehension-filter`

Checks adjacent `let` initializers using `.iter().filter(...).collect::<Vec<_>>()` on the same immutable local
`Vec`, array, or slice. Requires the same direct primitive or `str` comparison with a literal; closure binding names may differ.
Different predicates, captures, calls, mutable sources, intervening statements, custom types, extra adapters, and macro uses are excluded.
Review ownership before sharing results; no automatic fix.

```rust
#![warn(no_repeated_comprehension_filter)]
```

#### do / don't

```diff
  let first: Vec<_> = values.iter().filter(|value| **value > 0).collect();
- let second: Vec<_> = values.iter().filter(|value| **value > 0).collect();
+ let second = first.clone();
```

<a id="leg078-prefer-named-exception-context"></a>

### `LEG078 prefer-named-exception-context`

Checks `Result::map_err` closures returning `std::io::Error::new` or `other` with only the bound error's standard
`.to_string()` as the message. Accepts direct expressions or braces without extra statements, including type aliases.
Typed wrappers, preserved error values, added context, custom methods, captured values, and macro uses are excluded.
Logging libraries and other constructors are outside this check. No automatic fix.

Formatted context adds a message; retain a typed source with [`Error::source()`](https://doc.rust-lang.org/std/error/trait.Error.html#method.source)
when callers need the original error.

```rust
#![warn(prefer_named_exception_context)]
```

#### do / don't

```diff
- input.map_err(|error| std::io::Error::other(error.to_string()))
+ input.map_err(|error| {
+     let message = format!("loading profile: {error}");
+     std::io::Error::other(message)
+ })
```

<a id="leg079-no-boolean-parameter-name-drift"></a>

### `LEG079 no-boolean-parameter-name-drift`

Checks `str`/`String` inequalities when the literal matches a positive boolean name's subject, or boolean `!` when the operand's local name matches.
String matching ignores ASCII case. Unrelated subjects, custom operators, and macro expansions are excluded.
Review intent and update uses together; no automatic fix.

#### do / don't

```diff
- let is_ready = status != "ready";
+ let has_other_status = status != "ready";
```

<a id="leg081-prefer-functions"></a>

### `LEG081 prefer-functions`

Rust has no Shell-style top-level executable statements.

<a id="leg082-use-defaults-in-functions"></a>

### `LEG082 use-defaults-in-functions`

Shell positional-parameter expansion rules do not apply to typed Rust parameters.

---

## Development

| Command | Purpose |
| --- | --- |
| `make validate` | Formatting, compilation, lint, and tests; also the default for `make` and `make check`. |
| `make format` / `make format-fix` | Check / apply Rust formatting. |
| `make typecheck` | Compile-check all targets. |
| `make test` | Run compiler fixtures and plugin smoke checks. |
| `make test-unit` / `make test-smoke` | Run unit tests / load the plugin and list its rules. |
| `make test-hooks` | Check hook installation, preservation, and validation failures. |
| `make test-coverage` / `make test-coverage-html` | Write LCOV / HTML coverage to `target/coverage/`. |
| `make build` | Build the release plugin. |
| `make validate-full` | Validation, release build, and coverage. |
| `make install-hooks` | Install the pre-commit hook; also runs during `make setup`. |
| `make release-check` | Full validation, coverage, and a packaged-library smoke test. |

`make lint-session` runs `make lint`. Coverage includes the plugin loaded by compiler fixtures.
Instrumented builds stay in `target/coverage-build/`; reports stay in `target/coverage/`.

The compiler is pinned in [rust-toolchain.toml](rust-toolchain.toml); Dylint uses version 6.3.2.
Rustfmt uses Rust 2024, 100 columns, four spaces, and LF endings.
Module items follow Clippy's native groups without alphabetical sorting.

Lint implementations and unit tests live in [src/](src/).
[tests/lints.rs](tests/lints.rs) runs compiler fixtures from [tests/fixtures/](tests/fixtures/);
`.stderr` files record expected diagnostics.
Add one reviewed rule at a time, with accepted and rejected cases.
Handwritten rule logic must stay safe; unsafe code is denied except for required plugin exports.

The pre-commit hook runs `make validate`. Installation preserves custom hooks and skips CI.
Cargo must be on PATH when committing.

---

## Releases

Version 0.0.1 targets Linux x86-64 and macOS ARM64 through GitHub release archives.
Each archive contains the Dylint library, license, README, and pinned toolchain file.
This crate is not published to crates.io.

Before tagging:

```sh
make release-check
```

Archives appear in `target/release-artifacts/`. The check extracts the archive and loads its library with Dylint.
CI runs packaging on both supported platforms. A `v0.0.1` tag must match `Cargo.toml` and point to a commit on `main`.
The release workflow validates both platforms, audits dependencies, and publishes archives with checksums and provenance.
Artifact attestations require a public repository or GitHub Enterprise Cloud for private repositories
([GitHub requirements](https://github.com/actions/attest#readme)). Scorecard's SARIF upload requires code scanning enabled.

To use a downloaded archive, extract it and install its pinned toolchain plus Dylint 6.3.2:

```sh
rustup toolchain install nightly-2026-08-20 --component rustc-dev --component llvm-tools-preview
cargo install cargo-dylint --locked --version 6.3.2
cargo +nightly-2026-08-20 dylint --no-metadata --lib-path /absolute/path/to/librs_legibility_lints@TOOLCHAIN.EXT -- --all-targets
```

Replace the library path with the extracted filename for your platform. Clippy checks remain separately configured.
