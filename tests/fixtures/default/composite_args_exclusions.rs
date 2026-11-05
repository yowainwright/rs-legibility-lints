struct Http {
    timeout: u8,
}

struct Config {
    http: Http,
}

struct Wrapper<T>(T);

fn consume<T>(_value: T) {}

fn named() {
    let http = Http { timeout: 1 };
    consume(Config { http });
    let inner = [1];
    consume([inner]);
    let nested = [[1]];
    consume(nested);
}

fn construct() -> [[u8; 1]; 1] {
    [[1]]
}

fn boundaries() {
    consume(construct());
    consume({ [[1]] });
    consume(|| [[1]]);
    consume(Wrapper([1]));
    consume(Some([1]));
    consume(&[1]);
}

fn standalone() {
    let _ = Config { http: Http { timeout: 1 } };
    let _ = [[[1]]];
}

trait Check {
    fn check(self);
}

impl Check for [[u8; 1]; 1] {
    fn check(self) {}
}

fn receiver() {
    [[1]].check();
}

macro_rules! generated_call {
    ($value:expr) => {
        consume($value)
    };
}

macro_rules! generated_literal {
    () => { [[1]] };
}

fn macros() {
    generated_call!([[1]]);
    consume(generated_literal!());
    consume([generated_literal!()]);
}

#[allow(no_deep_composite_literal_arg, reason = "verify function-level lint suppression")]
fn suppressed() {
    consume([[1]]);
}

fn locally_suppressed() {
    #[allow(no_deep_composite_literal_arg, reason = "verify expression-level lint suppression")]
    consume([[1]]);
}

fn main() {}
