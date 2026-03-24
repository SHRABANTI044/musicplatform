#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, Address};

#[contract]
pub struct LoyaltyContract;

#[contractimpl]
impl LoyaltyContract {

    // Add points to a user
    pub fn add_points(env: Env, user: Address, points: u32) {
        let key = Symbol::new(&env, "POINTS");
        let current: u32 = env.storage().instance().get(&(key.clone(), &user)).unwrap_or(0);
        env.storage().instance().set(&(key, &user), &(current + points));
    }

    // Get points of a user
    pub fn get_points(env: Env, user: Address) -> u32 {
        let key = Symbol::new(&env, "POINTS");
        env.storage().instance().get(&(key, &user)).unwrap_or(0)
    }

    // Redeem points
    pub fn redeem_points(env: Env, user: Address, points: u32) {
        let key = Symbol::new(&env, "POINTS");
        let current: u32 = env.storage().instance().get(&(key.clone(), &user)).unwrap_or(0);

        if current < points {
            panic!("Not enough points");
        }

        env.storage().instance().set(&(key, &user), &(current - points));
    }
}