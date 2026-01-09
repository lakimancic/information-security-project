#[derive(Debug)]
pub struct EncryptedKey {
    pub key_size: usize,
    pub iv_size: usize,
    pub ciphertext: Vec<u8>,
}

#[derive(Debug)]
pub struct PlainKey {
    pub key: Vec<u8>,
    pub iv: Option<Vec<u8>>,
}