use crate::path::Path;
use borsh::{BorshDeserialize, BorshSerialize};
use cosmwasm_std::{StdError, StdResult, Storage};
use cw_storage_plus::{Key, PrimaryKey};
use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Map<'a, K, T: BorshSerialize + BorshDeserialize> {
    namespace: &'a [u8],
    // see https://doc.rust-lang.org/std/marker/struct.PhantomData.html#unused-type-parameters for why this is needed
    key_type: PhantomData<K>,
    data_type: PhantomData<T>,
}

impl<'a, K, T: BorshSerialize + BorshDeserialize> Map<'a, K, T> {
    pub const fn new(namespace: &'a str) -> Self {
        Map {
            namespace: namespace.as_bytes(),
            data_type: PhantomData,
            key_type: PhantomData,
        }
    }

    pub fn namespace(&self) -> &'a [u8] {
        self.namespace
    }
}

impl<'a, K, T: BorshSerialize + BorshDeserialize> Map<'a, K, T>
where
    K: PrimaryKey<'a>,
{
    pub fn key(&self, k: K) -> Path<T> {
        Path::new(
            self.namespace,
            &k.key().iter().map(Key::as_ref).collect::<Vec<_>>(),
        )
    }

    pub fn save(&self, store: &mut dyn Storage, k: K, data: &T) -> StdResult<()> {
        self.key(k).save(store, data)
    }

    pub fn remove(&self, store: &mut dyn Storage, k: K) {
        self.key(k).remove(store)
    }

    /// load will return an error if no data is set at the given key, or on parse error
    pub fn load(&self, store: &dyn Storage, k: K) -> StdResult<T> {
        self.key(k).load(store)
    }

    /// may_load will parse the data stored at the key if present, returns Ok(None) if no data there.
    /// returns an error on issues parsing
    pub fn may_load(&self, store: &dyn Storage, k: K) -> StdResult<Option<T>> {
        self.key(k).may_load(store)
    }

    /// has returns true or false if any data is at this key, without parsing or interpreting the
    /// contents.
    pub fn has(&self, store: &dyn Storage, k: K) -> bool {
        self.key(k).has(store)
    }

    /// Loads the data, perform the specified action, and store the result
    /// in the database. This is shorthand for some common sequences, which may be useful.
    ///
    /// If the data exists, `action(Some(value))` is called. Otherwise `action(None)` is called.
    pub fn update<A, E>(&self, store: &mut dyn Storage, k: K, action: A) -> Result<T, E>
    where
        A: FnOnce(Option<T>) -> Result<T, E>,
        E: From<StdError>,
    {
        self.key(k).update(store, action)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use borsh_derive::{BorshDeserialize, BorshSerialize};
    use cosmwasm_std::testing::MockStorage;
    use cw_storage_plus::IntKey;
    use std::ops::Deref;

    #[derive(BorshDeserialize, BorshSerialize, PartialEq, Debug, Clone)]
    struct Data {
        pub name: String,
        pub age: i32,
    }

    const PEOPLE: Map<&[u8], Data> = Map::new("people");

    const ALLOWANCE: Map<(&[u8], &[u8]), u64> = Map::new("allow");

    const TRIPLE: Map<(&[u8], u8, &str), u64> = Map::new("triple");

    const TUPLE: Map<(u128, u128), u64> = Map::new("tuple");

    #[test]
    fn save_and_load_tuple() {
        let mut store = MockStorage::new();
        let key = (11u128, 2u128);
        let key2 = (1u128, 12u128);
        TUPLE.save(&mut store, key, &100u64).unwrap();
        TUPLE.save(&mut store, key2, &200u64).unwrap();
        assert_eq!(TUPLE.load(&store, key).unwrap(), 100u64);
        assert_eq!(TUPLE.load(&store, key2).unwrap(), 200u64);
    }

    #[test]
    fn create_path() {
        let path = PEOPLE.key(b"john");
        let key = path.deref();
        // this should be prefixed(people) || john
        assert_eq!("people".len() + "john".len() + 2, key.len());
        assert_eq!(b"people".to_vec().as_slice(), &key[2..8]);
        assert_eq!(b"john".to_vec().as_slice(), &key[8..]);

        let path = ALLOWANCE.key((b"john", b"maria"));
        let key = path.deref();
        // this should be prefixed(allow) || prefixed(john) || maria
        assert_eq!(
            "allow".len() + "john".len() + "maria".len() + 2 * 2,
            key.len()
        );
        assert_eq!(b"allow".to_vec().as_slice(), &key[2..7]);
        assert_eq!(b"john".to_vec().as_slice(), &key[9..13]);
        assert_eq!(b"maria".to_vec().as_slice(), &key[13..]);

        let path = TRIPLE.key((b"john", 8u8, "pedro"));
        let key = path.deref();
        // this should be prefixed(allow) || prefixed(john) || maria
        assert_eq!(
            "triple".len() + "john".len() + 1 + "pedro".len() + 2 * 3,
            key.len()
        );
        assert_eq!(b"triple".to_vec().as_slice(), &key[2..8]);
        assert_eq!(b"john".to_vec().as_slice(), &key[10..14]);
        assert_eq!(8u8.to_cw_bytes(), &key[16..17]);
        assert_eq!(b"pedro".to_vec().as_slice(), &key[17..]);
    }

    #[test]
    fn save_and_load() {
        let mut store = MockStorage::new();

        // save and load on one key
        let john = PEOPLE.key(b"john");
        let data = Data {
            name: "John".to_string(),
            age: 32,
        };
        assert_eq!(None, john.may_load(&store).unwrap());
        john.save(&mut store, &data).unwrap();
        assert_eq!(data, john.load(&store).unwrap());

        // nothing on another key
        assert_eq!(None, PEOPLE.may_load(&store, b"jack").unwrap());

        // same named path gets the data
        assert_eq!(data, PEOPLE.load(&store, b"john").unwrap());

        // removing leaves us empty
        john.remove(&mut store);
        assert_eq!(None, john.may_load(&store).unwrap());
    }

    #[test]
    fn existence() {
        let mut store = MockStorage::new();

        // set data in proper format
        let data = Data {
            name: "John".to_string(),
            age: 32,
        };
        PEOPLE.save(&mut store, b"john", &data).unwrap();

        // set and remove it
        PEOPLE.save(&mut store, b"removed", &data).unwrap();
        PEOPLE.remove(&mut store, b"removed");

        // invalid, but non-empty data
        store.set(&PEOPLE.key(b"random"), b"random-data");

        // any data, including invalid or empty is returned as "has"
        assert!(PEOPLE.has(&store, b"john"));
        assert!(PEOPLE.has(&store, b"random"));

        // if nothing was written, it is false
        assert!(!PEOPLE.has(&store, b"never-writen"));
        assert!(!PEOPLE.has(&store, b"removed"));
    }

    #[test]
    fn composite_keys() {
        let mut store = MockStorage::new();

        // save and load on a composite key
        let allow = ALLOWANCE.key((b"owner", b"spender"));
        assert_eq!(None, allow.may_load(&store).unwrap());
        allow.save(&mut store, &1234).unwrap();
        assert_eq!(1234, allow.load(&store).unwrap());

        // not under other key
        let different = ALLOWANCE.may_load(&store, (b"owners", b"pender")).unwrap();
        assert_eq!(None, different);

        // matches under a proper copy
        let same = ALLOWANCE.load(&store, (b"owner", b"spender")).unwrap();
        assert_eq!(1234, same);
    }

    #[test]
    fn triple_keys() {
        let mut store = MockStorage::new();

        // save and load on a triple composite key
        let triple = TRIPLE.key((b"owner", 10u8, "recipient"));
        assert_eq!(None, triple.may_load(&store).unwrap());
        triple.save(&mut store, &1234).unwrap();
        assert_eq!(1234, triple.load(&store).unwrap());

        // not under other key
        let different = TRIPLE
            .may_load(&store, (b"owners", 10u8, "ecipient"))
            .unwrap();
        assert_eq!(None, different);

        // matches under a proper copy
        let same = TRIPLE.load(&store, (b"owner", 10u8, "recipient")).unwrap();
        assert_eq!(1234, same);
    }
}
