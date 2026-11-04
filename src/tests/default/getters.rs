struct User {
    owner: String,
    count: u8,
}

impl User {
    fn get_owner(&self) -> &str {
        &self.owner
    }

    pub fn get_count(&self) -> u8 {
        self.count
    }

    fn get_owner_mut(&mut self) -> &mut String {
        &mut self.owner
    }

    fn get_count_return(&self) -> u8 {
        return self.count;
    }

    fn owner(&self) -> &str {
        &self.owner
    }

    fn get(&self, _index: usize) -> u8 {
        self.count
    }

    fn get_indexed(&self, _index: usize) -> u8 {
        self.count
    }

    fn get_count_computed(&self) -> u8 {
        self.count + 1
    }

    fn get_count_with_work(&self) -> u8 {
        let count = self.count;
        count
    }

    fn get_(&self) -> u8 {
        self.count
    }

    fn get_static() -> u8 {
        1
    }
}

trait Access {
    fn get_count(&self) -> u8;
}

impl Access for User {
    fn get_count(&self) -> u8 {
        self.count
    }
}

macro_rules! generated {
    () => {
        impl User {
            fn get_generated(&self) -> u8 {
                self.count
            }
        }
    };
}

generated!();

impl User {
    #[allow(no_getter_prefix, reason = "verify method-level lint suppression")]
    fn get_suppressed(&self) -> u8 {
        self.count
    }
}

fn main() {}
