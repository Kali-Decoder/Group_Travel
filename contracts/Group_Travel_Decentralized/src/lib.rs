#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, vec, token};

#[derive(Clone)]
#[contracttype]
pub enum StorageKey {
    GroupID,
}

#[derive(Clone)]
#[contracttype]
pub struct TravelGroup {
    organizer: Address,
    title: String,
    participants: Vec<Address>,
    expenses: Vec<Expense>,
    contributions: Vec<Contribution>,
    itinerary: String,
}

#[derive(Clone)]
#[contracttype]
pub struct Expense {
    description: String,
    amount: u64,
}

#[derive(Clone)]
#[contracttype]
pub struct Contribution {
    contributor: Address,
    amount: u64,
}

#[contract]
pub struct TravelGroupContract;

#[contractimpl]
impl TravelGroupContract {
    pub fn create_group(env: Env, from: Address, title: String, initial_itinerary: String) {
        // Ensure that the transaction sender is authorized to create groups
        from.require_auth();

        // Generate a new unique group ID
        let group_id = env.storage().instance().get::<_, u64>(&StorageKey::GroupID).unwrap_or(0) + 1;

        // Create the travel group
        let mut participants = vec![&env, from.clone()];

        let group = TravelGroup {
            organizer: from,
            title,
            participants,
            expenses: vec![&env],
            contributions: vec![&env],
            itinerary: initial_itinerary,
        };

        // Store the group in the contract storage
        env.storage().instance().set(&StorageKey::GroupID, &group_id);
        env.storage().instance().set(&group_id, &group);
    }

    pub fn join_group(env: Env, from: Address, group_id: u64) {
        // Ensure that the transaction sender is authorized to join groups
        from.require_auth();

        // Retrieve the group from the contract storage
        let mut group: TravelGroup = env.storage().instance().get(&group_id).unwrap();

        // Add the participant to the group
        group.participants.push_back(from);

        // Update the group in the contract storage
        env.storage().instance().set(&group_id, &group);
    }

    pub fn add_expense(env: Env, from: Address, group_id: u64, description: String, amount: u64) {
        // Ensure that the transaction sender is authorized to add expenses
        from.require_auth();

        // Retrieve the group from the contract storage
        let mut group: TravelGroup = env.storage().instance().get(&group_id).unwrap();

        // Add the expense to the group
        let expense = Expense { description, amount };
        group.expenses.push_back(expense);

        // Update the group in the contract storage
        env.storage().instance().set(&group_id, &group);
    }

    pub fn contribute(env: Env, from: Address, group_id: u64, amount: u64) {
        // Ensure that the transaction sender is authorized to contribute
        from.require_auth();

        // Retrieve the group from the contract storage
        let mut group: TravelGroup = env.storage().instance().get(&group_id).unwrap();

        // Add the contribution to the group
        let contribution = Contribution { contributor: from, amount };
        group.contributions.push_back(contribution);

        // Update the group in the contract storage
        env.storage().instance().set(&group_id, &group);
    }

    pub fn get_itinerary(env: Env, group_id: u64) -> String {
        // Retrieve the group from the contract storage
        let group: TravelGroup = env.storage().instance().get(&group_id).unwrap();

        group.itinerary.clone()
    }
}
