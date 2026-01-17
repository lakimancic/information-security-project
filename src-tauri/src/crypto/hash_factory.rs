use crate::crypto::errors::CryptoError;
use crate::crypto::hash::blake_256::Blake256;
use crate::crypto::hash::HashFunction;

pub struct HashFactory {}

impl HashFactory {
    pub fn create(hash_algo: &String) -> Result<Box<dyn HashFunction>, CryptoError> {
        match hash_algo.as_str() {
            "blake256" => Ok(Box::new(Blake256::new())),
            _ => Err(CryptoError::UnknownHashFunction(hash_algo.clone())),
        }
    }
}