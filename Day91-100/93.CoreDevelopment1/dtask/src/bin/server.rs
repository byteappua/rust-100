use clap::Parser;
use dtask::network::Network;
use dtask::store::MemStorage;
use dtask::DTaskRaft;
use dtask::models::{AppRequest, Task, TaskStatus};
use dtask::scheduler::TaskScheduler;
use dtask::dispatcher::Dispatcher;
use openraft::Config;
use std::sync::Arc;
use std::collections::BTreeMap;
use openraft::storage::Adaptor;
use tracing_subscriber::EnvFilter;
use tokio::sync::mpsc;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "1")]
    node_id: u64,

    #[arg(long, default_value = "127.0.0.1:50051")]
    addr: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Setup logging
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let args = Args::parse();

    // Setup Raft config
    let config = Config {
        heartbeat_interval: 500,
        election_timeout_min: 1500,
        election_timeout_max: 3000,
        ..Default::default()
    };

    let config = Arc::new(config.validate().unwrap());

    // Setup storage and network
    let store = MemStorage::new();
    let network = Network::new();

    // Adaptor wraps custom storage for OpenRaft
    // For OpenRaft 0.8:
    // Raft<C, N, LS, SM>
    // We defined DTaskRaft as Raft<..., Adaptor, Adaptor>
    // We need to pass both log_store and sm_store which are BOTH the same Adaptor wrapping the same store.
    let log_store = Adaptor::new(store.clone());
    let sm_store = Adaptor::new(store.clone());

    // Create Raft node
    let raft = DTaskRaft::new(args.node_id, config, network, log_store, sm_store).await.unwrap();

    println!("Raft node {} started at {}", args.node_id, args.addr);

    // Setup Dispatcher Channel
    let (tx, rx) = mpsc::channel(100);

    // Start Scheduler
    let scheduler_store = store.clone();
    tokio::spawn(async move {
        let scheduler = TaskScheduler::new(scheduler_store, tx);
        scheduler.run().await;
    });

    // Start Dispatcher
    tokio::spawn(async move {
        let dispatcher = Dispatcher::new(rx);
        dispatcher.run().await;
    });

    // Initialize the cluster (Bootstrap)
    if args.node_id == 1 {
        println!("Initializing cluster...");
        let mut members = BTreeMap::new();
        members.insert(args.node_id, openraft::BasicNode { addr: args.addr.clone() });

        match raft.initialize(members).await {
            Ok(_) => println!("Cluster initialized"),
            Err(e) => println!("Cluster initialization failed (might be already initialized): {:?}", e),
        }
    }

    // Wait for election
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

    // Check status
    let metrics = raft.metrics().borrow().clone();
    println!("State: {:?}", metrics.state);
    println!("Leader: {:?}", metrics.current_leader);

    // Submit a dummy task if leader
    if metrics.state == openraft::ServerState::Leader {
        println!("Submitting a test task...");
        let task = Task {
            id: "task-1".to_string(),
            name: "Demo Task".to_string(),
            payload: vec![],
            status: TaskStatus::Pending,
        };

        match raft.client_write(AppRequest::SubmitTask(task)).await {
            Ok(res) => println!("Task submitted successfully: {:?}", res),
            Err(e) => println!("Task submission failed: {:?}", e),
        }
    }

    // Keep running
    loop {
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
        let metrics = raft.metrics().borrow().clone();
        println!("Heartbeat - State: {:?}, Leader: {:?}", metrics.state, metrics.current_leader);
    }
}
