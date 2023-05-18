use secret_storage_plus::{Key, KeyDeserialize, PrimaryKey};
use borsh::{BorshDeserialize, BorshSerialize};
use cosmwasm_schema::cw_serde;
use cosmwasm_std::{Addr, Api, ContractInfo, StdResult};

/// Validates an optional address.
pub fn opt_addr_validate(api: &dyn Api, addr: &Option<String>) -> StdResult<Option<Addr>> {
    addr.as_ref()
        .map(|addr| api.addr_validate(addr))
        .transpose()
}

/// Validates a vector of Strings as Addrs
pub fn validate_vec(api: &dyn Api, unvalidated_addresses: &[String]) -> StdResult<Vec<Addr>> {
    let items: Result<Vec<_>, _> = unvalidated_addresses
        .iter()
        .map(|f| api.addr_validate(f.as_str()))
        .collect();
    Ok(items?)
}

#[cw_serde]
#[derive(BorshSerialize, BorshDeserialize, Eq, Default, Hash)]
pub struct RawContract {
    pub address: String,
    pub code_hash: String,
}

#[cw_serde]
#[derive(Hash, Eq)]
pub struct Contract {
    pub address: Addr,
    pub code_hash: String,
}

impl RawContract {
    pub fn new(address: impl Into<String>, code_hash: impl Into<String>) -> Self {
        RawContract {
            address: address.into(),
            code_hash: code_hash.into(),
        }
    }

    pub fn validate(self, api: &dyn Api) -> StdResult<Contract> {
        let address = api.addr_validate(&self.address)?;
        Ok(Contract {
            address,
            code_hash: self.code_hash,
        })
    }

    /// Validates an optional RawContract.
    pub fn validate_optional(api: &dyn Api, contract: Option<Self>) -> StdResult<Option<Contract>> {
        contract.map(|contract| contract.validate(api)).transpose()
    }
}

impl From<ContractInfo> for RawContract {
    fn from(item: ContractInfo) -> Self {
        RawContract {
            address: item.address.to_string(),
            code_hash: item.code_hash,
        }
    }
}

impl Into<ContractInfo> for RawContract {
    fn into(self) -> ContractInfo {
        ContractInfo {
            address: Addr::unchecked(self.address),
            code_hash: self.code_hash,
        }
    }
}

impl Contract {
    pub fn is_valid(&self) -> bool {
        true
    }
    pub fn new(address: impl Into<String>, code_hash: impl Into<String>) -> Self {
        Contract {
            address: Addr::unchecked(address),
            code_hash: code_hash.into(),
        }
    }
}

impl BorshSerialize for Contract {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        self.address.as_bytes().serialize(writer)?;
        self.code_hash.as_bytes().serialize(writer)
    }
}

impl BorshDeserialize for Contract {
    fn deserialize_reader<R: std::io::Read>(reader: &mut R) -> std::io::Result<Self> {
        Ok(Contract {
            address: Addr::unchecked(<String>::deserialize_reader(reader)?),
            code_hash: <String>::deserialize_reader(reader)?,
        })
    }
}

impl From<ContractInfo> for Contract {
    fn from(item: ContractInfo) -> Self {
        Contract {
            address: item.address,
            code_hash: item.code_hash,
        }
    }
}

impl Into<ContractInfo> for Contract {
    fn into(self) -> ContractInfo {
        ContractInfo {
            address: Addr::unchecked(self.address),
            code_hash: self.code_hash,
        }
    }
}

impl Default for Contract {
    fn default() -> Self {
        Contract {
            address: Addr::unchecked(""),
            code_hash: "".to_string(),
        }
    }
}

impl KeyDeserialize for &Contract {
    type Output = Addr;

    fn from_vec(value: Vec<u8>) -> StdResult<Self::Output> {
        let addr = <&str>::from_vec(value)?;
        Ok(Addr::unchecked(addr))
    }
}

// Allow using `Contract` as a key in a `Map`
impl<'a> PrimaryKey<'a> for &Contract {
    type Prefix = ();
    type SubPrefix = ();
    type Suffix = Self;
    type SuperSuffix = Self;

    fn key(&self) -> Vec<Key> {
        vec![Key::Ref(self.address.as_bytes())]
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn borsh_impl() {
        let contract = Contract::new("addr", "code_hash");
        let serialized = contract.try_to_vec().unwrap();
        let deserialized = Contract::try_from_slice(&serialized).unwrap();
        assert_eq!(contract, deserialized);
    }
}
