use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;
use serde::Deserialize;
use crate::crypto::CryptoRequest;
use crate::key_manager::key::PlainKey;

#[derive(Deserialize, Clone)]
pub enum WatchMode {
    Encrypt(CryptoRequest),
    Decrypt(PlainKey),
}

pub struct WatcherService {
    pub(crate) stop: Arc<AtomicBool>,
    pub(crate) handle: std::thread::JoinHandle<()>,
}

pub type WatcherState = Arc<Mutex<Option<WatcherService>>>;