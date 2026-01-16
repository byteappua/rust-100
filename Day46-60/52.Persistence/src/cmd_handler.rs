use crate::{Command, Db, Frame, Error};
use bytes::Bytes;
use crate::cmd::{Get, Set, Unknown};

impl Command {
    pub(crate) fn apply(self, db: &Db, dst: &mut crate::Connection) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<(), Error>> + Send>> {
        let db = db.clone();
        // Since Connection is not Clone, we cannot pass it easily to async block if we consume it.
        // But here we are passing mutable reference.
        // We need to Box::pin because async traits are not yet stable.
        // However, we are not using a trait here, just a method.
        // But to make it recursive-safe or just easy to call in loop:

        Box::pin(async move {
            let response = match self {
                Command::Get(cmd) => cmd.apply(&db).await,
                Command::Set(cmd) => cmd.apply(&db).await,
                Command::Unknown(cmd) => cmd.apply().await,
            };

            // Write the response back to the client
            // Note: In a real app we would pass `&mut Connection` down.
            // Here, we just return the frame and let the caller write it?
            // Or we can restructure.
            // Let's change the signature to return Frame, and let the loop write it.
            // But wait, `apply` signature in typical Mini-Redis is `apply(self, db, dst)`.
            // Let's assume we pass `dst` (Connection) to `apply`.
            // But `dst` needs to be `&mut Connection`.
            // Let's simplify: `apply` returns `Frame`.

            // Wait, we need to modify the implementation of `apply` in the sub-commands.
            // Let's do that.

            // NO, wait. I can't easily modify the `Command` enum methods without modifying `src/cmd/mod.rs` and others.
            // I should have planned to modify them.
            // Let's implement `apply` on `Command` that returns `Frame`.

            dst.write_frame(&response).await
        })
    }
}

impl Get {
    pub(crate) async fn apply(self, db: &Db) -> Frame {
        let value = db.get(&self.key);
        match value {
            Some(v) => Frame::Bulk(v),
            None => Frame::Null,
        }
    }
}

impl Set {
    pub(crate) async fn apply(self, db: &Db) -> Frame {
        db.set(self.key, self.value);
        Frame::Simple("OK".to_string())
    }
}

impl Unknown {
    pub(crate) async fn apply(self) -> Frame {
        Frame::Error(format!("unknown command '{}'", self.command_name))
    }
}
