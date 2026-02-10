use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicBool;

pub struct CryptoJob {
    pub cancel: Arc<AtomicBool>,
}

pub type JobRegistry = Arc<Mutex<HashMap<String, CryptoJob>>>;

pub struct ListenerControl {
    pub stop: Arc<AtomicBool>,
}

pub struct JobGuard {
    pub(crate) registry: JobRegistry,
    pub(crate) filename: String,
}

impl Drop for JobGuard {
    fn drop(&mut self) {
        self.registry.lock().unwrap().remove(&self.filename);
    }
}