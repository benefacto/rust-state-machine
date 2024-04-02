use types::{AccountId, Balance, BlockNumber, Nonce};

mod balances;
mod system;
mod types;

#[derive(Debug)]
pub struct Runtime {
	system: system::Pallet<AccountId, BlockNumber, Nonce>,
	balances: balances::Pallet<AccountId, Balance>,
}

impl Runtime {
	fn new() -> Self {
		Self { system: system::Pallet::new(), balances: balances::Pallet::new() }
	}
}

fn main() {
	let mut runtime = Runtime::new();
	let alice = "alice".to_string();
	runtime.balances.set_balance(alice.to_owned(), 100);

	let block_number = runtime.system.block_number();
	let inc_block_number_result = runtime.system.inc_block_number();
	assert!(inc_block_number_result.is_ok());
	assert!(runtime.system.block_number() > block_number);

	let nonce = runtime.system.nonce(&alice);
	let inc_nonce_result = runtime.system.inc_nonce(alice.to_owned());
	assert!(inc_nonce_result.is_ok());
	assert!(runtime.system.nonce(&alice) > nonce);
	let transfer_result = runtime.balances.transfer(alice.to_owned(), "bob".to_owned(), 30);
	assert!(transfer_result.is_ok());

	let inc_nonce_result = runtime.system.inc_nonce(alice.to_owned());
	assert!(inc_nonce_result.is_ok());
	let transfer_result = runtime.balances.transfer(alice.to_owned(), "charlie".to_owned(), 30);
	assert!(transfer_result.is_ok());

	println!("{:#?}", runtime);
}
