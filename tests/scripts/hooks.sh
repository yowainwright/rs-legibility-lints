#!/bin/sh
set -eu

main() {
    mkdir -p target/tmp
    test_directory="$(mktemp -d "$PWD/target/tmp/hooks.XXXXXX")"
    trap 'rm -rf "$test_directory"' EXIT HUP INT TERM
    export TEST_ROOT="$test_directory/repository with spaces"
    export TEST_HOOKS="$TEST_ROOT/hooks"
    export TEST_MAKE_ARGS="$test_directory/make-args"
    mkdir -p "$test_directory/bin" "$TEST_ROOT"
    create_git_stub
    create_make_stub
    chmod +x "$test_directory/bin/git" "$test_directory/bin/make"
    export PATH="$test_directory/bin:$PATH"
    test_ci_skip
    test_validation
    test_managed_refresh
    test_custom_preservation
    echo 'Hook tests passed'
}

create_git_stub() {
    cat >"$test_directory/bin/git" <<'STUB'
#!/bin/sh
set -eu
case "$*" in
  'rev-parse --git-path hooks') printf '%s\n' "$TEST_HOOKS" ;;
  'rev-parse --show-toplevel') printf '%s\n' "$TEST_ROOT" ;;
  *) exit 1 ;;
esac
STUB
}

create_make_stub() {
    cat >"$test_directory/bin/make" <<'STUB'
#!/bin/sh
set -eu
printf '%s\n' "$@" >"$TEST_MAKE_ARGS"
exit "${TEST_MAKE_STATUS:-0}"
STUB
}

test_ci_skip() {
    CI=true sh scripts/hooks.sh
    test ! -e "$TEST_HOOKS/pre-commit"
    CI=1 sh scripts/hooks.sh
    test ! -e "$TEST_HOOKS/pre-commit"
}

test_validation() {
    CI=false sh scripts/hooks.sh
    test -x "$TEST_HOOKS/pre-commit"
    "$TEST_HOOKS/pre-commit"
    printf '%s\n' '-C' "$TEST_ROOT" 'validate' >"$test_directory/expected"
    cmp "$test_directory/expected" "$TEST_MAKE_ARGS"
    status=0
    TEST_MAKE_STATUS=7 "$TEST_HOOKS/pre-commit" || status=$?
    test "$status" -eq 7
}

test_managed_refresh() {
    printf '\n# obsolete managed hook\n' >>"$TEST_HOOKS/pre-commit"
    CI=false sh scripts/hooks.sh
    if grep -q 'obsolete managed hook' "$TEST_HOOKS/pre-commit"; then
        echo 'Managed hook was not refreshed' >&2
        exit 1
    fi
}

test_custom_preservation() {
    printf '%s\n' '#!/bin/sh' 'exit 42' >"$TEST_HOOKS/pre-commit"
    cp "$TEST_HOOKS/pre-commit" "$test_directory/custom-hook"
    CI=false sh scripts/hooks.sh
    cmp "$test_directory/custom-hook" "$TEST_HOOKS/pre-commit"
    rm "$TEST_HOOKS/pre-commit"
    ln -s "$test_directory/custom-hook" "$TEST_HOOKS/pre-commit"
    CI=false sh scripts/hooks.sh
    test -L "$TEST_HOOKS/pre-commit"
    cmp "$test_directory/custom-hook" "$TEST_HOOKS/pre-commit"
}

main
