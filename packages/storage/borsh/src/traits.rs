use std::any::type_name;

use crate::{BorshItem as Item, BorshMap as Map};
use borsh::{BorshDeserialize, BorshSerialize};
use cosmwasm_std::{StdError, StdResult, Storage};
use cw_storage_plus::PrimaryKey;

pub struct Borsh;
impl Borsh {
    pub fn serialize<T: BorshSerialize>(obj: &T) -> StdResult<Vec<u8>> {
        borsh::to_vec(obj).map_err(|err| StdError::serialize_err(type_name::<T>(), err))
    }

    pub fn deserialize<T: BorshDeserialize>(data: &[u8]) -> StdResult<T> {
        T::try_from_slice(data).map_err(|err| StdError::parse_err(type_name::<T>(), err))
    }
}
pub trait NaiveItemStorage: BorshSerialize + BorshDeserialize {
    fn load(storage: &dyn Storage, item: Item<Self>) -> StdResult<Self> {
        item.load(storage)
    }

    fn may_load(storage: &dyn Storage, item: Item<Self>) -> StdResult<Option<Self>> {
        item.may_load(storage)
    }

    fn remove(storage: &mut dyn Storage, item: Item<Self>) {
        item.remove(storage)
    }

    fn save(&self, storage: &mut dyn Storage, item: Item<Self>) -> StdResult<()> {
        item.save(storage, self)
    }

    fn update<A, E>(
        &self,
        storage: &mut dyn Storage,
        item: Item<Self>,
        action: A,
    ) -> Result<Self, E>
    where
        A: FnOnce(Self) -> Result<Self, E>,
        E: From<StdError>,
    {
        item.update(storage, action)
    }
}

pub trait ItemStorage: BorshSerialize + BorshDeserialize {
    const ITEM: Item<'static, Self>;

    fn load(storage: &dyn Storage) -> StdResult<Self> {
        Self::ITEM.load(storage)
    }

    fn may_load(storage: &dyn Storage) -> StdResult<Option<Self>> {
        Self::ITEM.may_load(storage)
    }

    fn remove(storage: &mut dyn Storage) {
        Self::ITEM.remove(storage)
    }

    fn save(&self, storage: &mut dyn Storage) -> StdResult<()> {
        Self::ITEM.save(storage, self)
    }

    fn update<A, E>(&self, storage: &mut dyn Storage, action: A) -> Result<Self, E>
    where
        A: FnOnce(Self) -> Result<Self, E>,
        E: From<StdError>,
    {
        Self::ITEM.update(storage, action)
    }
}

pub trait GenericItemStorage<T: BorshSerialize + BorshDeserialize> {
    const ITEM: Item<'static, T>;

    fn load(storage: &dyn Storage) -> StdResult<T> {
        Self::ITEM.load(storage)
    }

    fn may_load(storage: &dyn Storage) -> StdResult<Option<T>> {
        Self::ITEM.may_load(storage)
    }

    fn save(storage: &mut dyn Storage, item: &T) -> StdResult<()> {
        Self::ITEM.save(storage, item)
    }

    fn update<A, E, S: Storage>(storage: &mut dyn Storage, action: A) -> Result<T, E>
    where
        A: FnOnce(T) -> Result<T, E>,
        E: From<StdError>,
    {
        Self::ITEM.update(storage, action)
    }
}

pub trait NaiveMapStorage<'a>: BorshSerialize + BorshDeserialize {
    fn load<K: PrimaryKey<'a>>(
        storage: &dyn Storage,
        map: Map<'a, K, Self>,
        key: K,
    ) -> StdResult<Self> {
        map.load(storage, key)
    }

    fn may_load<K: PrimaryKey<'a>>(
        storage: &dyn Storage,
        map: Map<'a, K, Self>,
        key: K,
    ) -> StdResult<Option<Self>> {
        map.may_load(storage, key)
    }

    fn remove<K: PrimaryKey<'a>>(storage: &mut dyn Storage, map: Map<'a, K, Self>, key: K) {
        map.remove(storage, key)
    }

    fn save<K: PrimaryKey<'a>>(
        &self,
        storage: &mut dyn Storage,
        map: Map<'a, K, Self>,
        key: K,
    ) -> StdResult<()> {
        map.save(storage, key, self)
    }

    fn update<A, E, K: PrimaryKey<'a>>(
        &self,
        storage: &mut dyn Storage,
        map: Map<'a, K, Self>,
        key: K,
        action: A,
    ) -> Result<Self, E>
    where
        A: FnOnce(Option<Self>) -> Result<Self, E>,
        E: From<StdError>,
    {
        map.update(storage, key, action)
    }
}

pub trait MapStorage<'a, K: PrimaryKey<'a>>: BorshSerialize + BorshDeserialize {
    const MAP: Map<'static, K, Self>;

    fn load(storage: &dyn Storage, key: K) -> StdResult<Self> {
        Self::MAP.load(storage, key)
    }

    fn may_load(storage: &dyn Storage, key: K) -> StdResult<Option<Self>> {
        Self::MAP.may_load(storage, key)
    }

    fn remove(storage: &mut dyn Storage, key: K) {
        Self::MAP.remove(storage, key)
    }

    fn save(&self, storage: &mut dyn Storage, key: K) -> StdResult<()> {
        Self::MAP.save(storage, key, self)
    }

    fn update<A, E>(&self, storage: &mut dyn Storage, key: K, action: A) -> Result<Self, E>
    where
        A: FnOnce(Option<Self>) -> Result<Self, E>,
        E: From<StdError>,
    {
        Self::MAP.update(storage, key, action)
    }
}

pub trait GenericMapStorage<'a, K: PrimaryKey<'a>, T: BorshSerialize + BorshDeserialize> {
    const MAP: Map<'static, K, T>;

    fn load(storage: &dyn Storage, key: K) -> StdResult<T> {
        Self::MAP.load(storage, key)
    }

    fn may_load(storage: &dyn Storage, key: K) -> StdResult<Option<T>> {
        Self::MAP.may_load(storage, key)
    }

    fn save(storage: &mut dyn Storage, key: K, item: &T) -> StdResult<()> {
        Self::MAP.save(storage, key, item)
    }

    fn update<A, E, S: Storage>(&self, storage: &mut dyn Storage, key: K, action: A) -> Result<T, E>
    where
        A: FnOnce(Option<T>) -> Result<T, E>,
        E: From<StdError>,
    {
        Self::MAP.update(storage, key, action)
    }
}
