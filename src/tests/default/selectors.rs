struct Root {
    user: User,
}

struct User {
    profile: Profile,
}

struct Profile {
    settings: Settings,
}

struct Settings {
    email: Email,
}

struct Email {
    enabled: bool,
}

fn at_limit(root: &Root) -> &Settings {
    &root.user.profile.settings
}

fn above_limit(root: &Root) -> &Email {
    &root.user.profile.settings.email
}

fn longer_chain(root: &Root) -> bool {
    root.user.profile.settings.email.enabled
}

fn named_steps(root: &Root) -> bool {
    let profile = &root.user.profile;
    let email = &profile.settings.email;
    email.enabled
}

fn array_at_limit(values: &[[[[u8; 1]; 1]; 1]; 1]) -> &[u8; 1] {
    &values[0][0][0]
}

fn array_above_limit(values: &[[[[u8; 1]; 1]; 1]; 1]) -> u8 {
    values[0][0][0][0]
}

struct Row {
    cells: [Cell; 1],
}

struct Cell {
    value: u8,
}

fn mixed_access(rows: &[Row]) -> u8 {
    rows[0].cells[0].value
}

fn tuple_access(value: &((((u8,),),),)) -> u8 {
    value.0.0.0.0
}

fn index_expression(values: &[u8], positions: &[[[[usize; 1]; 1]; 1]; 1]) -> u8 {
    values[positions[0][0][0][0]]
}

fn method_parent(root: &Root) -> Option<u8> {
    root.user.profile.settings.email.enabled.then_some(1)
}

fn assignment(rows: &mut [Row]) {
    rows[0].cells[0].value = 1;
}

fn through_call(root: &Root) -> &Email {
    &profile(&root.user.profile).settings.email
}

fn profile(value: &Profile) -> &Profile {
    value
}

macro_rules! generated {
    ($root:expr) => {
        $root.user.profile.settings.email.enabled
    };
}

fn from_macro(root: &Root) -> bool {
    generated!(root)
}

#[allow(no_deep_selector_chain, reason = "verify native lint suppression")]
fn suppressed(root: &Root) -> bool {
    root.user.profile.settings.email.enabled
}

fn main() {}
