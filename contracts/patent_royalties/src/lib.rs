#![no_std]
use soroban_sdk::{contract, contractimpl, Address, Env, Map, String};

#[contract]
pub struct PatentRoyalties;

#[contractimpl]
impl PatentRoyalties {
    /// Initialize the contract
    pub fn init(env: Env) {
        if env.storage().instance().get::<_, bool>(&"initialized").is_some() {
            panic!("Already initialized");
        }
        env.storage().instance().set(&"initialized", &true);
    }

    /// IP owner registers a patent.
    /// Stores: patents[patent_id] = (owner, title)
    pub fn register_patent(env: Env, patent_id: u64, owner: Address, title: String) {
        owner.require_auth();
        let mut patents: Map<u64, (Address, String)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        if patents.contains_key(patent_id) {
            panic!("Patent already registered");
        }

        patents.set(patent_id, (owner, title));
        env.storage().instance().set(&"patents", &patents);
    }

    /// Licensor creates a license for a patent.
    /// Stores: licenses[(patent_id, licensee)] = (royalty_amount, paid)
    pub fn create_license(env: Env, patent_id: u64, licensee: Address, royalty_amount: u64) {
        let patents: Map<u64, (Address, String)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        let entry = patents.get(patent_id).unwrap_or_else(|| panic!("Patent not found"));
        let (_owner, _) = entry;

        licensee.require_auth();

        let mut licenses: Map<(u64, Address), (u64, u64)> = env
            .storage()
            .instance()
            .get(&"licenses")
            .unwrap_or(Map::new(&env));

        licenses.set((patent_id, licensee), (royalty_amount, 0u64));
        env.storage().instance().set(&"licenses", &licenses);
    }

    /// Licensee pays royalty for a patent.
    /// Increases the paid amount for the license.
    pub fn pay_royalty(env: Env, patent_id: u64, licensee: Address, amount: u64) {
        licensee.require_auth();

        let mut licenses: Map<(u64, Address), (u64, u64)> = env
            .storage()
            .instance()
            .get(&"licenses")
            .unwrap_or(Map::new(&env));

        let key = (patent_id, licensee.clone());
        let entry = licenses.get(key.clone()).unwrap_or_else(|| panic!("License not found"));
        let (royalty_amount, paid) = entry;

        let mut accumulators: Map<Address, u64> = env
            .storage()
            .instance()
            .get(&"accumulators")
            .unwrap_or(Map::new(&env));

        let to_pay = amount.min(royalty_amount.saturating_sub(paid));
        if to_pay > 0 {
            let new_paid = paid + to_pay;
            licenses.set(key.clone(), (royalty_amount, new_paid));

            let patents: Map<u64, (Address, String)> = env
                .storage()
                .instance()
                .get(&"patents")
                .unwrap_or(Map::new(&env));

            if let Some((owner, _)) = patents.get(patent_id) {
                let current = accumulators.get(owner.clone()).unwrap_or(0u64);
                accumulators.set(owner, current + to_pay);
            }

            env.storage().instance().set(&"licenses", &licenses);
            env.storage().instance().set(&"accumulators", &accumulators);
        }
    }

    /// Owner claims accumulated royalties for a patent.
    pub fn claim_royalties(env: Env, owner: Address, patent_id: u64) {
        owner.require_auth();

        let patents: Map<u64, (Address, String)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        let entry = patents.get(patent_id).unwrap_or_else(|| panic!("Patent not found"));
        let (stored_owner, _) = entry;

        if owner != stored_owner {
            panic!("Not the patent owner");
        }

        let mut accumulators: Map<Address, u64> = env
            .storage()
            .instance()
            .get(&"accumulators")
            .unwrap_or(Map::new(&env));

        let amount = accumulators.get(owner.clone()).unwrap_or(0u64);
        if amount > 0 {
            accumulators.set(owner, 0u64);
            env.storage().instance().set(&"accumulators", &accumulators);
        }
    }

    /// Get patent details. Returns (owner, title).
    pub fn get_patent(env: Env, patent_id: u64) -> (Address, String) {
        let patents: Map<u64, (Address, String)> = env
            .storage()
            .instance()
            .get(&"patents")
            .unwrap_or(Map::new(&env));

        patents.get(patent_id).unwrap_or_else(|| panic!("Patent not found"))
    }

    /// Get license details. Returns (royalty_amount, paid).
    pub fn get_license(env: Env, patent_id: u64, licensee: Address) -> (u64, u64) {
        let licenses: Map<(u64, Address), (u64, u64)> = env
            .storage()
            .instance()
            .get(&"licenses")
            .unwrap_or(Map::new(&env));

        licenses.get((patent_id, licensee)).unwrap_or((0u64, 0u64))
    }
}
