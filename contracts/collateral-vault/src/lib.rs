#![no_std]

mod storage;

use soroban_sdk::{contract, contractevent, contractimpl, Address, Env};

use storage::{add_asset, get_admin, is_asset_supported, set_admin};

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
}
