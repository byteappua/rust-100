use tokio::net::{TcpListener, TcpStream};
use tokio::sync::Mutex;
use tracing::{info, error};
use crate::{Command, Db, Connection, Frame, Error, Aof};
use std::sync::Arc;

pub async fn run(addr: &str, aof_path: &str) -> anyhow::Result<()> {
    info!("Mini-Redis Server starting on {}", addr);

    let listener = TcpListener::bind(addr).await?;
    let db = Db::new();

    // Initialize AOF
    let aof = Arc::new(Mutex::new(Aof::new(aof_path).await?));

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();
        let aof = aof.clone();

        tokio::spawn(async move {
            if let Err(e) = process(socket, db, aof).await {
                error!("Connection error: {:?}", e);
            }
        });
    }
}

async fn process(socket: TcpStream, db: Db, aof: Arc<Mutex<Aof>>) -> Result<(), Error> {
    let mut connection = Connection::new(socket);

    while let Some(frame) = connection.read_frame().await? {
        info!("Received frame: {:?}", frame);

        let command = Command::from_frame(frame)?;
        info!("Parsed command: {:?}", command);

        // Execute the command
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

                // Persist to AOF
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
                // Subscription Logic
                let mut subscriptions = vec![];
                for channel_name in &cmd.channels {
                    let rx = db.subscribe(channel_name.clone());
                    subscriptions.push((channel_name.clone(), rx));

                    // Respond with "subscribe" message
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
            Command::Unknown(cmd) => {
                Frame::Error(format!("unknown command '{}'", cmd.command_name))
            }
        };

        connection.write_frame(&response).await?;
    }

    Ok(())
}
