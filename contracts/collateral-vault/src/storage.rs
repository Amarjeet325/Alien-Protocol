use soroban_sdk::{contracttype, Address, Env, Vec};

// ── Storage Keys ────────────────────────────────────────────────────────────

/// Top-level storage key enum for the collateral-vault contract.
#[contracttype]
pub enum DataKey {
    /// Stores the admin address.
    Admin,
    /// Stores whether a given asset is supported (true/false).
    SupportedAsset(Address),
    /// Stores a collateral Position for a given user.
    Position(Address),
    /// Stores the Vec<Address> index of all users with active positions.
    PositionIndex,
}

// ── Position type ────────────────────────────────────────────────────────────

/// Represents a single user's active collateral position in the vault.
#[contracttype]
#[derive(Clone, Debug, PartialEq)]
pub struct Position {
    /// The user who owns this position.
    pub user: Address,
    /// The whitelisted asset used as collateral.
    pub asset: Address,
    /// The current deposited balance (in asset's smallest unit).
    pub amount: i128,
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

// ── Position helpers ─────────────────────────────────────────────────────────

/// Write a Position to persistent storage, keyed by user address.
pub fn set_position(env: &Env, position: &Position) {
    env.storage()
        .persistent()
        .set(&DataKey::Position(position.user.clone()), position);
}

/// Read a Position from persistent storage.
/// Returns `None` if the user has no active position.
pub fn get_position(env: &Env, user: &Address) -> Option<Position> {
    env.storage()
        .persistent()
        .get(&DataKey::Position(user.clone()))
}

/// Remove a Position from persistent storage (called on full withdrawal).
pub fn remove_position(env: &Env, user: &Address) {
    env.storage()
        .persistent()
        .remove(&DataKey::Position(user.clone()));
}

// ── PositionIndex helpers ────────────────────────────────────────────────────

/// Read the full list of users with active positions.
/// Returns an empty Vec if no index exists yet.
pub fn get_position_index(env: &Env) -> Vec<Address> {
    env.storage()
        .persistent()
        .get(&DataKey::PositionIndex)
        .unwrap_or_else(|| Vec::new(env))
}

/// Add a user to the PositionIndex if they are not already present.
pub fn index_add_user(env: &Env, user: &Address) {
    let mut index = get_position_index(env);
    // Only append if the user is not already tracked.
    if !index.contains(user) {
        index.push_back(user.clone());
        env.storage()
            .persistent()
            .set(&DataKey::PositionIndex, &index);
    }
}

/// Remove a user from the PositionIndex (called when their balance reaches zero).
pub fn index_remove_user(env: &Env, user: &Address) {
    let mut index = get_position_index(env);
    // Find the position of the user in the index and remove it.
    if let Some(pos) = index.iter().position(|u| u == *user) {
        index.remove(pos as u32);
        env.storage()
            .persistent()
            .set(&DataKey::PositionIndex, &index);
    }
}
