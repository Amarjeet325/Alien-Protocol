use soroban_sdk::{contracttype, Address, Env};

// ── Storage Keys ────────────────────────────────────────────────────────────

/// Top-level storage key enum for the collateral-vault contract.
#[contracttype]
pub enum DataKey {
    /// Stores the admin address.
    Admin,
    /// Stores whether a given asset is supported (true/false).
    SupportedAsset(Address),
}

// ── Admin helpers ────────────────────────────────────────────────────────────

/// Persist the admin address in instance storage.
pub fn set_admin(env: &Env, admin: &Address) {
    env.storage().instance().set(&DataKey::Admin, admin);
}

/// Read the admin address from instance storage.
/// Panics if the contract has not been initialized.
pub fn get_admin(env: &Env) -> Address {
    env.storage()
        .instance()
        .get(&DataKey::Admin)
        .expect("contract not initialized: admin not set")
}

// ── Supported-asset helpers ──────────────────────────────────────────────────

/// Write `SupportedAsset(asset) = true` to persistent storage.
pub fn add_asset(env: &Env, asset: &Address) {
    env.storage()
        .persistent()
        .set(&DataKey::SupportedAsset(asset.clone()), &true);
}

/// Return `true` when the asset has been whitelisted, `false` otherwise.
pub fn is_asset_supported(env: &Env, asset: &Address) -> bool {
    env.storage()
        .persistent()
        .get(&DataKey::SupportedAsset(asset.clone()))
        .unwrap_or(false)
}
