use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use bytes::Bytes;

/// A wrapper around the `Shared` state of the database.
/// `Db` is cheap to clone (Arc increment).
#[derive(Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

struct Shared {
    state: Mutex<State>,
}

struct State {
    entries: HashMap<String, Bytes>,
}

impl Db {
    pub fn new() -> Db {
        let shared = Arc::new(Shared {
            state: Mutex::new(State {
                entries: HashMap::new(),
            }),
        });
        Db { shared }
    }

    /// Gets the value associated with the key.
    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.lock().unwrap();
        state.entries.get(key).cloned()
    }

    /// Sets the value associated with the key.
    pub fn set(&self, key: String, value: Bytes) {
        let mut state = self.shared.state.lock().unwrap();
        state.entries.insert(key, value);
    }
}
