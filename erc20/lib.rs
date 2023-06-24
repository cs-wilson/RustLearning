#![cfg_attr(not(feature = "std"), no_std, no_main)]

#[ink::contract]
mod erc20 {
    use ink::storage::Mapping;

    #[ink(storage)]
    #[derive(Default)]
    pub struct Erc20 {
        total_supply: Balance,
        balances: Mapping<AccountId, Balance>,
        allowances: Mapping<(AccountId, AccountId), Balance>,
    }

    #[derive(Debug, PartialEq, Eq, scale::Encode, scale::Decode)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        InsufficientBalance,
        InsufficientAllowance,
    }

    #[ink(event)]
    pub struct Transfer {
        #[ink(topic)]
        from: AccountId,
        #[ink(topic)]
        to: AccountId,
        value: Balance,
    }

    type Result<T> = core::result::Result<T, Error>;

    impl Erc20 {
        #[ink(constructor)]
        pub fn new(total_supply: Balance) -> Self {
            let caller = Self::env().caller();
            let mut balances = Mapping::new();
            balances.insert(caller, &total_supply);
            Self { 
                total_supply,
                balances,
                ..Default::default()
             }
        }

        #[ink(message)]
        pub fn total_supply(&self) -> Balance {
            self.total_supply
        }

        #[ink(message)]
        pub fn balance_of(&self, owner: AccountId) -> Balance {
            self.balances.get(&owner).unwrap_or_default()
        }

        #[ink(message)]
        pub fn allowance(&self, owner: AccountId, spender: AccountId) -> Balance {
            self.allowances.get(&(owner, spender)).unwrap_or_default()
        }

        #[ink(message)]
        pub fn transfer(&mut self, to: AccountId, value: Balance) -> Result<()> {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        #[ink(message)]
        pub fn approve(&mut self, spender: AccountId, value: Balance) -> Result<()> {
            let owner = self.env().caller();
            self.allowances.insert((owner, spender), &value);
            Ok(())
        }

        #[ink(message)]
        pub fn transfer_from(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let spender = self.env().caller();
            let allowance = self.allowance(from, spender);
            if allowance < value {
                return Err(Error::InsufficientAllowance)
            }
            self.transfer_from_to(from, to, value)?;
            self.allowances.insert((from, spender), &(allowance - value));
            Ok(())
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, value: Balance) -> Result<()> {
            let from_balance = self.balance_of(from);
            let to_balance = self.balance_of(to);
            if from_balance < value {
                return Err(Error::InsufficientBalance)
            }
            self.balances.insert(from, &(from_balance - value));
            self.balances.insert(to, &(to_balance + value));

            self.env().emit_event(Transfer { from, to, value });

            Ok(())
        }

        #[ink(message)]
        pub fn mint(&mut self, to: AccountId, value: Balance) {
            let caller = self.env().caller();
            assert!(caller == self.owner(), "Only owner can mint");
            let to_balance = self.balance_of(to);
            self.balances.insert(to, &(to_balance + value));
            self.total_supply += value;
        }

        #[ink(message)]
        pub fn burn(&mut self, from: AccountId, value: Balance) {
            let caller = self.env().caller();
            assert!(caller == self.owner(), "Only owner can burn");
            let from_balance = self.balance_of(from);
            assert!(from_balance >= value, "Not enough balance to burn");
            self.balances.insert(from, &(from_balance - value));
            self.total_supply -= value;
        }

        #[ink(message)]
        pub fn owner(&self) -> AccountId {
            self.env().caller()
        }

    }
}
