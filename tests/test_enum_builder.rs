mod more_animals;

use enum_builder::{enum_builder, enum_builder_variant};
use enum_dispatch::enum_dispatch;
use more_animals::chicken::*;
use more_animals::goat::*;

#[enum_dispatch(Animal)]
trait AnimalSound {
	fn speak(&self);
}

#[enum_builder]
#[enum_dispatch]
enum Animal {}

#[enum_builder_variant(Animal)]
struct Dog {}

impl AnimalSound for Dog {
	fn speak(&self) {
		println!("The dog goes woof!");
	}
}

#[enum_builder_variant(Animal)]
struct Cow {}

impl AnimalSound for Cow {
	fn speak(&self) {
		println!("The cow goes moo!");
	}
}

#[enum_builder_variant(Animal)]
struct Fish {}

impl AnimalSound for Fish {
	fn speak(&self) {
		println!("The fish goes blub blub!");
	}
}

#[enum_builder_variant(Animal)]
pub type Snake<'a> = ();

impl<'a> AnimalSound for Snake<'a> {
	fn speak(&self) {
		println!("The snake goes hiss!");
	}
}

#[test]
fn test_enum_builder() {
	let dog: Animal = Dog {}.into();
	let cow: Animal = Cow {}.into();
	let fish: Animal = Fish {}.into();
	let snake = ().into();
	let chicken: Animal = Chicken { count: &3 }.into();
	let goat: Animal = Goat(7).into();
	let farm = vec![dog, cow, fish, snake, chicken, goat];

	for animal in &farm {
		animal.speak();
	}
}
