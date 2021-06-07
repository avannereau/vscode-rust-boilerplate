mod animal;

use animal::Named;

fn main() {
    println!("Hello animals!");
    let dog = animal::Animal {
        name: String::from("Rex"),
        is_dog: true,
    };
    println!("{}", dog.say_name());
    let cat = animal::Animal {
        name: String::from("Garfield"),
        is_dog: false,
    };
    println!("{}", cat.say_name());
}
