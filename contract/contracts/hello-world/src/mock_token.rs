use soroban_sdk::{contract, contractimpl, Address, Env, String, Symbol};

#[contract]
pub struct MockToken;

const DAY_IN_LEDGERS: u32 = 17280;
const PERSISTENT_BUMP_THRESHOLD: u32 = 7 * DAY_IN_LEDGERS;
const PERSISTENT_BUMP_AMOUNT: u32 = 30 * DAY_IN_LEDGERS;

fn bump_persistent<K: soroban_sdk::IntoVal<Env, soroban_sdk::Val>>(env: &Env, key: &K) {
    if env.storage().persistent().has(key) {
        env.storage().persistent().extend_ttl(
            key,
            PERSISTENT_BUMP_THRESHOLD,
            PERSISTENT_BUMP_AMOUNT,
        );
    }
}

#[contractimpl]
impl MockToken {
    pub fn initialize(env: Env, admin: Address, decimal: u32, name: String, symbol: String) {
        if env.storage().instance().has(&Symbol::new(&env, "admin")) {
            panic!("Already initialized");
        }
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "admin"), &admin);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "decimal"), &decimal);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "name"), &name);
        env.storage()
            .instance()
            .set(&Symbol::new(&env, "symbol"), &symbol);
    }

    pub fn mint(env: Env, to: Address, amount: i128) {
        if amount <= 0 {
            panic!("Invalid amount");
        }
        let admin: Address = env
            .storage()
            .instance()
            .get(&Symbol::new(&env, "admin"))
            .unwrap();
        admin.require_auth();

        let key = (&to,);
        let mut balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if balance > 0 {
            bump_persistent(&env, &key);
        }
        balance += amount;
        env.storage().persistent().set(&key, &balance);
        bump_persistent(&env, &key);

        // Update total supply
        let supply_key = Symbol::new(&env, "total_supply");
        let mut supply: i128 = env.storage().instance().get(&supply_key).unwrap_or(0);
        supply += amount;
        env.storage().instance().set(&supply_key, &supply);
    }

    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) {
        from.require_auth();
        if amount <= 0 {
            panic!("Invalid amount");
        }

        let from_key = (&from,);
        let mut from_balance: i128 = env.storage().persistent().get(&from_key).unwrap_or(0);
        if from_balance > 0 {
            bump_persistent(&env, &from_key);
        }

        if from_balance < amount {
            panic!("Insufficient balance");
        }

        from_balance -= amount;
        env.storage().persistent().set(&from_key, &from_balance);
        bump_persistent(&env, &from_key);

        let to_key = (&to,);
        let mut to_balance: i128 = env.storage().persistent().get(&to_key).unwrap_or(0);
        if to_balance > 0 {
            bump_persistent(&env, &to_key);
        }
        to_balance += amount;
        env.storage().persistent().set(&to_key, &to_balance);
        bump_persistent(&env, &to_key);
    }

    pub fn balance(env: Env, id: Address) -> i128 {
        let key = (&id,);
        let balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        if balance > 0 {
            bump_persistent(&env, &key);
        }
        balance
    }

    pub fn decimals(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "decimal"))
            .unwrap_or(0)
    }

    pub fn name(env: Env) -> String {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "name"))
            .unwrap()
    }

    pub fn symbol(env: Env) -> String {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "symbol"))
            .unwrap()
    }

    pub fn total_supply(env: Env) -> i128 {
        env.storage()
            .instance()
            .get(&Symbol::new(&env, "total_supply"))
            .unwrap_or(0)
    }
}
