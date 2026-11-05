fn inequality(status: &str) {
    let is_ready = status != "ready";
    let _ = is_ready;
}

fn matching(status: &str) {
    let is_ready = status == "ready";
    let _ = is_ready;
}

fn unrelated(status: &str) {
    let is_ready = status != "blocked";
    let _ = is_ready;
}

fn negation(ready: bool) {
    let is_ready = !ready;
    let _ = is_ready;
}

fn reversed(status: &str) {
    let is_ready = "ready" != status;
    let _ = is_ready;
}

fn owned(status: String) {
    let is_ready = status != "ready";
    let _ = is_ready;
}

fn annotated(status: &str) {
    type Flag = bool;
    let is_ready: Flag = status != "READY";
    let _ = is_ready;
}

fn borrowed(ready: &bool) {
    let is_ready = !ready;
    let _ = is_ready;
}

fn compound_subject(status: &str) {
    let has_read_access = status != r#"read_access"#;
    let _ = has_read_access;
}

fn main() {}
