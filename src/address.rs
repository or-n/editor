use md5::{Md5, Digest};
use std::fmt;

#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Address([u8; 16]);

pub fn hash(inputs: &Vec<Address>) -> Address {
    let mut hasher = Md5::new();
    for input in inputs {
        hasher.update(input.0)
    }
    Address(hasher.finalize().into())
}

#[derive(Debug)]
pub enum FromStrError {
    HexDecode(hex::FromHexError),
    TryInto(Vec<u8>)
}

impl std::str::FromStr for Address {
    type Err = FromStrError;

    fn from_str(s: &str) -> Result<Address, Self::Err> {
        let bytes = hex::decode(s).map_err(FromStrError::HexDecode)?;
        let address = bytes.try_into().map_err(FromStrError::TryInto)?;
        Ok(Address(address))
    }
}

impl std::string::ToString for Address {
    fn to_string(&self) -> String {
        hex::encode(self.0)
    }
}

impl fmt::Debug for Address {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
