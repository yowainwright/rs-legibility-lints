/* Preserve provider retry order. */
fn main() {
    /*
     * Keep retries ordered.
     */
    if true {
        /* Explain this branch. */
    }
}

#[allow(prefer_line_comments, reason = "verify nested warning overrides")]
fn nested_override() {
    #[warn(prefer_line_comments)]
    {
        /* Preserve this explanation. */
    }
}

#[allow(prefer_line_comments, reason = "verify external module suppression")]
#[path = "auxiliary/line_comments.rs"]
mod suppressed_external;

#[path = "auxiliary/line_comments.rs"]
mod external;
