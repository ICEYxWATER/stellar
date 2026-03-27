#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, Address};

#[contract]
pub struct CarbonTracker;

#[contractimpl]
impl CarbonTracker {

    // Add carbon emission for a user
    pub fn add_emission(env: Env, user: Address, amount: i128) {
        // 🔐 Require user authentication (recommended)
        user.require_auth();

        let key = Symbol::new(&env, "emissions");

        let mut data: Map<Address, i128> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let current = data.get(user.clone()).unwrap_or(0);
        data.set(user, current + amount);

        env.storage().instance().set(&key, &data);
    }

    // Get total emissions of a user
    pub fn get_emission(env: Env, user: Address) -> i128 {
        let key = Symbol::new(&env, "emissions");

        let data: Map<Address, i128> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        data.get(user).unwrap_or(0)
    }

    // Reset emissions (user must authorize)
    pub fn reset(env: Env, user: Address) {
        user.require_auth();

        let key = Symbol::new(&env, "emissions");

        let mut data: Map<Address, i128> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        data.set(user, 0);
        env.storage().instance().set(&key, &data);
    }
}


/* =========================================================
   ===================== README ==============================
   =========================================================

   Carbon Tracker (Soroban Smart Contract)

   Project Description
   ---------------------------------------------------------
   Carbon Tracker is a decentralized application built on the
   Stellar Soroban smart contract platform. It allows users
   to track their carbon emissions securely and transparently
   on-chain.

   What it does
   ---------------------------------------------------------
   - Records carbon emissions per user
   - Stores emission data on blockchain
   - Retrieves total emissions for any user
   - Allows users to reset their own emissions

   Features
   ---------------------------------------------------------
   - Decentralized carbon tracking
   - Per-user emission storage
   - Secure with user authentication
   - Built using Rust and Soroban SDK
   - Easy to extend (carbon credits, rewards, etc.)

   Deployed Smart Contract Link
   ---------------------------------------------------------
   carbon tracker

   Future Improvements
   ---------------------------------------------------------
   - Carbon credit marketplace
   - Emission analytics dashboard
   - Token rewards for low emissions
   - Organization-level tracking

   Tech Stack
   ---------------------------------------------------------
   - Rust
   - Soroban SDK
   - Stellar Blockchain

   How to Use
   ---------------------------------------------------------
   1. Deploy contract on Soroban network
   2. Call add_emission to log emissions
   3. Call get_emission to fetch data
   4. Call reset to clear emissions (requires auth)

   License
   ---------------------------------------------------------
   MIT

========================================================= */