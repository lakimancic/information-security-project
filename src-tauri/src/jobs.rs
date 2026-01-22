use std::collections::HashMap;
use std::net::{SocketAddr};
use std::sync::{Arc, Condvar, Mutex};
use std::sync::atomic::AtomicBool;

pub struct CryptoJob {
    pub cancel: Arc<AtomicBool>,
}

pub type JobRegistry = Arc<Mutex<HashMap<String, CryptoJob>>>;

pub struct ListenerControl {
    pub stop: Arc<AtomicBool>,
}

pub struct PendingControl {
    pub approved: bool,
    pub canceled: bool,
}

pub type ReceiverRegistry =
    Arc<(Mutex<HashMap<SocketAddr, PendingControl>>, Condvar)>;