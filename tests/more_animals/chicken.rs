use enum_builder::enum_builder_variant;

use crate::AnimalSound;

#[enum_builder_variant(Animal)]
pub struct Chicken<'a> {
	pub count: &'a usize,
}

impl<'a> AnimalSound for Chicken<'a> {
	fn speak(&self) {
		println!("The chicken goes{}", " cluck!".repeat(*self.count));
	}
}
