#![warn(prefer_named_exception_context)]

use std::io::Error;

fn wrapped(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| Error::other(error.to_string()))
}

fn block_body(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| {
        Error::new(std::io::ErrorKind::InvalidData, error.to_string())
    })
}

type AliasError = Error;

fn aliased(input: Result<(), std::fmt::Error>) -> Result<(), AliasError> {
    input.map_err(|cause| AliasError::other(cause.to_string()))
}

fn main() {}
