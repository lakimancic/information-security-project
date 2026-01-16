use std::collections::HashMap;
use std::net::IpAddr;
use crate::key_manager::key::PlainKey;

mod sender;
mod errors;
pub mod commands;
mod receiver;

pub type NetworkKeys = HashMap<IpAddr, PlainKey>;