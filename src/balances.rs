use std::{collections::BTreeMap, u128};

pub struct Pallet {
    balances: BTreeMap<String, u128>,
}

impl Pallet {
    pub fn new() -> Self {
        Pallet {
            balances: BTreeMap::new(),
        }
    }
    pub fn set_balance(&mut self, account: &String, amount: u128) {
        self.balances.insert(account.clone(), amount);
    }

    pub fn get_balance(&self, account: &String) -> u128 {
        *self.balances.get(account).unwrap_or(&0)
    }

    /// Transfer amount from one account to another.
    /// This function verifies that `from` has at least `amount` balance
    /// and that no mathematical overflows accur.
    pub fn transfer(
        &mut self,
        caller: String,
        to: String,
        amount: u128,
    ) -> Result<(), &'static str> {
        let caller_balance = self.get_balance(&caller);
        let to_balance = self.get_balance(&to);

        let new_caller_balance = caller_balance
            .checked_sub(amount)
            .ok_or("Insuficient balance")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("OverFlow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);
        Ok(())
    }
}

#[test]

fn init_balances() {
    let mut balances = Pallet::new();
    assert_eq!(balances.get_balance(&"Matheus".to_string()), 0);

    balances.set_balance(&"Matheus".to_string(), 10000);
    assert_eq!(balances.get_balance(&"Matheus".to_string()), 10000);

    assert_eq!(balances.get_balance(&"Marcos".to_string()), 0);
}

#[test]

fn transfer_balance() {
    let mut balances = Pallet::new();

    assert_eq!(
        balances.transfer("vini".to_string(), "daniel".to_string(), 10),
        Err("Insuficient balance")
    );

    balances.set_balance(&"daniel".to_string(), 10);
    assert_eq!(
        balances.transfer("daniel".to_string(), "vini".to_string(), 3),
        Ok(())
    );

    assert_eq!(balances.get_balance(&"daniel".to_string()), 7);
    assert_eq!(balances.get_balance(&"vini".to_string()), 3);

    balances.set_balance(&"vini".to_string(), u128::MAX);

    assert_eq!(
        balances.transfer("daniel".to_string(), "vini".to_string(), 3),
        Err("OverFlow"))
}
