use std::{collections::BTreeMap, ptr::eq};

pub struct Pallet {
    block_number: u32,
    nonce: BTreeMap<String, u32>,
}

impl Pallet {
    pub fn new() -> Self {
        Self { block_number: 0, nonce: BTreeMap::new() }
    }

    pub fn block_number(&self) -> u32 { 
        self.block_number
    }

    pub fn inc_block_number(&mut self) -> Result<(), String> {
        self.block_number = self.block_number.checked_add(1).ok_or_else(|| {
            format!("Block number {} will overflow, upgrade necessary", self.block_number)
        })?;
        Ok(())
    }

	pub fn nonce(&self, who: &str) -> u32 {
		*self.nonce.get(who).unwrap_or(&0)
    }

    pub fn inc_nonce(&mut self, who: String) -> Result<(), String> {
        let current_nonce = *self.nonce.get(&who).unwrap_or(&0);
        let new_nonce = current_nonce.checked_add(1).ok_or_else(|| {
			format!("Nonce {} for {} will overflow, upgrade necessary", current_nonce, who)
		})?;

        self.nonce.insert(who.to_string(), new_nonce);

        Ok(())
    }
}

#[test]
fn block_number() {
    let mut system = Pallet::new();
    let starting_block_number = system.block_number();
    assert_eq!(starting_block_number, 0);
    system.inc_block_number().unwrap();
    let incremented_block_number = system.block_number();
    assert_eq!(incremented_block_number, starting_block_number + 1);
}

#[test]
fn nonce() {
    let mut system = Pallet::new();
    let starting_nonce = system.nonce("alice");
    system.inc_nonce("alice".to_owned()).unwrap();
    let incremented_nonce_number = system.nonce("alice");
    assert_eq!(incremented_nonce_number, starting_nonce + 1);
}