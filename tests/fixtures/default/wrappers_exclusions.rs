#![allow(unconditional_recursion, reason = "verify recursive calls do not match LEG008")]

pub fn public(value: u8) -> u8 {
    target(value)
}

pub(crate) fn crate_visible(value: u8) -> u8 {
    target(value)
}

mod nested {
    pub(super) fn parent_visible(value: u8) -> u8 {
        super::target(value)
    }
}

trait Convert {
    fn required(&self, value: u8) -> u8;

    fn provided(&self, value: u8) -> u8 {
        self.required(value)
    }
}

struct State;

impl Convert for State {
    fn required(&self, value: u8) -> u8 {
        target(value)
    }
}

impl State {
    pub fn public_method(&self, value: u8) -> u8 {
        self.required(value)
    }
}

fn changed(value: u8) -> u8 {
    target(value + 1)
}

fn reordered(left: u8, right: u8) -> u8 {
    pair(right, left)
}

fn duplicated(left: u8, right: u8) -> u8 {
    pair(left, left)
}

fn added(value: u8) -> u8 {
    pair(value, 1)
}

fn discarded(value: u8) {
    target(value);
}

fn with_statement(value: u8) -> u8 {
    let adjusted = value.wrapping_add(1);
    target(adjusted)
}

fn destructured((value, _): (u8, u8)) -> u8 {
    target(value)
}

fn reference(ref value: u8) -> u8 {
    target(*value)
}

fn recursive(value: u8) -> u8 {
    recursive(value)
}

impl State {
    fn recursive_method(&self, value: u8) -> u8 {
        self.recursive_method(value)
    }
}

fn callback(function: fn(u8) -> u8, value: u8) -> u8 {
    function(value)
}

fn closure() -> impl Fn(u8) -> u8 {
    |value| target(value)
}

fn no_parameters() -> u8 {
    zero_target()
}

fn constructor(value: u8) -> Wrapped {
    Wrapped(value)
}

struct Wrapped(u8);

const fn constant(value: u8) -> u8 {
    constant_target(value)
}

const fn constant_target(value: u8) -> u8 {
    value + 1
}

async fn asynchronous(value: u8) -> u8 {
    target(value)
}

unsafe fn unsafe_boundary(value: u8) -> u8 {
    target(value)
}

extern "C" fn abi_boundary(value: u8) -> u8 {
    target(value)
}

#[unsafe(no_mangle)]
fn exported(value: u8) -> u8 {
    target(value)
}

fn coerced(value: &String) -> &str {
    string_identity(value)
}

fn string_identity(value: &String) -> &String {
    value
}

#[allow(no_trivial_wrapper_functions, reason = "verify native lint suppression")]
fn suppressed(value: u8) -> u8 {
    target(value)
}

macro_rules! generated {
    () => {
        fn generated_wrapper(value: u8) -> u8 {
            target(value)
        }
    };
}

generated!();

macro_rules! forward {
    ($value:expr) => {
        target($value)
    };
}

fn generated_call(value: u8) -> u8 {
    forward!(value)
}

fn target(value: u8) -> u8 {
    value.wrapping_add(1)
}

fn zero_target() -> u8 {
    0
}

pub fn pair(left: u8, right: u8) -> u8 {
    left.wrapping_add(right)
}

fn consumes_vector(values: Vec<u8>) -> usize {
    values.len()
}

fn consumes_string(value: String) -> bool {
    value.is_empty()
}

fn unsizes_argument(values: &[u8; 2]) -> usize {
    slice_size(values)
}

pub fn slice_size(values: &[u8]) -> usize {
    values.len()
}

fn main() {}
