fn save(_overwrite: bool) {}

fn options(_cache: bool, _retry: bool) {}

struct Flag(bool);

struct Writer;

impl Writer {
    fn new(_enabled: bool) -> Self {
        Self
    }

    fn save(&self, _overwrite: bool) {}
}

fn direct_calls() {
    save(true);
    save(false);
    options(true, false);
}

fn method_calls() {
    let writer = Writer::new(true);
    writer.save(false);
    Writer::save(&writer, true);
}

fn constructors() {
    let _ = Flag(true);
    let _ = Some(false);
}

fn indirect(function: fn(bool), closure: impl Fn(bool)) {
    function(true);
    closure(false);
}

fn parenthesized() {
    save((true));
}

fn main() {}
