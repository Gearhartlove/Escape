pub trait Item {
    fn name(&self) -> &str;
    fn get_description(&self) -> &str;
    fn use_item(&self, verb: String);
    fn message(&self) -> String;
}

impl Item for Sword {
    fn name(&self) -> &str {
        self.name
    }
    fn get_description(&self) -> &str {
        self.description
    }
    fn use_item(&self, verb: String) {
        println!("The sword has been {} and you hit for {}", verb, self.power);
    }
    fn message(&self) -> String {
        format!(
            "item: {} description: {} power: {}",
            self.name(),
            self.get_description(),
            self.power()
    }
}

pub struct Sword {
    pub name: &'static str,
    pub description: &'static str,
    pub power: i32,
}

impl Sword {
    fn power(&self) -> i32 {
        self.power
    }
}
