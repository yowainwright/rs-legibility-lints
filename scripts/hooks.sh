#!/bin/sh
set -eu

main() {
    case "${CI:-}" in true | 1) return 0 ;; esac
    hooks_directory="$(git rev-parse --git-path hooks 2>/dev/null)" || return 0
    pre_commit_path="$hooks_directory/pre-commit"
    if [ -L "$pre_commit_path" ]; then
        return 0
    fi
    if is_unmanaged_hook "$pre_commit_path"; then
        echo "Preserving existing hook: $pre_commit_path"
        return 0
    fi
    mkdir -p "$hooks_directory"
    install_pre_commit_hook
}

is_unmanaged_hook() {
    [ -e "$1" ] && ! grep -Fq '# legibility-managed-hook' "$1"
}

install_pre_commit_hook() {
    cat >"$pre_commit_path" <<'HOOK'
#!/bin/sh
# legibility-managed-hook
set -eu

repo_root="$(git rev-parse --show-toplevel)"
exec make -C "$repo_root" validate
HOOK
    chmod 755 "$pre_commit_path"
}

main
