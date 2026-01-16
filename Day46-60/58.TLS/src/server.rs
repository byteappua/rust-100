use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{info, error};
use crate::{Command, Db, Connection, Frame, Error, Aof};
use std::sync::Arc;
use tokio::sync::broadcast;
use tokio_rustls::TlsAcceptor;
use tokio::io::{AsyncRead, AsyncWrite};

// Generic trait for socket that works with Connection
pub trait AsyncStream: AsyncRead + AsyncWrite + Unpin + Send + 'static {}
impl<T: AsyncRead + AsyncWrite + Unpin + Send + 'static> AsyncStream for T {}

pub async fn run(addr: &str, aof_path: &str, mut shutdown: broadcast::Receiver<()>, tls_acceptor: Option<TlsAcceptor>) -> anyhow::Result<()> {
    info!("Mini-Redis Server starting on {}", addr);
    if tls_acceptor.is_some() {
        info!("TLS enabled");
    }

    let listener = TcpListener::bind(addr).await?;
    let db = Db::new();

    // Initialize AOF
    let aof = Arc::new(Mutex::new(Aof::new(aof_path).await?));

    loop {
        tokio::select! {
            res = listener.accept() => {
                 match res {
                     Ok((socket, _)) => {
                        let db = db.clone();
                        let aof = aof.clone();
                        let tls_acceptor = tls_acceptor.clone();

                        tokio::spawn(async move {
                            if let Some(acceptor) = tls_acceptor {
                                match acceptor.accept(socket).await {
                                    Ok(tls_stream) => {
                                         if let Err(e) = process(tls_stream, db, aof).await {
                                             error!("Connection error: {:?}", e);
                                         }
                                    }
                                    Err(e) => {
                                        error!("TLS handshake failed: {:?}", e);
                                    }
                                }
                            } else {
                                if let Err(e) = process(socket, db, aof).await {
                                    error!("Connection error: {:?}", e);
                                }
                            }
                        });
                     }
                     Err(e) => return Err(e.into()),
                 }
            }
            _ = shutdown.recv() => {
                info!("Server at {} shutting down", addr);
                break;
            }
        }
    }
    Ok(())
}

async fn process<S>(socket: S, db: Db, aof: Arc<Mutex<Aof>>) -> Result<(), Error>
where S: AsyncStream
{
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await? {
        let command = Command::from_frame(frame)?;

        let response = match command {
            Command::Get(cmd) => {
                let value = db.get(&cmd.key);
                match value {
                    Some(v) => Frame::Bulk(v),
                    None => Frame::Null,
                }
            }
            Command::Set(cmd) => {
                db.set(cmd.key.clone(), cmd.value.clone());

                let mut aof = aof.lock().await;
                let persist_cmd = Command::Set(crate::cmd::set::Set { key: cmd.key, value: cmd.value });
                if let Err(e) = aof.append(persist_cmd).await {
                    error!("Failed to append to AOF: {:?}", e);
                }

                Frame::Simple("OK".to_string())
            }
            Command::Publish(cmd) => {
                let count = db.publish(&cmd.channel, cmd.message);
                Frame::Integer(count as i64)
            }
            Command::Subscribe(cmd) => {
                let mut subscriptions = vec![];
                for channel_name in &cmd.channels {
                    let rx = db.subscribe(channel_name.clone());
                    subscriptions.push((channel_name.clone(), rx));

                    let frame = Frame::Array(vec![
                        Frame::Bulk(bytes::Bytes::from("subscribe")),
                        Frame::Bulk(bytes::Bytes::from(channel_name.clone())),
                        Frame::Integer(subscriptions.len() as i64),
                    ]);
                    connection.write_frame(&frame).await?;
                }

                if let Some((channel_name, mut rx)) = subscriptions.pop() {
                     loop {
                         match rx.recv().await {
                             Ok(msg) => {
                                 let frame = Frame::Array(vec![
                                     Frame::Bulk(bytes::Bytes::from("message")),
                                     Frame::Bulk(bytes::Bytes::from(channel_name.clone())),
                                     Frame::Bulk(msg),
                                 ]);
                                 connection.write_frame(&frame).await?;
                             }
                             Err(_) => break, // Channel closed or lagged
                         }
                     }
                }

                return Ok(());
            }
            Command::Ping(cmd) => {
                 match cmd.msg {
                    Some(msg) => Frame::Bulk(msg),
                    None => Frame::Simple("PONG".to_string()),
                }
            }
            Command::Unknown(cmd) => {
                Frame::Error(format!("unknown command '{}'", cmd.command_name))
            }
        };

        connection.write_frame(&response).await?;
    }

    Ok(())
}
