use enum_builder::enum_builder_variant;

use crate::AnimalSound;

#[enum_builder_variant(Animal)]
pub struct Goat(pub usize);

impl<'a> AnimalSound for Goat {
	fn speak(&self) {
		println!("The goat goes{}", " bleat!".repeat(self.0));
	}
}
