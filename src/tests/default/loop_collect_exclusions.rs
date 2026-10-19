#![warn(no_loop_append_comprehension)]

fn branching(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        if value > 0 {
            output.push(value * 2);
        }
    }
    output
}

fn early_exit(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        if value == 0 {
            break;
        }
        output.push(value * 2);
    }
    output
}

fn opaque(value: i32) -> i32 {
    value
}

fn calls(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(opaque(value));
    }
    output
}

fn extra_work(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        let doubled = value * 2;
        output.push(doubled);
    }
    output
}

fn preallocated(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::with_capacity(values.len());
    for value in values {
        output.push(value * 2);
    }
    output
}

fn populated(values: Vec<i32>) -> Vec<i32> {
    let mut output = vec![0];
    for value in values {
        output.push(value * 2);
    }
    output
}

fn later_use(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * 2);
    }
    output.push(0);
    output
}

fn captured(values: Vec<i32>, factor: i32) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * factor);
    }
    output
}

fn nonprimitive(values: Vec<String>) -> Vec<String> {
    let mut output = Vec::new();
    for value in values {
        output.push(value);
    }
    output
}

fn other_iterator(values: std::ops::Range<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * 2);
    }
    output
}

mod custom_collection {
    pub struct Vec;

    impl Vec {
        fn new() -> Self {
            Self
        }

        fn push(&mut self, _: i32) {}
    }

    pub fn build(values: std::vec::Vec<i32>) -> Vec {
        let mut output = Vec::new();
        for value in values {
            output.push(value * 2);
        }
        output
    }
}

macro_rules! generated {
    ($output:ident, $values:expr) => {
        for value in $values {
            $output.push(value * 2);
        }
    };
}

fn expanded(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    generated!(output, values);
    output
}

macro_rules! borrowed_input {
    ($values:expr) => { &$values };
}

fn expanded_input(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in borrowed_input!(values) {
        output.push(*value * 2);
    }
    output
}

macro_rules! vector_constructor {
    () => { Vec::new };
}

fn expanded_constructor(values: Vec<i32>) -> Vec<i32> {
    let mut output = vector_constructor!()();
    for value in values {
        output.push(value * 2);
    }
    output
}

#[allow(no_loop_append_comprehension, reason = "verify native lint suppression")]
fn suppressed(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * 2);
    }
    output
}

fn accepted(values: Vec<i32>) -> Vec<i32> {
    values.into_iter().map(|value| value * 2).collect()
}

fn locally_suppressed(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    #[allow(no_loop_append_comprehension, reason = "verify loop-level suppression")]
    for value in values {
        output.push(value * 2);
    }
    output
}

fn main() {}
