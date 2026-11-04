struct Http {
    timeout: u8,
}

struct Config {
    http: Http,
}

enum Choice {
    Config { http: Http },
}

struct Writer;

impl Writer {
    fn save<T>(&self, _value: T) {}
}

fn consume<T>(_value: T) {}

fn pair<A, B>(_left: A, _right: B) {}

fn at_limit() {
    consume(Http { timeout: 1 });
    consume([1, 2]);
    consume((1, 2));
    consume([1; 2]);
}

fn structs() {
    consume(Config { http: Http { timeout: 1 } });
    consume(Choice::Config { http: Http { timeout: 1 } });
}

fn collections() {
    consume([[1]]);
    consume(((1, 2), 3));
    consume([[1; 2]; 2]);
    consume((Http { timeout: 1 }, 2));
}

fn mixed() {
    consume([Config { http: Http { timeout: 1 } }]);
}

fn methods() {
    Writer.save(Config { http: Http { timeout: 1 } });
}

fn references() {
    consume(&[&[1]]);
}

fn multiple() {
    pair([[1]], ((1, 2), 3));
}

fn constructors() {
    let _ = Some([[1]]);
}

fn updates() {
    consume(Http { ..Http { timeout: 1 } });
}

fn main() {}
