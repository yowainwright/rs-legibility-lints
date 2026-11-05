# Contributing

## Development

Install [rustup](https://rustup.rs/) and [fs-lint](https://github.com/yowainwright/fs-lint), with Cargo on your PATH. Then run:

```sh
make setup
make validate
make build
make test-coverage
```

[`rust-toolchain.toml`](../rust-toolchain.toml) pins the nightly compiler and its components. Keep that pin aligned with Dylint; do not replace it with the latest stable compiler.

[`make validate`](../Makefile) checks formatting, compilation, Clippy, Dylint, file paths, and tests. The integration runner in `tests/lints.rs` checks compiler fixtures under `tests/fixtures/`; loading the plugin alone verifies setup, not rule behavior. Unit tests stay beside their implementations in `src/`.

## Code Style

Use syntax checks unless a rule needs resolved types or methods. Prefer small helpers, early returns, and named intermediate values for complex conditions. Follow Rust conventions where another language's rule differs; see the [rule coverage matrix](../README.md#rule-coverage).

## Pull Requests

Open focused pull requests. Review each new rule's behavior before adding accepted/rejected fixtures and boundary cases. Include a short before-and-after example when changing diagnostics or settings.

## Automation

CI validates Linux and macOS with the pinned toolchain and audits `Cargo.lock`. Version tags must match `Cargo.toml` and point to a commit on `main`; releases attach the platform-specific Dylint libraries, checksums, and build attestations. `publish = false` remains in effect; the workflow does not publish to crates.io.

The dependency workflow requires a `CODEPENDENCE_TOKEN` secret with permission to update workflow files and open pull requests. It updates GitHub Actions and compatible lockfile dependencies; compiler and Dylint version changes stay coordinated. Scorecard publishes results and uploads SARIF when repository code scanning is available.
