use std::collections::BTreeMap;

use num::{CheckedAdd, One, Zero};

pub trait Config {
	type AccountId: Ord + std::fmt::Debug + std::fmt::Display;
	type BlockNumber: Zero + One + CheckedAdd + Copy + std::fmt::Debug + std::fmt::Display;
	type Nonce: Copy + Zero + One + std::fmt::Debug + CheckedAdd + std::fmt::Display;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	block_number: T::BlockNumber,
	nonce: BTreeMap<T::AccountId, T::Nonce>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { block_number: Zero::zero(), nonce: BTreeMap::new() }
	}

	pub fn block_number(&self) -> T::BlockNumber {
		self.block_number
	}

	pub fn inc_block_number(&mut self) -> Result<(), String> {
		self.block_number = self.block_number.checked_add(&One::one()).ok_or_else(|| {
			format!("Block number {} will overflow, upgrade necessary", self.block_number)
		})?;
		Ok(())
	}

	pub fn nonce(&self, who: &T::AccountId) -> T::Nonce {
		*self.nonce.get(who).unwrap_or(&Zero::zero())
	}

	pub fn inc_nonce(&mut self, who: T::AccountId) -> Result<(), String> {
		let current_nonce = *self.nonce.get(&who).unwrap_or(&Zero::zero());
		let new_nonce = current_nonce.checked_add(&One::one()).ok_or_else(|| {
			format!("Nonce {} for {} will overflow, upgrade necessary", current_nonce, who)
		})?;

		self.nonce.insert(who, new_nonce);

		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl super::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}

	#[test]
	fn block_number() {
		let mut system = super::Pallet::<TestConfig>::new();
		let starting_block_number = system.block_number();
		assert_eq!(starting_block_number, 0);
		let inc_block_number_result = system.inc_block_number();
		assert!(inc_block_number_result.is_ok());
		let incremented_block_number = system.block_number();
		assert_eq!(incremented_block_number, starting_block_number + 1);
	}

	#[test]
	fn nonce() {
		let mut system = super::Pallet::<TestConfig>::new();
		let alice: &'static str = "alice";

		let starting_nonce = system.nonce(&alice);
		let inc_nonce_result = system.inc_nonce(alice);
		assert!(inc_nonce_result.is_ok());
		let incremented_nonce_number = system.nonce(&alice);
		assert_eq!(incremented_nonce_number, starting_nonce + 1);
	}
}