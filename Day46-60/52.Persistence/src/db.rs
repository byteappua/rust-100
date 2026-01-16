use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bytes::Bytes;

/// A wrapper around the `Shared` state of the database.
/// `Db` is cheap to clone (Arc increment).
#[derive(Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

struct Shared {
    state: RwLock<State>,
}

struct State {
    entries: HashMap<String, Bytes>,
}

impl Db {
    pub fn new() -> Db {
        let shared = Arc::new(Shared {
            state: RwLock::new(State {
                entries: HashMap::new(),
            }),
        });
        Db { shared }
    }

    /// Gets the value associated with the key.
    /// Uses a read lock to allow concurrent readers.
    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.read().unwrap();
        state.entries.get(key).cloned()
    }

    /// Sets the value associated with the key.
    /// Uses a write lock to ensure exclusive access during update.
    pub fn set(&self, key: String, value: Bytes) {
        let mut state = self.shared.state.write().unwrap();
        state.entries.insert(key, value);
    }
}
