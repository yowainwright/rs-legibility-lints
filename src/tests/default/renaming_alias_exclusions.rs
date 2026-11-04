fn show<T>(_value: T) {}

fn mutable_reference(user: &mut u32) {
    let user_data = user;
    show(user_data);
}

fn mutable_source(mut user: u32) {
    let user_data = user;
    show(user_data);
}

fn mutable_alias(user: u32) {
    let mut user_data = user;
    show(user_data);
}

fn annotated(user: u32) {
    let user_data: u32 = user;
    show(user_data);
}

fn borrowing(user: u32) {
    let user_data = &user;
    show(user_data);
}

fn borrowed_use(user: u32) {
    let user_data = user;
    show(&user_data);
}

fn multiple_uses(user: u32) {
    let user_data = user;
    show(user_data);
    show(user_data);
}

fn later_use(user: u32) {
    let user_data = user;
    show(user_data);
    let later = || user_data;
    show(later);
}

fn intervening(user: u32) {
    let user_data = user;
    let number = 1;
    show(user_data);
}

fn shadowing(user: u32) {
    let user = user;
    show(user);
}

fn computed(user: u32) {
    let user_data = user + 1;
    show(user_data);
}

fn callback(user: u32, callback: impl Fn(u32)) {
    let user_data = user;
    callback(user_data);
}

fn multiple_arguments(user: u32) {
    fn pair(_first: u32, _second: u32) {}
    let user_data = user;
    pair(user_data, user);
}

macro_rules! generated {
    ($user:expr) => {
        let user_data = $user;
        show(user_data);
    };
}

fn macros(user: u32) {
    generated!(user);
}

#[allow(no_single_use_renaming_alias, reason = "verify function-level lint suppression")]
fn suppressed(user: u32) {
    let user_data = user;
    show(user_data);
}

fn locally_suppressed(user: u32) {
    #[allow(no_single_use_renaming_alias, reason = "verify binding-level lint suppression")]
    let user_data = user;
    show(user_data);
}

fn main() {}
