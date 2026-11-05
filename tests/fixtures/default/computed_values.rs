fn total(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    return subtotal + tax + shipping;
}

fn tail(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    subtotal + tax + shipping
}

fn returned_block(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    return { subtotal + tax + shipping };
}

fn product(left: u32, middle: u32, right: u32) -> u32 {
    left * (middle - right)
}

fn quotient(left: u32, middle: u32, right: u32) -> u32 {
    left / (middle % right)
}

fn branch(should_add: bool, subtotal: u32, tax: u32, shipping: u32) -> u32 {
    if should_add {
        subtotal + tax + shipping
    } else {
        subtotal
    }
}

fn matched(mode: u32, subtotal: u32, tax: u32, shipping: u32) -> u32 {
    match mode {
        0 => subtotal + tax + shipping,
        _ => subtotal,
    }
}

fn casted(subtotal: u16, tax: u16, shipping: u32) -> u32 {
    (subtotal + tax) as u32 + shipping
}

fn closure(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    let calculate = || subtotal + tax + shipping;
    calculate()
}

async fn future(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    subtotal + tax + shipping
}

fn main() {}
