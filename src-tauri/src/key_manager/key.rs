use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptedKey {
    pub key_size: usize,
    pub iv_size: usize,
    pub ciphertext: Vec<u8>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlainKey {
    pub key: Vec<u8>,
    pub iv: Option<Vec<u8>>,
}

#[derive(Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ShortKey {
    pub name: String,
    pub key_size: usize,
    pub iv_size: usize,
}