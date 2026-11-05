mod base {}
mod common {}
mod helper {}
mod helpers {}
mod lib {}
mod libs {}
mod misc {}
mod shared {}
mod util {}
mod utils {}

mod retry {}
mod utilities {}
mod common_api {}
mod nested {
    mod utils {}
}

macro_rules! generated {
    () => {
        mod util {}
    };
}

mod generated_module {
    generated!();
}

#[allow(no_generic_package_names, reason = "verify module-level lint suppression")]
mod suppressed {
    mod utils {}
}

#[cfg(any())]
mod helper {}

fn main() {}
