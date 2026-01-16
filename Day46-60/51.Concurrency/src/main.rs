use tokio::net::TcpListener;
use tracing::{info, error};
use mini_redis::{Command, Db, Connection, Frame, Error};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt::init();
    info!("Mini-Redis Server starting on 127.0.0.1:6379");

    let listener = TcpListener::bind("127.0.0.1:6379").await?;
    let db = Db::new();

    loop {
        let (socket, _) = listener.accept().await?;
        let db = db.clone();

        tokio::spawn(async move {
            if let Err(e) = process(socket, db).await {
                error!("Connection error: {:?}", e);
            }
        });
    }
}

async fn process(socket: tokio::net::TcpStream, db: Db) -> Result<(), Error> {
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
                db.set(cmd.key, cmd.value);
                Frame::Simple("OK".to_string())
            }
            Command::Unknown(cmd) => {
                Frame::Error(format!("unknown command '{}'", cmd.command_name))
            }
        };

        connection.write_frame(&response).await?;
    }

    Ok(())
}
