use std::io::{Error, ErrorKind};

fn accepted() {
    let _ = Error::other("invalid user");
    let _ = Error::other("HTTP failure");
    let _ = Error::other("TLSConfig invalid");
    let _ = Error::other("HTTP");
    let _ = Error::other("X509");
    let _ = Error::other("I/O failure");
    let _ = Error::other("x509 failed");
    let _ = Error::other("");
    let _ = Error::other("123 rejected");
    let _ = Error::other("   ");
}

fn dynamic() {
    let message = "Bad request.";
    let _ = Error::other(message);
    let _ = Error::other(format!("Bad request: {}.", 1));
    let _ = Error::new(ErrorKind::Other, String::from("Bad request."));
    let _ = Err::<(), _>("Bad request.");
}

mod custom {
    pub struct Error;

    impl Error {
        pub fn new(_kind: std::io::ErrorKind, _message: &str) -> Self {
            Self
        }

        pub fn other(_message: &str) -> Self {
            Self
        }
    }
}

fn unrelated() {
    use custom::Error;
    let _ = Error::other("Bad request.");
    let _ = Error::new(ErrorKind::Other, "Bad request.");
}

fn indirect() {
    let make_error = Error::other;
    let _ = make_error("Bad request.");
}

macro_rules! generated_call {
    ($message:expr) => {
        Error::other($message)
    };
}

fn macros() {
    let _ = generated_call!("Bad request.");
    let _ = Error::other(concat!("Bad", " request."));
}

#[allow(prefer_lowercase_error_strings, reason = "verify function-level lint suppression")]
fn suppressed() {
    let _ = Error::other("Bad request.");
}

fn locally_suppressed() {
    #[allow(prefer_lowercase_error_strings, reason = "verify expression-level lint suppression")]
    let _ = Error::other("Bad request.");
}

fn main() {}
