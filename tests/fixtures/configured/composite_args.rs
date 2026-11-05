fn consume<T>(_value: T) {}

fn at_limit() {
    consume([[1]]);
    consume(((1, 2), 3));
}

fn above_limit() {
    consume([[[1]]]);
}

fn main() {}
