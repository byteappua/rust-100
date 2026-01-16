use crate::cmd::{get::Get, set::Set, publish::Publish};
use crate::{Connection, Frame, Error};
use bytes::Bytes;
use tokio::net::TcpStream;

pub struct Client {
    connection: Connection,
}

impl Client {
    /// Establish a connection with the Redis server located at `addr`.
    pub async fn connect(addr: &str) -> Result<Client, Error> {
        let socket = TcpStream::connect(addr).await?;
        let connection = Connection::new(socket);
        Ok(Client { connection })
    }

    /// Get the value of key.
    pub async fn get(&mut self, key: &str) -> Result<Option<Bytes>, Error> {
        let frame = Get {
            key: key.to_string(),
        }.into_frame();

        self.connection.write_frame(&frame).await?;

        match self.read_response().await? {
            Frame::Simple(value) => Ok(Some(Bytes::from(value))), // Should not happen for GET
            Frame::Bulk(value) => Ok(Some(value)),
            Frame::Null => Ok(None),
            frame => Err(Error::Other(format!("unexpected frame: {:?}", frame))),
        }
    }

    /// Set key to hold the string value.
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
