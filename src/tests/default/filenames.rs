#[allow(no_mixed_filename_casing, reason = "suppression must not hide another module's warning")]
#[path = "auxiliary/API_module.rs"]
mod suppressed_first;

#[path = "auxiliary/API_module.rs"]
mod first;

#[path = "auxiliary/API_module.rs"]
mod second;

#[path = "auxiliary/api-client.rs"]
mod hyphenated;

#[path = "auxiliary/UpperDir/api_client.rs"]
mod lowercase;

#[path = "auxiliary/Suppressed_module.rs"]
mod file_suppressed;

#[cfg(any())]
#[path = "auxiliary/Unused_module.rs"]
mod unused;

#[path = "auxiliary/Empty_module.rs"]
mod empty;

fn main() {}
