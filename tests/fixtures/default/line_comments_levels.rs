#![allow(prefer_line_comments, reason = "verify explicit error overrides")]

#[deny(prefer_line_comments)]
fn rejected() {
    /* Explain this function. */
}

fn main() {}
