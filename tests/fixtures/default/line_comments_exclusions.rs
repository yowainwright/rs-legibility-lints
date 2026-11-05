//! Module documentation.
/** Item documentation. */
fn documented() {}

#[allow(prefer_line_comments, reason = "verify function-level suppression")]
fn suppressed() {
    /* Preserve this explanation. */
}

mod docs {
    /*! Inner block documentation. */
    /// Line documentation.
    pub struct Item;
}

fn main() {
    let _ = 1
        /* Keep the two operands together. */
        + 2;
    let _ = 1 /* Preserve the inline explanation. */ + 2;
    /* Before code on the same line. */ let _ = 1;
    let _ = 1; /* After code on the same line. */
    let _ = "/* A string is not a comment. */";
    let _ = r#"/* Neither is a raw string. */"#;
    // An ordinary line comment.
    #[allow(prefer_line_comments, reason = "verify block-level suppression")]
    {
        /* Preserve this explanation. */
    }
}
