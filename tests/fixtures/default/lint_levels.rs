fn save(_overwrite: bool) {}

#[allow(no_bool_literal_args, reason = "verify disabled lint level")]
fn ignored() {
    save(true);
}

#[warn(no_bool_literal_args)]
fn warning() {
    save(true);
}

#[deny(no_bool_literal_args)]
fn error() {
    save(false);
}

fn main() {}
