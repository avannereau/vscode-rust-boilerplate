pub trait Named {
    fn say_name(&self) -> String;
}

pub struct Animal {
    pub name: String,
    pub is_dog: bool,
}

impl Named for Animal {
    fn say_name(&self) -> String {
        if self.is_dog {
            return format!("Woof: {}", self.name);
        }
        return format!("Meow: {}", self.name);
    }
}
