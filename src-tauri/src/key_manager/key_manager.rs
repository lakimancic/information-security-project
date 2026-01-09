use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, BufWriter, Cursor, Read, Write};
use std::path::PathBuf;
use rand::Rng;
use crate::crypto::block::aes256::AES256;
use crate::crypto::block::modes::ofb::OfbMode;
use crate::crypto::CipherInstance;
use crate::crypto::encryptor::Encryptor;
use crate::crypto::hash::blake_256::Blake256;
use crate::crypto::hash::HashFunction;
use crate::crypto::padding::pkcs7::Pkcs7;
use crate::key_manager::errors::KeysError;
use crate::key_manager::key::{EncryptedKey, PlainKey};

#[derive(Debug)]
pub struct KeyManager {
    keys: HashMap<String, EncryptedKey>,
}

impl KeyManager {
    pub fn new() -> Self {
        Self { keys: HashMap::new() }
    }

    pub fn generate_new(&mut self, key_name: String, password: String, key_size: usize, iv_size: usize) -> Result<(), KeysError> {
        if self.keys.contains_key(&key_name) {
            return Err(KeysError::KeyAlreadyExists(key_name));
        }
        let password_bytes = password.as_bytes().to_vec();

        let mut hasher = Blake256::new();
        hasher.update(&password_bytes)?;
        let hash = hasher.finalize()?;

        let mut rng = rand::rng();
        let mut message = (0..key_size + iv_size).map(|_| { rng.random::<u8>()}).collect::<Vec<u8>>();
        message.extend_from_slice(hash.as_ref());

        let mut cipher = Encryptor::from_instance(CipherInstance::Block {
            cipher: Box::new(AES256::new(hash)?),
            mode: Box::new(OfbMode::new(vec![0; 16])),
            padding: Box::new(Pkcs7)
        });

        let mut input = Cursor::new(message);
        let mut output = Cursor::new(vec![]);

        cipher.encrypt(&mut input, &mut output)?;
        let ciphertext = output.into_inner();

        self.keys.insert(key_name, EncryptedKey{
            key_size,
            iv_size,
            ciphertext
        });

        Ok(())
    }

    pub fn find_key(&self, name: String, password: String) -> Result<PlainKey, KeysError> {
        let encrypted_key = self.keys.get(&name).ok_or(KeysError::KeyNotFound(name))?;

        let password_bytes = password.as_bytes().to_vec();

        let mut hasher = Blake256::new();
        hasher.update(&password_bytes)?;
        let hash = hasher.finalize()?;

        let mut input = Cursor::new(encrypted_key.ciphertext.clone());
        let mut output = Cursor::new(vec![]);

        let mut cipher = Encryptor::from_instance(CipherInstance::Block {
            cipher: Box::new(AES256::new(hash.clone())?),
            mode: Box::new(OfbMode::new(vec![0; 16])),
            padding: Box::new(Pkcs7)
        });

        cipher.decrypt(&mut input, &mut output).map_err(|_| KeysError::InvalidPassword)?;
        let plaintext = output.into_inner();

        if plaintext.len() != encrypted_key.key_size + encrypted_key.iv_size + hash.len() {
            return Err(KeysError::InvalidPassword)
        }

        let start_index = plaintext.len() - hash.len();
        if plaintext[start_index..] != hash {
            return Err(KeysError::InvalidPassword)
        }

        let key = plaintext[0..encrypted_key.key_size].to_vec();
        let iv = if encrypted_key.iv_size > 0 {
            Some(plaintext[encrypted_key.key_size..start_index].to_vec())
        } else {
            None
        };
        Ok(PlainKey { key, iv })
    }

    pub fn save_to_disk(&self, path: &PathBuf) -> Result<(), KeysError> {
        let file = File::create(path)?;
        let mut writer = BufWriter::new(file);

        writer.write(b"KOXKM\xde\xad")?;
        writer.write(&self.keys.len().to_le_bytes())?;
        for (name, key) in &self.keys {
            let mut name_bytes = name.as_bytes().to_vec();
            name_bytes.push(0);

            writer.write(name_bytes.as_slice())?;
            writer.write(&key.key_size.to_le_bytes())?;
            writer.write(&key.iv_size.to_le_bytes())?;
            writer.write(&key.ciphertext.len().to_le_bytes())?;
            writer.write(&key.ciphertext)?;
        }

        writer.flush()?;
        Ok(())
    }

    pub fn load_from_disk(&mut self, path: &PathBuf) -> Result<(), KeysError> {
        let file = File::open(path)?;
        let mut reader = BufReader::new(file);

        let mut magic = [0u8; 7];
        reader.read_exact(&mut magic)?;
        if &magic != b"KOXKM\xde\xad" {
            return Err(KeysError::InvalidFormat);
        }

        let mut len_bytes = [0u8; 8];
        reader.read_exact(&mut len_bytes)?;
        let num_keys = usize::from_le_bytes(len_bytes);

        self.keys.clear();

        for _ in 0..num_keys {
            let mut name_bytes = Vec::new();
            loop {
                let mut byte = [0u8; 1];
                reader.read_exact(&mut byte)?;
                if byte[0] == 0 { break; }
                name_bytes.push(byte[0]);
            }
            let name = String::from_utf8(name_bytes).map_err(|_| KeysError::InvalidName)?;

            let mut ks_bytes = [0u8; 8];
            reader.read_exact(&mut ks_bytes)?;
            let key_size = usize::from_le_bytes(ks_bytes);

            let mut iv_bytes = [0u8; 8];
            reader.read_exact(&mut iv_bytes)?;
            let iv_size = usize::from_le_bytes(iv_bytes);

            let mut ct_len_bytes = [0u8; 8];
            reader.read_exact(&mut ct_len_bytes)?;
            let ct_len = usize::from_le_bytes(ct_len_bytes);

            let mut ciphertext = vec![0u8; ct_len];
            reader.read_exact(&mut ciphertext)?;

            self.keys.insert(name, EncryptedKey {
                key_size,
                iv_size,
                ciphertext,
            });
        }

        Ok(())
    }
}