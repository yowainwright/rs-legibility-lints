const SHOULD_OVERWRITE: bool = true;

fn save(_overwrite: bool) {}

fn borrowed(_enabled: &bool) {}

fn values(should_overwrite: bool) {
    save(should_overwrite);
    save(SHOULD_OVERWRITE);
    save(!should_overwrite);
    let _ = (true, false);
    let _ = [true, false];
    let _ = Some(1);
    let _ = Some("true");
    borrowed(&true);
    save({ true });
}

struct Settings {
    enabled: bool,
}

trait Flag {
    fn check(self);
}

impl Flag for bool {
    fn check(self) {}
}

fn receivers() {
    true.check();
    let _ = Settings { enabled: false };
}

macro_rules! generated_call {
    ($value:expr) => {
        save($value)
    };
}

macro_rules! generated_literal {
    () => { false };
}

fn macros() {
    generated_call!(true);
    save(generated_literal!());
    assert!(true);
}

#[allow(no_bool_literal_args, reason = "verify function-level lint suppression")]
fn suppressed() {
    save(true);
}

fn locally_suppressed() {
    #[allow(no_bool_literal_args, reason = "verify expression-level lint suppression")]
    save(false);
}

macro_rules! pass {
    ($value:expr) => { $value };
}

fn forwarded_macro_argument() {
    save(pass!(true));
}

fn main() {}
