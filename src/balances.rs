use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Pallet {
	balances: BTreeMap<String, u128>,
}

impl Pallet {
	pub fn new() -> Self {
		Self { balances: BTreeMap::new() }
	}

	pub fn set_balance(&mut self, who: String, amount: u128) -> () {
		self.balances.insert(who, amount);
	}

	pub fn balance(&self, who: &str) -> u128 {
		*self.balances.get(who).unwrap_or(&0)
	}

	pub fn transfer(&mut self, caller: String, to: String, amount: u128) -> Result<(), String> {
		let caller_balance = self.balance(&caller);
		let to_balance = self.balance(&to);

		let new_caller_balance = caller_balance.checked_sub(amount).ok_or_else(|| {
			format!("Insufficient balance of {} to transfer {} to {}", caller, amount, to)
		})?;
		let new_to_balance = to_balance.checked_add(amount).ok_or_else(|| {
			format!("Overflow of balance of {} while transferring {} from {}", to, amount, caller)
		})?;

		self.set_balance(caller.clone(), new_caller_balance);
		self.set_balance(to.clone(), new_to_balance);

		Ok(())
	}
}

#[test]
fn init_balances() {
	let mut balances = Pallet::new();

	assert_eq!(balances.balance("alice"), 0);
	let _ = balances.set_balance("alice".to_owned(), 100);
	assert_eq!(balances.balance("alice"), 100);
	assert_eq!(balances.balance("bob"), 0);
}

#[test]
fn transfer_balance() {
	let mut balances = Pallet::new();

	let initial_transfer_result = balances.transfer("alice".to_owned(), "bob".to_owned(), 100);
	assert!(
		initial_transfer_result.is_err(),
		"Expected Err from initial transfer due to insufficient funds, got: {:?}",
		initial_transfer_result
	);
	balances.set_balance("alice".to_owned(), 100);
	let transfer_result = balances.transfer("alice".to_owned(), "bob".to_owned(), 100);
	assert!(
		transfer_result.is_ok(),
		"Expected OK from transfer after setting balance, got: {:?}",
		transfer_result
	);
	let alice_balance = balances.balance("alice");
	assert_eq!(
		alice_balance, 0,
		"Expected 'alice' balance to be 0 after transfer, found: {}",
		alice_balance
	);
	let bob_balance = balances.balance("bob");
	assert_eq!(
		bob_balance, 100,
		"Expected 'bob' balance to be 100 after receiving transfer, found: {}",
		bob_balance
	);
}
