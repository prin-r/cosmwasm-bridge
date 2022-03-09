use cosmwasm_std::{CanonicalAddr, Storage};
use cosmwasm_storage::{singleton, singleton_read, ReadonlySingleton, Singleton};

pub static OWNER_KEY: &[u8] = b"owner";
pub static BRIDGE_KEY: &[u8] = b"bridge";
pub static RESULT_KEY: &[u8] = b"result";

pub fn owner<S: Storage>(storage: &mut S) -> Singleton<S, CanonicalAddr> {
    singleton(storage, OWNER_KEY)
}

pub fn owner_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, CanonicalAddr> {
    singleton_read(storage, OWNER_KEY)
}

pub fn bridge<S: Storage>(storage: &mut S) -> Singleton<S, CanonicalAddr> {
    singleton(storage, BRIDGE_KEY)
}

pub fn bridge_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, CanonicalAddr> {
    singleton_read(storage, BRIDGE_KEY)
}

pub fn result<S: Storage>(storage: &mut S) -> Singleton<S, String> {
    singleton(storage, RESULT_KEY)
}

pub fn result_read<S: Storage>(storage: &S) -> ReadonlySingleton<S, String> {
    singleton_read(storage, RESULT_KEY)
}