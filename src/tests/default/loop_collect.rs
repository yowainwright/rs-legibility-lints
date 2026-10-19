#![warn(no_loop_append_comprehension)]

fn doubled(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * 2);
    }
    output
}

fn explicit(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * 2);
    }
    return output;
}

fn array(values: [i32; 3]) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value + 1);
    }
    output
}

fn borrowed(values: &[i32]) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(*value * 2);
    }
    output
}

type Output = Vec<i32>;

fn alias(values: Vec<i32>) -> Output {
    let mut output: Output = Output::new();
    for value in values {
        output.push(value * 2)
    }
    output
}

fn main() {}
