use std::collections::BTreeMap;

type AccountID = String;
type Balance = u128;

#[derive(Debug)]
pub struct Pallet {
    balances: BTreeMap<AccountID, Balance>,
}

impl Pallet {
    pub fn new() -> Self {
        Self {
            balances: BTreeMap::new()
        }
    }

    pub fn set_balance(&mut self, who: &AccountID, amount: Balance) {
        self.balances.insert(who.clone(), amount);
    }

    pub fn balance(&self, who: &AccountID) -> Balance {
        *self.balances.get(who).unwrap_or(&0)
    }

    pub fn transfer(
        &mut self,
        caller: AccountID,
        to: AccountID,
        amount: Balance,
    ) -> Result<(), &'static str> {
        let caller_balance = self.balance(&caller);
        let to_balance = self.balance(&to);

        let new_caller_balance = caller_balance.checked_sub(amount).ok_or("Not enough fund")?;
        let new_to_balance = to_balance.checked_add(amount).ok_or("overflow")?;

        self.balances.insert(caller, new_caller_balance);
        self.balances.insert(to, new_to_balance);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn init_balances() {
        let mut balances = super::Pallet::new();
        assert_eq!(balances.balance(&"alice".to_string()), 0);
        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.balance(&"alice".to_string()), 100);
        assert_eq!(balances.balance(&"bob".to_string()), 0);
    }

    #[test]
    fn transfer_balance() {
        /* TODO: Create a test that checks the following:
            - That `alice` cannot transfer funds she does not have.
            - That `alice` can successfully transfer funds to `bob`.
            - That the balance of `alice` and `bob` is correctly updated.
        */
        let mut balances = super::Pallet::new();

        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 100),
            Err("Not enough fund")
        );

        balances.set_balance(&"alice".to_string(), 100);
        assert_eq!(balances.transfer("alice".to_string(), "bob".to_string(), 49), Ok(()));
        assert_eq!(balances.balance(&"alice".to_string()), 51);
        assert_eq!(balances.balance(&"bob".to_string()), 49);
        assert_eq!(
            balances.transfer("alice".to_string(), "bob".to_string(), 100),
            Err("Not enough fund")
        );


    }
}