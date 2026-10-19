#![warn(no_stacked_comments)]

// Retry failed requests.
// Preserve their order.
// Avoid duplicate retries.

fn inside() {
    // Explain the retry.
    // Explain its order.
}

#[allow(no_stacked_comments, reason = "verify an inner warning override")]
fn overridden() {
    #[warn(no_stacked_comments)]
    {
        // Explain the retry.
        // Explain its order.
    }
}

#[allow(no_stacked_comments, reason = "verify external module suppression")]
#[path = "auxiliary/stacked_comments.rs"]
mod suppressed_external;

#[path = "auxiliary/stacked_comments.rs"]
mod external;

fn main() {}
