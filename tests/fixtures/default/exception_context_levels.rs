#[deny(prefer_named_exception_context)]
fn rejected(input: Result<(), std::io::Error>) -> Result<(), std::io::Error> {
    input.map_err(|error| std::io::Error::other(error.to_string()))
}

fn default_level(input: Result<(), std::io::Error>) -> Result<(), std::io::Error> {
    input.map_err(|error| std::io::Error::other(error.to_string()))
}

#[warn(prefer_named_exception_context)]
fn locally_allowed(input: Result<(), std::io::Error>) -> Result<(), std::io::Error> {
    #[allow(prefer_named_exception_context)]
    let wrapped = input.map_err(|error| std::io::Error::other(error.to_string()));
    wrapped
}

fn main() {}
