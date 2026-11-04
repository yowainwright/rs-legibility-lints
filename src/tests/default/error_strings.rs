use std::io::{Error, ErrorKind};
use std::io::Error as IoError;

type AliasError = Error;

fn messages() {
    let _ = Error::other("Invalid user.");
    let _ = Error::other("invalid user.");
    let _ = Error::other("invalid user!");
    let _ = Error::other("invalid user:");
    let _ = Error::new(ErrorKind::InvalidInput, "Invalid request");
    let _ = IoError::other("Bad request");
    let _ = AliasError::new(ErrorKind::NotFound, r#"Missing user."#);
    let _ = Error::other("\u{00c9}chec");
    let _ = Error::other("HTTP request.");
    let _ = Error::other("TLSConfig invalid.");
    let _ = std::io::Error::other(" Invalid user");
}

fn main() {}
