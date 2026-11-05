#![warn(prefer_verb_function_names)]

fn data_fn() {}

fn config_load() {}

fn settings_reset() {}

fn data_éléments() {}

fn load_data() {}

fn database() {}

fn dataéléments() {}

fn data() -> u8 {
    1
}

struct Reader {
    count: u8,
}

impl Reader {
    fn data_load(&self) -> u8 {
        self.count.wrapping_add(1)
    }

    fn data_count(&self) -> u8 {
        self.count
    }

    fn config_path(&self) -> &u8 {
        &self.count
    }

    fn value(&self) -> u8 {
        self.count
    }
}

trait Required {
    fn data_read(&self);
    fn config_default(&self) {}
}

impl Required for Reader {
    fn data_read(&self) {}
}

macro_rules! generated {
    () => {
        fn object_load() {}
    };
}

generated!();

extern "C" fn result_export() {}

#[allow(prefer_verb_function_names, reason = "verify function-level lint suppression")]
fn object_skip() {}

#[unsafe(export_name = "legacy_settings")]
fn settings_export() {}

fn main() {}
