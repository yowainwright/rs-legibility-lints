#!/usr/bin/env bash
set -euo pipefail

main() {
    package_id="$(cargo pkgid --locked)"
    version="${package_id##*#}"
    version="${version##*@}"
    release_tag="${RELEASE_TAG:-v$version}"
    if [[ "$release_tag" != "v$version" ]]; then
        echo "Release tag must match package version: v$version" >&2
        exit 1
    fi
    libraries=(target/release/librs_legibility_lints@*)
    test "${#libraries[@]}" -eq 1
    test -f "${libraries[0]}"
    package_archive
    verify_archive
    printf 'Verified %s\n' "$archive"
}

package_archive() {
    host="$(rustc -vV | sed -n 's/^host: //p')"
    mkdir -p target/release-artifacts target/tmp
    archive="$PWD/target/release-artifacts/rs-legibility-lints-$release_tag-$host.tar.gz"
    library="${libraries[0]##*/}"
    tar -czf "$archive" -C target/release "$library" -C "$PWD" LICENSE README.md rust-toolchain.toml
}

verify_archive() {
    unpacked="$(mktemp -d "$PWD/target/tmp/release.XXXXXX")"
    trap 'rm -rf "$unpacked"' EXIT
    tar -xzf "$archive" -C "$unpacked"
    cargo dylint list --no-metadata --lib-path "$unpacked/$library" --fail-on-no-libraries
}

main
