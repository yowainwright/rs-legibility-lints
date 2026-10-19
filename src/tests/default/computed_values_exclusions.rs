fn one_step(subtotal: u32, tax: u32) -> u32 {
    subtotal + tax
}

fn named_steps(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    let taxed_subtotal = subtotal + tax;
    taxed_subtotal + shipping
}

fn initializer(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    let total = subtotal + tax + shipping;
    total
}

fn unary(value: i32) -> i32 {
    -value + 1
}

fn bits(value: u32, other: u32) -> u32 {
    value << 1 | other >> 1
}

fn separate_branches(should_add: bool, left: u32, right: u32) -> u32 {
    if should_add { left + right } else { left - right }
}

fn call_boundary(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    (subtotal + tax).wrapping_add(shipping) + 1
}

#[allow(no_computed_values, reason = "verify function-level suppression")]
fn suppressed(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    subtotal + tax + shipping
}

fn nested_suppression(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    #[allow(no_computed_values, reason = "verify block-level suppression")]
    {
        subtotal + tax + shipping
    }
}

macro_rules! computed {
    ($left:expr, $middle:expr, $right:expr) => {
        $left + $middle + $right
    };
}

fn generated(subtotal: u32, tax: u32, shipping: u32) -> u32 {
    computed!(subtotal, tax, shipping)
}

fn main() {}
