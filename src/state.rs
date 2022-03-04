use cosmwasm_std::{Uint128, Storage, CanonicalAddr};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton, PrefixedStorage, ReadonlyPrefixedStorage};

pub static OWNER_KEY: &[u8] = b"owner";
pub static BLOCK_DETAILS_KEY: &[u8] = b"block_details";
pub static VALIDATORS_POWER_KEY: &[u8] = b"validators_power";
pub static TOTAL_VALIDATOR_POWER_KEY: &[u8] = b"total_validators_power";
pub static CANDIDATE_BLOCK_DETAILS: &[u8] = b"candidate_block_details";
pub static VERIFIED_RESULTS_KEY: &[u8] = b"verified_results";
pub static TOTAL_VALIDATOR_POWER_LAST_UPDATED: &[u8] = b"total_validator_power_last_updated";

pub fn owner(storage: &mut dyn Storage) -> Singleton<CanonicalAddr> {
    singleton(storage, OWNER_KEY)
}

pub fn owner_read(storage: &dyn Storage) -> ReadonlySingleton<CanonicalAddr> {
    singleton_read(storage, OWNER_KEY)
}

pub fn block_details(storage: &mut dyn Storage) -> PrefixedStorage {
    PrefixedStorage::new(storage, BLOCK_DETAILS_KEY)
}

pub fn block_details_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, BLOCK_DETAILS_KEY)
}

pub fn validators_power(storage: &mut dyn Storage) -> PrefixedStorage {
    PrefixedStorage::new(storage, VALIDATORS_POWER_KEY)
}

pub fn validators_power_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, VALIDATORS_POWER_KEY)
}

pub fn total_validator_power(storage: &mut dyn Storage) -> Singleton<Uint128> {
    singleton(storage, TOTAL_VALIDATOR_POWER_KEY)
}

pub fn total_validator_power_read(storage: &dyn Storage) -> ReadonlySingleton<Uint128> {
    singleton_read(storage, TOTAL_VALIDATOR_POWER_KEY)
}

pub fn candidate_block_details(storage: &mut dyn Storage) -> PrefixedStorage {
    PrefixedStorage::new(storage, CANDIDATE_BLOCK_DETAILS)
}

pub fn candidate_block_details_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, CANDIDATE_BLOCK_DETAILS)
}

pub fn verified_results(storage: &mut dyn Storage) -> PrefixedStorage {
    PrefixedStorage::new(storage, VERIFIED_RESULTS_KEY)
}

pub fn verified_results_read(storage: &dyn Storage) -> ReadonlyPrefixedStorage {
    ReadonlyPrefixedStorage::new(storage, VERIFIED_RESULTS_KEY)
}

pub fn total_validator_power_last_updated(storage: &mut dyn Storage) -> Singleton<u64> {
    singleton(storage, TOTAL_VALIDATOR_POWER_LAST_UPDATED)
}

pub fn total_validator_power_last_updated_read(storage: &dyn Storage) -> ReadonlySingleton<u64> {
    singleton_read(storage, TOTAL_VALIDATOR_POWER_LAST_UPDATED)
}
