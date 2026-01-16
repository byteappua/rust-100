use crate::{Client, Error};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use bytes::Bytes;

pub struct ClusterClient {
    nodes: Vec<String>,
    // Map address -> Client
    clients: HashMap<String, Arc<Mutex<Client>>>,
}

impl ClusterClient {
    pub fn new(nodes: Vec<String>) -> Self {
        Self {
            nodes,
            clients: HashMap::new(),
        }
    }

    /// Calculate which node a key belongs to.
    /// Uses DefaultHasher for simplicity (Not compatible with real Redis Cluster CRC16).
    fn get_node_addr(&self, key: &str) -> String {
        let mut hasher = DefaultHasher::new();
        key.hash(&mut hasher);
        let hash = hasher.finish();
        let index = (hash as usize) % self.nodes.len();
        self.nodes[index].clone()
    }

    /// Get a client connection for the given address, connecting if necessary.
    async fn get_client(&mut self, addr: &str) -> Result<Arc<Mutex<Client>>, Error> {
        if let Some(client) = self.clients.get(addr) {
            return Ok(client.clone());
        }

        let client = Client::connect(addr).await?;
        let client = Arc::new(Mutex::new(client));
        self.clients.insert(addr.to_string(), client.clone());
        Ok(client)
    }

    pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>, Error> {
        let addr = self.get_node_addr(key);
        // Note: Connecting might be slow, in real app we keep pool.
        // Here we cache connections in self.clients.
        let client_mutex = self.get_client(&addr).await?;
        let mut client = client_mutex.lock().await;
        client.get(key).await
    }

    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<(), Error> {
        let addr = self.get_node_addr(key);
        let client_mutex = self.get_client(&addr).await?;
        let mut client = client_mutex.lock().await;
        client.set(key, value).await
    }

    // Expose which node a key would go to (for demo purposes)
    pub fn get_target_node(&self, key: &str) -> String {
        self.get_node_addr(key)
    }
}
