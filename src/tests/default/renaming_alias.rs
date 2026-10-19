fn show<T>(_value: T) {}

fn integer(user: u32) {
    let user_data = user;
    show(user_data);
}

fn owned(user: String) {
    let user_data = user;
    show(user_data);
}

fn tail(user: u32) {
    let user_data = user;
    show(user_data)
}

fn shared(user: &str) {
    let user_data = user;
    show(user_data);
}

#[derive(Clone, Copy)]
struct User(u32);

fn copy_struct(user: User) {
    let user_data = user;
    show(user_data);
}

fn generic<T: Copy>(user: T) {
    let user_data = user;
    show(user_data);
}

fn nested(user: u32) {
    {
        let user_data = user;
        show(user_data);
    }
}

fn closure(user: u32) {
    let check = || {
        let user_data = user;
        show(user_data);
    };
    check();
}

fn main() {}
