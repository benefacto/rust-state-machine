use std::collections::BTreeMap;

use num::{CheckedAdd, CheckedSub, Zero};

pub trait Config: crate::system::Config {
	type Balance: Zero + CheckedSub + CheckedAdd + Copy + std::fmt::Debug;
}

#[derive(Debug)]
pub struct Pallet<T: Config> {
	balances: BTreeMap<T::AccountId, T::Balance>,
}

impl<T: Config> Pallet<T> {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: T::AccountId, amount: T::Balance) -> () {
		self.balances.insert(who, amount);
	}

	pub fn balance(&self, who: &T::AccountId) -> T::Balance {
		self.balances.get(who).copied().unwrap_or_else(Zero::zero)
	}

	pub fn transfer(
		&mut self,
		caller: T::AccountId,
		to: T::AccountId,
		amount: T::Balance,
	) -> Result<(), String> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(&amount).ok_or_else(|| {
			format!("Insufficient balance of {:?} to transfer {:?} to {:?}", caller, amount, to)
		})?;
		let new_to_balance = to_balance.checked_add(&amount).ok_or_else(|| {
			format!(
				"Overflow of balance of {:?} while transferring {:?} from {:?}",
				to, amount, caller
			)
		})?;

		self.set_balance(caller, new_caller_balance);
		self.set_balance(to, new_to_balance);

		Ok(())
	}
}

#[cfg(test)]
mod test {
	struct TestConfig;
	impl crate::system::Config for TestConfig {
		type AccountId = &'static str;
		type BlockNumber = u32;
		type Nonce = u32;
	}
	impl super::Config for TestConfig {
		type Balance = u128;
	}
	#[test]
	fn init_balances() {
		let mut balances = super::Pallet::<TestConfig>::new();
		let alice: &'static str = "alice";
		let bob: &'static str = "bob";

		assert_eq!(balances.balance(&alice), 0);
		let _ = balances.set_balance(alice, 100);
		assert_eq!(balances.balance(&alice), 100);
		assert_eq!(balances.balance(&bob), 0);
	}

	#[test]
	fn transfer_balance() {
		let mut balances = super::Pallet::<TestConfig>::new();
		let alice: &'static str = "alice";
		let bob: &'static str = "bob";
		let amount = 100;

		let initial_transfer_result = balances.transfer(alice, bob, amount);
		assert!(
			initial_transfer_result.is_err(),
			"Expected Err from initial transfer due to insufficient funds, got: {:?}",
			initial_transfer_result
		);
		balances.set_balance(alice, amount);
		let transfer_result = balances.transfer(alice, bob, amount);
		assert!(
			transfer_result.is_ok(),
			"Expected OK from transfer after setting balance, got: {:?}",
			transfer_result
		);
		let alice_balance = balances.balance(&alice);
		assert_eq!(
			alice_balance, 0,
			"Expected 'alice' balance to be 0 after transfer, found: {}",
			alice_balance
		);
		let bob_balance = balances.balance(&bob);
		assert_eq!(
			bob_balance, amount,
			"Expected 'bob' balance to be 100 after receiving transfer, found: {}",
			bob_balance
		);
	}
}
