use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use bytes::Bytes;
use tokio::sync::broadcast;

/// A wrapper around the `Shared` state of the database.
#[derive(Clone)]
pub struct Db {
    shared: Arc<Shared>,
}

struct Shared {
    state: RwLock<State>,
}

struct State {
    entries: HashMap<String, Bytes>,
    /// The pub/sub key-space. Redis uses a **separate** key space for pub/sub.
    /// We verify this by checking Redis docs: "Pub/Sub has no relation to the key space".
    /// Map: Channel Name -> Broadcast Sender
    pub_sub: HashMap<String, broadcast::Sender<Bytes>>,
}

impl Db {
    pub fn new() -> Db {
        let shared = Arc::new(Shared {
            state: RwLock::new(State {
                entries: HashMap::new(),
                pub_sub: HashMap::new(),
            }),
        });
        Db { shared }
    }

    /// Gets the value associated with the key.
    pub fn get(&self, key: &str) -> Option<Bytes> {
        let state = self.shared.state.read().unwrap();
        state.entries.get(key).cloned()
    }

    /// Sets the value associated with the key.
    pub fn set(&self, key: String, value: Bytes) {
        let mut state = self.shared.state.write().unwrap();
        state.entries.insert(key, value);
    }

    /// Returns a `Receiver` for the requested channel.
    /// The returned `Receiver` is used to receive values broadcast by `PUBLISH` commands.
    pub fn subscribe(&self, channel_name: String) -> broadcast::Receiver<Bytes> {
        use std::collections::hash_map::Entry;

        let mut state = self.shared.state.write().unwrap();

        match state.pub_sub.entry(channel_name) {
            Entry::Occupied(e) => e.get().subscribe(),
            Entry::Vacant(e) => {
                // Create a new broadcast channel
                let (tx, rx) = broadcast::channel(1024);
                e.insert(tx);
                rx
            }
        }
    }

    /// Publishes a message to the channel. Returns the number of subscribers listening on the channel.
    pub fn publish(&self, channel_name: &str, message: Bytes) -> usize {
        let state = self.shared.state.read().unwrap();

        if let Some(tx) = state.pub_sub.get(channel_name) {
            // Send method returns the number of receivers
            // If send fails (no receivers), it returns an error, but we just return 0.
            tx.send(message).unwrap_or(0)
        } else {
            0
        }
    }
}
