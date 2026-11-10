pub struct Cat {
    age: u8,
    name: String,
}

impl Cat {
    pub fn new() -> Self {
        Self {
            age: 32,
            name: String::from("garfield"),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
    pub fn get_age(&self) -> u8 {
        self.age
    }
}
