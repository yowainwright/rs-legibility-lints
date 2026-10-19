#[deny(no_loop_append_comprehension)]
fn rejected(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * 2);
    }
    output
}

fn default_level(values: Vec<i32>) -> Vec<i32> {
    let mut output = Vec::new();
    for value in values {
        output.push(value * 2);
    }
    output
}

fn main() {}
