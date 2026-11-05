#[warn(no_mixed_filename_casing)]
#[path = "auxiliary/API_module.rs"]
mod warned;

#[deny(no_mixed_filename_casing)]
#[path = "auxiliary/API_module.rs"]
mod rejected;

#[expect(no_mixed_filename_casing)]
#[path = "auxiliary/API_module.rs"]
mod expected_first;

#[expect(no_mixed_filename_casing)]
#[path = "auxiliary/API_module.rs"]
mod expected_second;

fn main() {}
