.DEFAULT_GOAL := validate

CARGO ?= cargo
export PATH := $(CURDIR)/.tools/bin:$(PATH)
export TMPDIR := $(CURDIR)/target/tmp
export DYLINT_DRIVER_PATH := $(CURDIR)/target/dylint-drivers

.PHONY: setup prepare build typecheck format format-fix lint lint-rust \
 lint-legibility lint-paths lint-session test test-unit test-smoke \
 test-coverage test-coverage-html validate validate-full check

setup build typecheck format format-fix lint-rust lint-legibility \
 test test-unit test-smoke test-coverage test-coverage-html: | prepare

setup:
	rustup show active-toolchain
	CARGO_TARGET_DIR="$(CURDIR)/target/tool-install" $(CARGO) install --locked \
		--version 6.3.2 --root .tools cargo-dylint dylint-link
	CARGO_TARGET_DIR="$(CURDIR)/target/tool-install" $(CARGO) install --locked \
		--version 0.9.1 --root .tools cargo-llvm-cov

prepare:
	mkdir -p "$(TMPDIR)" "$(DYLINT_DRIVER_PATH)"

build:
	$(CARGO) build --locked --release

typecheck:
	$(CARGO) check --locked --all-targets

format:
	$(CARGO) fmt --all -- --check

format-fix:
	$(CARGO) fmt --all

lint: lint-rust lint-legibility lint-paths

lint-rust:
	$(CARGO) clippy --locked --all-targets

lint-legibility:
	$(CARGO) dylint --all -- --all-targets

lint-paths:
	env fs-lint
	bash -o pipefail -c 'git ls-files --cached --others --exclude-standard -z | fs-lint check --stdin0'

lint-session: lint

test: test-smoke
	$(CARGO) test --locked

test-unit:
	$(CARGO) test --locked --lib

test-smoke:
	$(CARGO) dylint list --all

test-coverage:
	mkdir -p target/coverage
	CARGO_TARGET_DIR="$(CURDIR)/target/coverage-build" bash -euo pipefail -c \
		'coverage_env="$$($(CARGO) llvm-cov show-env --sh)"; eval "$$coverage_env"; \
		$(CARGO) llvm-cov clean --workspace; \
		$(CARGO) test --locked --all-targets; \
		$(CARGO) llvm-cov report --lcov --output-path target/coverage/lcov.info'

test-coverage-html: test-coverage
	CARGO_TARGET_DIR="$(CURDIR)/target/coverage-build" bash -euo pipefail -c \
		'coverage_env="$$($(CARGO) llvm-cov show-env --sh)"; eval "$$coverage_env"; \
		$(CARGO) llvm-cov report --html --output-dir target/coverage'

validate: format typecheck lint test

validate-full: validate build test-coverage

check: validate
