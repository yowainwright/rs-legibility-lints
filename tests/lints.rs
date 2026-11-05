const LIBRARY_NAME: &str = "rs_legibility_lints";

#[test]
fn default_limits() {
    dylint_testing::ui::Test::src_base(LIBRARY_NAME, "tests/fixtures/default")
        .rustc_flags(["--edition=2024"])
        .run();
}

#[test]
fn invalid_comment_policy() {
    dylint_testing::ui::Test::src_base(LIBRARY_NAME, "tests/fixtures/invalid_comment_policy")
        .dylint_toml("[rs-legibility-lints]\ncomment-matchers = ['[']\n")
        .run();
}

#[test]
fn configured_limits() {
    dylint_testing::ui::Test::src_base(LIBRARY_NAME, "tests/fixtures/configured")
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
