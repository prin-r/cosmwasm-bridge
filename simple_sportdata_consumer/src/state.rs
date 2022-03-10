use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static OWNER_KEY: &[u8] = b"owner";
pub static BRIDGE_KEY: &[u8] = b"bridge";
pub static RESULT_KEY: &[u8] = b"result";

pub fn owner(storage: &mut dyn Storage) -> Singleton<CanonicalAddr> {
    singleton(storage, OWNER_KEY)
}

pub fn owner_read(storage: &dyn Storage) -> ReadonlySingleton<CanonicalAddr> {
    singleton_read(storage, OWNER_KEY)
}

pub fn bridge(storage: &mut dyn Storage) -> Singleton<CanonicalAddr> {
    singleton(storage, BRIDGE_KEY)
}

pub fn bridge_read(storage: &dyn Storage) -> ReadonlySingleton<CanonicalAddr> {
    singleton_read(storage, BRIDGE_KEY)
}

pub fn result(storage: &mut dyn Storage) -> Singleton<String> {
    singleton(storage, RESULT_KEY)
}

pub fn result_read(storage: &dyn Storage) -> ReadonlySingleton<String> {
    singleton_read(storage, RESULT_KEY)
}