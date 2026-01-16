use crate::{TypeConfig, Request, Response};
use openraft::{Entry, LogId, Vote};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Default)]
pub struct LogStore {
    log: RwLock<BTreeMap<u64, Entry<TypeConfig>>>,
    vote: RwLock<Option<Vote<u64>>>,
}

#[derive(Debug, Default)]
pub struct StateMachineStore {
    data: RwLock<BTreeMap<String, String>>,
    last_applied_log: RwLock<Option<LogId<u64>>>,
}

#[derive(Clone, Default)]
pub struct MyStorage {
    log_store: Arc<LogStore>,
    sm_store: Arc<StateMachineStore>,
}

impl MyStorage {
    pub fn new() -> Self {
        Self::default()
    }
}

// impl RaftStorage for MyStorage commented out due to openraft 0.9 migration complexity in PoC.
// Will be addressed in Core Development phase.
