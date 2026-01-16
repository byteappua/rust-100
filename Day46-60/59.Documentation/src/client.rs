use crate::cmd::{get::Get, set::Set, publish::Publish};
use crate::{Connection, Frame, Error};
use bytes::Bytes;
use tokio::net::TcpStream;

/// A client for the Mini-Redis server.
///
/// `Client` provides a high-level API to interact with the server.
/// It wraps a `Connection` and handles sending commands and receiving responses.
///
/// # Examples
///
/// ```no_run
/// use mini_redis_doc::Client;
/// use bytes::Bytes;
///
/// # async fn doc() -> anyhow::Result<()> {
/// let mut client = Client::connect("127.0.0.1:6379").await?;
/// client.set("hello", Bytes::from("world")).await?;
/// let val = client.get("hello").await?;
/// # Ok(())
/// # }
/// ```
pub struct Client {
    connection: Connection,
}

impl Client {
    /// Establish a connection with the Redis server located at `addr`.
    ///
    /// # Arguments
    ///
    /// * `addr` - The address of the Redis server (e.g., "127.0.0.1:6379").
    pub async fn connect(addr: &str) -> Result<Client, Error> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(Client { connection })
    }

    /// Get the value of key.
    ///
    /// If the key does not exist, `None` is returned.
    pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>, Error> {
        let frame = Get {
            key: key.to_string(),
        }.into_frame();

        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Simple(value) => Ok(Some(Bytes::from(value))), // Should not happen for GET usually
            Frame::Bulk(value) => Ok(Some(value)),
            Frame::Null => Ok(None),
            frame => Err(Error::Other(format!("unexpected frame: {:?}", frame))),
        }
    }

    /// Set key to hold the string value.
    ///
    /// If key already holds a value, it is overwritten, regardless of its type.
    pub async fn set(&mut self, key: &str, value: Bytes) -> Result<(), Error> {
        let frame = Set {
            key: key.to_string(),
            value,
        }.into_frame();

        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Simple(response) if response == "OK" => Ok(()),
            frame => Err(Error::Other(format!("unexpected frame: {:?}", frame))),
        }
    }

    /// Publish a message to the channel.
    ///
    /// Returns the number of clients that received the message.
    pub async fn publish(&mut self, channel: &str, message: Bytes) -> Result<u64, Error> {
        let frame = Publish {
            channel: channel.to_string(),
            message,
        }.into_frame();

        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Integer(response) => Ok(response as u64),
            frame => Err(Error::Other(format!("unexpected frame: {:?}", frame))),
        }
    }

    /// Read a single response frame.
    async fn read_response(&mut self) -> Result<Frame, Error> {
        let response = self.connection.read_frame().await?;
        match response {
            Some(frame) => Ok(frame),
            None => {
                let err = std::io::Error::new(std::io::ErrorKind::ConnectionReset, "connection reset by peer");
                Err(err.into())
            }
        }
    }
}
