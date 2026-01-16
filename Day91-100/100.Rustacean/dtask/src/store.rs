use std::collections::BTreeMap;
use std::fmt::Debug;
use std::io::Cursor;
use std::ops::RangeBounds;
use std::sync::Arc;

use async_trait::async_trait;
use openraft::storage::{LogState, Snapshot};
use openraft::{
    Entry, EntryPayload, LogId, RaftLogReader, RaftSnapshotBuilder, RaftStorage,
    StorageError, Vote, StoredMembership
};
use tokio::sync::RwLock;
use crate::raft::{NodeId, TypeConfig, Response, Node};
use sqlx::SqlitePool;
use crate::model::{Task, TaskStatus};
use uuid::Uuid;
use chrono::Utc;

#[derive(Debug)]
struct DTaskStoreInner {
    log: BTreeMap<u64, Entry<TypeConfig>>,
    vote: Option<Vote<NodeId>>,
    last_purged_log_id: Option<LogId<NodeId>>,
    snapshot: Option<Snapshot<TypeConfig>>,
    last_membership: StoredMembership<NodeId, Node>,
}

#[derive(Debug, Clone)]
pub struct DTaskStore {
    inner: Arc<RwLock<DTaskStoreInner>>,
    pool: SqlitePool,
}

impl DTaskStore {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            inner: Arc::new(RwLock::new(DTaskStoreInner {
                log: BTreeMap::new(),
                vote: None,
                last_purged_log_id: None,
                snapshot: None,
                last_membership: Default::default(),
            })),
            pool,
        }
    }
}

#[async_trait]
impl RaftLogReader<TypeConfig> for DTaskStore {
    async fn get_log_state(&mut self) -> Result<LogState<TypeConfig>, StorageError<NodeId>> {
        let inner = self.inner.read().await;
        let last = inner.log.iter().last().map(|(_, ent)| ent.log_id);
        let last_purged = inner.last_purged_log_id;

        let last_log_id = match last {
            None => last_purged,
            Some(x) => Some(x),
        };

        Ok(LogState {
            last_purged_log_id: last_purged,
            last_log_id,
        })
    }

    async fn try_get_log_entries<RB: RangeBounds<u64> + Clone + Debug + Send + Sync>(
        &mut self,
        range: RB,
    ) -> Result<Vec<Entry<TypeConfig>>, StorageError<NodeId>> {
        let inner = self.inner.read().await;
        let response = inner.log.range(range.clone())
            .map(|(_, val)| val.clone())
            .collect();
        Ok(response)
    }
}

#[async_trait]
impl RaftSnapshotBuilder<TypeConfig> for DTaskStore {
    async fn build_snapshot(
        &mut self,
    ) -> Result<Snapshot<TypeConfig>, StorageError<NodeId>> {
        let inner = self.inner.read().await;
        let last_applied_log_id = inner.last_purged_log_id;

        Ok(Snapshot {
            meta: openraft::SnapshotMeta {
                last_log_id: last_applied_log_id,
                last_membership: inner.last_membership.clone(),
                snapshot_id: "1".to_string(),
            },
            snapshot: Box::new(Cursor::new(Vec::new())),
        })
    }
}

#[async_trait]
impl RaftStorage<TypeConfig> for DTaskStore {
    type LogReader = Self;
    type SnapshotBuilder = Self;

    async fn save_vote(&mut self, vote: &Vote<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut inner = self.inner.write().await;
        inner.vote = Some(*vote);
        Ok(())
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<NodeId>>, StorageError<NodeId>> {
        let inner = self.inner.read().await;
        Ok(inner.vote)
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn append_to_log<I>(
        &mut self,
        entries: I,
    ) -> Result<(), StorageError<NodeId>>
    where I: IntoIterator<Item = Entry<TypeConfig>> + Send
    {
        let mut inner = self.inner.write().await;
        for entry in entries {
            inner.log.insert(entry.log_id.index, entry);
        }
        Ok(())
    }

    async fn delete_conflict_logs_since(
        &mut self,
        since: LogId<NodeId>,
    ) -> Result<(), StorageError<NodeId>> {
        let mut inner = self.inner.write().await;
        let keys: Vec<u64> = inner.log.range(since.index..).map(|(k, _)| *k).collect();
        for key in keys {
            inner.log.remove(&key);
        }
        Ok(())
    }

    async fn purge_logs_upto(&mut self, log_id: LogId<NodeId>) -> Result<(), StorageError<NodeId>> {
        let mut inner = self.inner.write().await;
        inner.last_purged_log_id = Some(log_id);
        let keys: Vec<u64> = inner.log.range(..=log_id.index).map(|(k, _)| *k).collect();
        for key in keys {
            inner.log.remove(&key);
        }
        Ok(())
    }

    async fn last_applied_state(
        &mut self,
    ) -> Result<(Option<LogId<NodeId>>, StoredMembership<NodeId, Node>), StorageError<NodeId>> {
        let inner = self.inner.read().await;
        Ok((inner.last_purged_log_id, inner.last_membership.clone()))
    }

    async fn apply_to_state_machine(
        &mut self,
        entries: &[Entry<TypeConfig>],
    ) -> Result<Vec<Response>, StorageError<NodeId>> {
        let mut responses = Vec::new();
        for entry in entries {
            if let EntryPayload::Membership(mem) = &entry.payload {
                 let mut inner = self.inner.write().await;
                 inner.last_membership = StoredMembership::new(Some(entry.log_id), mem.clone());
                 responses.push(Response { result: String::new() });
                 continue;
            }

            if let EntryPayload::Normal(req) = &entry.payload {
                let id = Uuid::new_v4().to_string();
                let task = Task {
                    id: id.clone(),
                    payload: req.payload.clone(),
                    created_at: Utc::now().timestamp(),
                    status: TaskStatus::Pending,
                };

                let res = sqlx::query("INSERT INTO tasks (id, payload, created_at, status) VALUES (?, ?, ?, ?)")
                    .bind(&task.id)
                    .bind(&task.payload)
                    .bind(task.created_at)
                    .bind(task.status)
                    .execute(&self.pool)
                    .await;

                match res {
                    Ok(_) => {
                        tracing::info!("Raft applied task: {}", id);
                        responses.push(Response { result: id });
                    }
                    Err(e) => {
                         tracing::error!("Failed to apply task to DB: {}", e);
                         responses.push(Response { result: "".to_string() });
                    }
                }
            } else {
                responses.push(Response { result: String::new() });
            }
        }
        Ok(responses)
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.clone()
    }

    async fn begin_receiving_snapshot(
        &mut self,
    ) -> Result<Box<Cursor<Vec<u8>>>, StorageError<NodeId>> {
        Ok(Box::new(Cursor::new(Vec::new())))
    }

    async fn install_snapshot(
        &mut self,
        meta: &openraft::SnapshotMeta<NodeId, Node>,
        _snapshot: Box<Cursor<Vec<u8>>>,
    ) -> Result<(), StorageError<NodeId>> {
        let mut inner = self.inner.write().await;
        inner.last_purged_log_id = meta.last_log_id;
        inner.last_membership = meta.last_membership.clone();
        inner.snapshot = Some(Snapshot {
            meta: meta.clone(),
            snapshot: Box::new(Cursor::new(Vec::new())),
        });
        Ok(())
    }

    async fn get_current_snapshot(
        &mut self,
    ) -> Result<Option<Snapshot<TypeConfig>>, StorageError<NodeId>> {
        let inner = self.inner.read().await;
        if let Some(snap) = &inner.snapshot {
             Ok(Some(Snapshot {
                 meta: snap.meta.clone(),
                 snapshot: Box::new(Cursor::new(Vec::new())),
             }))
        } else {
            Ok(None)
        }
    }
}
