struct Builder;

impl Builder {
    fn filter(self) -> Self {
        self
    }

    fn map(self) -> Self {
        self
    }

    fn collect(self) -> Self {
        self
    }
}

fn builder() -> Builder {
    Builder.filter().map().collect()
}

fn strings(value: &str) -> String {
    value.trim().to_owned().trim().to_owned()
}

fn separate_arguments(values: &[i32]) -> usize {
    (0..4).zip(values.iter().map(|value| *value + 1)).count()
}

macro_rules! generated {
    ($values:expr) => {
        $values.iter().filter(|value| **value > 0).map(|value| *value + 1).count()
    };
}

fn from_macro(values: &[i32]) -> usize {
    generated!(values)
}

#[allow(max_array_chain_depth, reason = "verify native lint suppression")]
fn suppressed(values: &[i32]) -> usize {
    values.iter().filter(|value| **value > 0).map(|value| *value + 1).count()
}

fn main() {}
