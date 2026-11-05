#![warn(prefer_named_exception_context)]

use std::io::{Error, ErrorKind};

fn preserves_source(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| Error::other(error))
}

fn named_context(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| {
        let message = format!("loading profile: {error}");
        Error::other(message)
    })
}

fn inline_context(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| Error::other(format!("loading profile: {error}")))
}

fn typed_error(input: Result<(), Error>) -> Result<(), ProfileError> {
    input.map_err(ProfileError)
}

struct ProfileError(Error);

fn captured_error(input: Result<(), Error>, other: Error) -> Result<(), Error> {
    input.map_err(|_error| Error::other(other.to_string()))
}

fn success_mapping(input: Result<Error, ()>) -> Result<Error, ()> {
    input.map(|error| Error::other(error.to_string()))
}

fn standalone(error: Error) -> Error {
    Error::other(error.to_string())
}

fn multiple_statements(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| {
        std::hint::black_box(&error);
        Error::other(error.to_string())
    })
}

struct CustomResult(Result<(), Error>);

impl CustomResult {
    fn map_err(self, wrap: impl FnOnce(Error) -> Error) -> Result<(), Error> {
        self.0.map_err(wrap)
    }
}

fn custom_mapping(input: CustomResult) -> Result<(), Error> {
    input.map_err(|error| Error::other(error.to_string()))
}

struct Contextual;

impl Contextual {
    fn to_string(&self) -> String {
        String::from("loading profile")
    }
}

fn custom_conversion(input: Result<(), Contextual>) -> Result<(), Error> {
    input.map_err(|error| Error::other(error.to_string()))
}

mod custom {
    pub struct Error;

    impl Error {
        pub fn new(_kind: std::io::ErrorKind, _message: String) -> Self { Self }
        pub fn other(_message: String) -> Self { Self }
    }
}

fn custom_constructor(input: Result<(), Error>) -> Result<(), custom::Error> {
    input.map_err(|error| custom::Error::other(error.to_string()))
}

fn custom_new(input: Result<(), Error>) -> Result<(), custom::Error> {
    input.map_err(|error| custom::Error::new(ErrorKind::Other, error.to_string()))
}

macro_rules! pass {
    ($value:expr) => { $value };
}

fn macro_receiver(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| Error::other(pass!(error).to_string()))
}

fn macro_closure(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(pass!(|error: Error| Error::other(error.to_string())))
}

#[allow(prefer_named_exception_context)]
fn suppressed(input: Result<(), Error>) -> Result<(), Error> {
    input.map_err(|error| Error::other(error.to_string()))
}

fn main() {}
