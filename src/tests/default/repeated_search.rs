fn repeated(values: &[u32], needle: u32) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
    let third = values.contains(&needle);
}

fn vector(values: Vec<u32>, needle: u32) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn array(values: [u32; 2], needle: u32) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn borrowed(values: &[char], needle: &char) {
    let first = values.contains(needle);
    let second = values.contains(needle);
}

fn booleans(values: &[bool], needle: bool) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn floats(values: &[f64], needle: f64) {
    let first = values.contains(&needle);
    let second = values.contains(&needle);
}

fn nested(values: &[u32], needle: u32) {
    {
        let first = values.contains(&needle);
        let second = values.contains(&needle);
    }
}

fn closure(values: &[u32], needle: u32) {
    let check = || {
        let first = values.contains(&needle);
        let second = values.contains(&needle);
    };
    let _ = check;
}

fn rewritten(values: &[u32], needle: u32) {
    let first = values.contains(&needle);
    let second = first;
}

fn main() {}
