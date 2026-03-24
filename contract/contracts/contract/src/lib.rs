#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address, Map};

#[contract]
pub struct GovernanceToken;

#[contractimpl]
impl GovernanceToken {

    // Initialize contract with admin
    pub fn initialize(env: Env, admin: Address) {
        let key = symbol_short!("ADMIN");
        env.storage().instance().set(&key, &admin);
    }

    // Mint tokens (only admin)
    pub fn mint(env: Env, to: Address, amount: i128) {
        let admin: Address = env.storage().instance().get(&symbol_short!("ADMIN")).unwrap();
        admin.require_auth();

        let mut balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&symbol_short!("BAL"))
            .unwrap_or(Map::new(&env));

        let current = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, current + amount);

        env.storage().instance().set(&symbol_short!("BAL"), &balances);
    }

    // Transfer tokens
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();

        let mut balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&symbol_short!("BAL"))
            .unwrap();

        let from_balance = balances.get(from.clone()).unwrap_or(0);
        assert!(from_balance >= amount, "Insufficient balance");

        balances.set(from.clone(), from_balance - amount);

        let to_balance = balances.get(to.clone()).unwrap_or(0);
        balances.set(to, to_balance + amount);

        env.storage().instance().set(&symbol_short!("BAL"), &balances);
    }

    // Get balance
    pub fn balance(env: Env, user: Address) -> i128 {
        let balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&symbol_short!("BAL"))
            .unwrap_or(Map::new(&env));

        balances.get(user).unwrap_or(0)
    }
}