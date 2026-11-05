#![warn(no_unmatched_comments)]
#![allow(prefer_line_comments, reason = "verify block comment normalization")]

// WHY: Retries must stay ordered.
// eNg-482: Retry after the reset.
// Retry after the reset. reviewed
/*
 * Why: Preserve retry order.
 */
fn main() {}
