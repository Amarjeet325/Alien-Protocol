#![no_std]

mod storage;

use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, Vec};

use storage::{
    add_asset, get_admin, get_position_index, get_position, is_asset_supported, set_admin,
};
pub use storage::Position;

// ── Error / panic messages ───────────────────────────────────────────────────

const ERR_ALREADY_SUPPORTED: &str = "AlreadySupported";

// ── Events ───────────────────────────────────────────────────────────────────

/// Emitted when a new RWA asset is whitelisted as collateral.
#[contractevent]
pub struct AssetAdded {
    pub asset: Address,
}

// ── Contract ─────────────────────────────────────────────────────────────────

#[contract]
pub struct CollateralVault;

#[contractimpl]
impl CollateralVault {
    // ── Initialization ───────────────────────────────────────────────────────

    /// Initialize the contract and set the admin address.
    /// Must be called once before any other function.
    pub fn initialize(env: Env, admin: Address) {
        // Prevent re-initialization.
        if env.storage().instance().has(&storage::DataKey::Admin) {
            panic!("already initialized");
        }
        set_admin(&env, &admin);
    }

    // ── Admin: asset management ──────────────────────────────────────────────

    /// Whitelist a new RWA asset token for use as collateral.
    ///
    /// # Requirements
    /// - Caller must be the admin.
    /// - `asset` must not already be in the supported set.
    ///
    /// # Panics
    /// - `"AlreadySupported"` — if the asset is already whitelisted.
    pub fn add_supported_asset(env: Env, asset: Address) {
        // 1. Require auth from the admin address.
        let admin = get_admin(&env);
        admin.require_auth();

        // 2. Assert the asset is not already supported.
        if is_asset_supported(&env, &asset) {
            panic!("{}", ERR_ALREADY_SUPPORTED);
        }

        // 3. Write SupportedAsset(asset) = true to persistent storage.
        add_asset(&env, &asset);

        // 4. Emit AssetAdded { asset } contract event.
        env.events().publish(
            (soroban_sdk::symbol_short!("AssetAdd"),),
            AssetAdded { asset },
        );
    }

    // ── Queries ──────────────────────────────────────────────────────────────

    /// Returns `true` if the given asset has been whitelisted as collateral.
    pub fn is_supported_asset(env: Env, asset: Address) -> bool {
        is_asset_supported(&env, &asset)
    }

    /// Returns all active collateral positions.
    ///
    /// Iterates the `PositionIndex` (a `Vec<Address>` of users with active
    /// positions) and loads each `Position` from persistent storage.
    /// Users who have fully withdrawn will not appear in the index and are
    /// therefore excluded from the result.
    ///
    /// No authentication required — public read.
    pub fn get_all_positions(env: Env) -> Vec<Position> {
        let index = get_position_index(&env);
        let mut positions: Vec<Position> = Vec::new(&env);

        for user in index.iter() {
            if let Some(position) = get_position(&env, &user) {
                positions.push_back(position);
            }
        }

        positions
    }

    /// Returns the current admin address.
    pub fn get_admin(env: Env) -> Address {
        get_admin(&env)
    }
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::testutils::{Address as _, AuthorizedFunction, AuthorizedInvocation};
    use soroban_sdk::{symbol_short, vec, Address, Env, IntoVal};

    fn setup() -> (Env, CollateralVaultClient<'static>, Address) {
        let env = Env::default();
        env.mock_all_auths();

        let contract_id = env.register_contract(None, CollateralVault);
        let client = CollateralVaultClient::new(&env, &contract_id);

        let admin = Address::generate(&env);
        client.initialize(&admin);

        (env, client, admin)
    }

    // ── initialize ───────────────────────────────────────────────────────────

    #[test]
    fn test_initialize_sets_admin() {
        let (_env, client, admin) = setup();
        assert_eq!(client.get_admin(), admin);
    }

    #[test]
    #[should_panic(expected = "already initialized")]
    fn test_initialize_panics_on_reinit() {
        let (env, client, _admin) = setup();
        let other = Address::generate(&env);
        client.initialize(&other);
    }

    // ── add_supported_asset ──────────────────────────────────────────────────

    #[test]
    fn test_add_supported_asset_success() {
        let (env, client, _admin) = setup();
        let asset = Address::generate(&env);

        // Asset should not be supported before addition.
        assert!(!client.is_supported_asset(&asset));

        client.add_supported_asset(&asset);

        // Asset must be supported after addition.
        assert!(client.is_supported_asset(&asset));
    }

    #[test]
    fn test_add_supported_asset_requires_admin_auth() {
        let (env, client, admin) = setup();
        let asset = Address::generate(&env);

        client.add_supported_asset(&asset);

        // Verify that admin auth was required for the call.
        assert_eq!(
            env.auths(),
            vec![
                &env,
                (
                    admin.clone(),
                    AuthorizedInvocation {
                        function: AuthorizedFunction::Contract((
                            client.address.clone(),
                            symbol_short!("add_supp"),
                            (asset.clone(),).into_val(&env),
                        )),
                        sub_invocations: vec![&env],
                    }
                )
            ]
        );
    }

    #[test]
    #[should_panic(expected = "AlreadySupported")]
    fn test_add_supported_asset_panics_on_duplicate() {
        let (env, client, _admin) = setup();
        let asset = Address::generate(&env);

        client.add_supported_asset(&asset);
        // Second call with the same asset must panic.
        client.add_supported_asset(&asset);
    }

    #[test]
    fn test_add_multiple_different_assets() {
        let (env, client, _admin) = setup();
        let asset_a = Address::generate(&env);
        let asset_b = Address::generate(&env);

        client.add_supported_asset(&asset_a);
        client.add_supported_asset(&asset_b);

        assert!(client.is_supported_asset(&asset_a));
        assert!(client.is_supported_asset(&asset_b));
    }

    // ── get_all_positions ────────────────────────────────────────────────────

    #[test]
    fn test_get_all_positions_empty() {
        let (_env, client, _admin) = setup();
        // No deposits have been made — must return an empty Vec.
        let positions = client.get_all_positions();
        assert_eq!(positions.len(), 0);
    }

    #[test]
    fn test_get_all_positions_after_deposits() {
        use storage::{index_add_user, set_position, Position};

        let (env, client, _admin) = setup();
        let asset = Address::generate(&env);
        let user_a = Address::generate(&env);
        let user_b = Address::generate(&env);

        // Simulate what deposit will do: write Position + update index.
        set_position(
            &env,
            &Position {
                user: user_a.clone(),
                asset: asset.clone(),
                amount: 500,
            },
        );
        index_add_user(&env, &user_a);

        set_position(
            &env,
            &Position {
                user: user_b.clone(),
                asset: asset.clone(),
                amount: 300,
            },
        );
        index_add_user(&env, &user_b);

        let positions = client.get_all_positions();
        assert_eq!(positions.len(), 2);
    }

    #[test]
    fn test_get_all_positions_excludes_withdrawn_users() {
        use storage::{index_add_user, index_remove_user, remove_position, set_position, Position};

        let (env, client, _admin) = setup();
        let asset = Address::generate(&env);
        let user_a = Address::generate(&env);
        let user_b = Address::generate(&env);

        // Both users deposit.
        set_position(
            &env,
            &Position {
                user: user_a.clone(),
                asset: asset.clone(),
                amount: 500,
            },
        );
        index_add_user(&env, &user_a);

        set_position(
            &env,
            &Position {
                user: user_b.clone(),
                asset: asset.clone(),
                amount: 300,
            },
        );
        index_add_user(&env, &user_b);

        // user_b fully withdraws — simulate what withdraw will do.
        remove_position(&env, &user_b);
        index_remove_user(&env, &user_b);

        let positions = client.get_all_positions();
        // Only user_a should remain.
        assert_eq!(positions.len(), 1);
        assert_eq!(positions.get(0).unwrap().user, user_a);
    }

    #[test]
    fn test_index_does_not_duplicate_user() {
        use storage::{get_position_index, index_add_user};

        let (env, _client, _admin) = setup();
        let user = Address::generate(&env);

        index_add_user(&env, &user);
        index_add_user(&env, &user); // second call — must be a no-op
        index_add_user(&env, &user); // third call — must be a no-op

        let index = get_position_index(&env);
        assert_eq!(index.len(), 1);
    }
}
