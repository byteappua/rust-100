use mini_redis_tls::{server, Client};
use tokio::sync::broadcast;
use tokio_rustls::rustls::{ServerConfig, RootCertStore, ClientConfig};
use tokio_rustls::{TlsAcceptor, TlsConnector};
use std::sync::Arc;
use std::fs::File;
use std::io::BufReader;
use rustls_pemfile::certs;
use tokio::time::{sleep, Duration};
use bytes::Bytes;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // tracing_subscriber::fmt::init();

    // 1. Load Server Certs
    let cert_file = &mut BufReader::new(File::open("certs/server.cert")?);
    let key_file = &mut BufReader::new(File::open("certs/server.key")?);

    let cert_chain = certs(cert_file).collect::<Result<Vec<_>, _>>()?;

    // Find the first valid private key
    let key = rustls_pemfile::private_key(key_file)?
        .ok_or_else(|| anyhow::anyhow!("No private key found in certs/server.key"))?;

    let server_config = ServerConfig::builder()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key.into())?;

    let tls_acceptor = TlsAcceptor::from(Arc::new(server_config));

    // 2. Start Server
    let (shutdown_tx, shutdown_rx) = broadcast::channel(1);

    tokio::spawn(async move {
        if let Err(e) = server::run("127.0.0.1:6379", "appendonly.aof", shutdown_rx, Some(tls_acceptor)).await {
            eprintln!("Server error: {:?}", e);
        }
    });

    sleep(Duration::from_millis(500)).await;
    println!(">>> TLS Server started on 127.0.0.1:6379");

    // 3. Configure Client TLS
    let ca_file = &mut BufReader::new(File::open("certs/ca.cert")?);
    let mut root_store = RootCertStore::empty();
    for cert in certs(ca_file) {
        root_store.add(cert?)?;
    }

    let client_config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    let connector = TlsConnector::from(Arc::new(client_config));

    // 4. Connect Client
    println!(">>> Client connecting via TLS...");
    let mut client = Client::connect_tls("127.0.0.1:6379", "localhost", connector).await?;

    // 5. Operations
    println!(">>> PING...");
    let pong = client.ping(None).await?;
    println!("<<< PONG: {:?}", pong);

    println!(">>> SET hello = world");
    client.set("hello", Bytes::from("world")).await?;

    println!(">>> GET hello");
    let val = client.get("hello").await?;
    println!("<<< GET: {:?}", val);

    assert_eq!(val, Some(Bytes::from("world")));
    println!(">>> TLS Communication Verified!");

    // Cleanup
    shutdown_tx.send(())?;
    let _ = std::fs::remove_file("appendonly.aof");

    Ok(())
}
