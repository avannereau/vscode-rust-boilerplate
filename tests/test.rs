use boilerplate::Animal;
use boilerplate::Named;

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn dog_test() {
		let dog = Animal {
			name: String::from("Rex"),
			is_dog: true,
		};
		assert_eq!("Rex", dog.name);
		assert_eq!(true, dog.is_dog);
		assert_eq!("Woof: Rex", dog.say_name())
	}

	#[test]
	fn cat_test() {
		let cat = Animal {
			name: String::from("Garfield"),
			is_dog: false,
		};
		assert_eq!("Garfield", cat.name);
		assert_eq!(false, cat.is_dog);
		assert_eq!("Meow: Garfield", cat.say_name())
	}
}
