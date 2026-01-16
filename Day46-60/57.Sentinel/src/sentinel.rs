use crate::Client;
use tokio::sync::Mutex;
use std::sync::Arc;
use tokio::time::{sleep, Duration};
use tracing::{info, warn, error};

pub struct SentinelService {
    master_addr: Arc<Mutex<String>>,
    replicas: Vec<String>,
}

impl SentinelService {
    pub fn new(master_addr: String, replicas: Vec<String>) -> Self {
        Self {
            master_addr: Arc::new(Mutex::new(master_addr)),
            replicas,
        }
    }

    // Run the monitoring loop.
    // Takes Arc<Mutex<Self>> to ensure we don't hold the lock while sleeping or connecting.
    pub async fn start(sentinel: Arc<Mutex<Self>>, check_interval: Duration) {
        let master_addr = sentinel.lock().await.get_master_addr().await;
        info!("Sentinel started. Monitoring master: {}", master_addr);

        loop {
            sleep(check_interval).await;

            // 1. Get current master (brief lock)
            let current_master = sentinel.lock().await.get_master_addr().await;

            // 2. Check health (no lock)
            let is_healthy = match Client::connect(&current_master).await {
                Ok(mut client) => client.ping(None).await.is_ok(),
                Err(_) => false,
            };

            // 3. Failover if needed (brief lock)
            if !is_healthy {
                warn!("Master {} is down/unreachable", current_master);
                let guard = sentinel.lock().await;
                // Double check to avoid race
                if guard.get_master_addr().await == current_master {
                    guard.failover().await;
                }
            }
        }
    }

    async fn failover(&self) {
        let mut master_lock = self.master_addr.lock().await;

        // Simple failover: pick the first replica that works
        for replica in &self.replicas {
            if replica == &*master_lock {
                continue;
            }

            info!("Attempting failover to replica: {}", replica);

            if let Ok(mut client) = Client::connect(replica).await {
                if client.ping(None).await.is_ok() {
                    warn!("+++ FAILOVER: Promoting {} to MASTER +++", replica);
                    *master_lock = replica.clone();
                    return;
                }
            }
        }

        error!("All nodes unreachable! Cluster is down.");
    }

    pub async fn get_master_addr(&self) -> String {
        self.master_addr.lock().await.clone()
    }
}
