fn unrelated(is_blocked: bool, count: usize, status: &str) {
    let is_ready = !is_blocked;
    let has_items = count != 0;
    let can_send = status != "blocked";
    let is_valid = status == "valid";
}

fn destructured(status: &str) {
    let (is_ready, number) = (status != "ready", 1);
}

fn negative(status: &str) {
    #[expect(prefer_positive_condition_names, reason = "isolate negative name exemption")]
    let is_not_ready = status != "not_ready";
}

struct Status;

impl PartialEq<&str> for Status {
    fn eq(&self, _other: &&str) -> bool {
        false
    }
}

fn custom_equality(status: Status) {
    let is_ready = status != "ready";
}

struct Ready;

impl std::ops::Not for Ready {
    type Output = bool;

    fn not(self) -> bool {
        false
    }
}

fn custom_negation(ready: Ready) {
    let is_ready = !ready;
}

fn numbers(ready: u8) {
    let is_ready = !ready;
}

macro_rules! generated {
    ($status:expr) => {
        let is_ready = $status != "ready";
    };
}

fn macros(status: &str) {
    generated!(status);
    let is_ready = status != concat!("rea", "dy");
}

#[allow(no_boolean_parameter_name_drift, reason = "verify function-level lint suppression")]
fn suppressed(status: &str) {
    let is_ready = status != "ready";
}

fn locally_suppressed(status: &str) {
    #[allow(no_boolean_parameter_name_drift, reason = "verify binding-level lint suppression")]
    let is_ready = status != "ready";
}

fn main() {}
