// src/store.rs

use crate::{TypeConfig, models::{AppRequest, AppResponse, Task, TaskStatus}};
use openraft::storage::{RaftStorage, Snapshot, LogState};
use openraft::{Entry, LogId, StorageError, Vote, SnapshotMeta, StoredMembership};
use std::collections::BTreeMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::io::Cursor;
use async_trait::async_trait;

#[derive(Debug, Default)]
pub struct LogStore {
    log: RwLock<BTreeMap<u64, Entry<TypeConfig>>>,
    vote: RwLock<Option<Vote<u64>>>,
}

#[derive(Debug, Default)]
pub struct StateMachineStore {
    tasks: RwLock<BTreeMap<String, Task>>,
    last_applied_log: RwLock<Option<LogId<u64>>>,
    last_membership: RwLock<StoredMembership<u64, openraft::BasicNode>>,
}

#[derive(Clone, Default)]
pub struct MemStorage {
    log_store: Arc<LogStore>,
    sm_store: Arc<StateMachineStore>,
}

impl MemStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn get_pending_tasks(&self) -> Vec<Task> {
        let tasks = self.sm_store.tasks.read().await;
        tasks.values()
            .filter(|t| t.status == TaskStatus::Pending)
            .cloned()
            .collect()
    }
}

#[async_trait]
impl RaftStorage<TypeConfig> for MemStorage {
    type LogReader = Self;
    type SnapshotBuilder = Self;

    async fn save_vote(&mut self, vote: &Vote<u64>) -> Result<(), StorageError<u64>> {
        let mut v = self.log_store.vote.write().await;
        *v = Some(*vote);
        Ok(())
    }

    async fn read_vote(&mut self) -> Result<Option<Vote<u64>>, StorageError<u64>> {
        Ok(*self.log_store.vote.read().await)
    }

    async fn get_log_reader(&mut self) -> Self::LogReader {
        self.clone()
    }

    async fn append_to_log<I>(&mut self, entries: I) -> Result<(), StorageError<u64>>
    where
        I: IntoIterator<Item = Entry<TypeConfig>> + Send,
    {
        let mut log = self.log_store.log.write().await;
        for entry in entries {
            log.insert(entry.log_id.index, entry);
        }
        Ok(())
    }

    async fn delete_conflict_logs_since(&mut self, log_id: LogId<u64>) -> Result<(), StorageError<u64>> {
        let mut log = self.log_store.log.write().await;
        let keys: Vec<u64> = log.range(log_id.index..).map(|(k, _)| *k).collect();
        for k in keys {
            log.remove(&k);
        }
        Ok(())
    }

    async fn purge_logs_upto(&mut self, log_id: LogId<u64>) -> Result<(), StorageError<u64>> {
        let mut log = self.log_store.log.write().await;
        let keys: Vec<u64> = log.range(..=log_id.index).map(|(k, _)| *k).collect();
        for k in keys {
            log.remove(&k);
        }
        Ok(())
    }

    async fn last_applied_state(&mut self) -> Result<(Option<LogId<u64>>, StoredMembership<u64, openraft::BasicNode>), StorageError<u64>> {
        let last_applied = *self.sm_store.last_applied_log.read().await;
        let last_membership = self.sm_store.last_membership.read().await.clone();
        Ok((last_applied, last_membership))
    }

    async fn apply_to_state_machine(&mut self, entries: &[Entry<TypeConfig>]) -> Result<Vec<AppResponse>, StorageError<u64>> {
        let mut tasks = self.sm_store.tasks.write().await;
        let mut last_applied = self.sm_store.last_applied_log.write().await;
        let mut last_membership = self.sm_store.last_membership.write().await;
        let mut responses = Vec::new();

        for entry in entries {
            *last_applied = Some(entry.log_id);

            if let openraft::EntryPayload::Membership(ref mem) = entry.payload {
                 *last_membership = StoredMembership::new(Some(entry.log_id), mem.clone());
                 responses.push(AppResponse::None);
            }
            else if let openraft::EntryPayload::Normal(ref req) = entry.payload {
                match req {
                    AppRequest::SubmitTask(task) => {
                        tasks.insert(task.id.clone(), task.clone());
                        responses.push(AppResponse::TaskSubmitted(task.id.clone()));
                    }
                }
            } else {
                 responses.push(AppResponse::None);
            }
        }
        Ok(responses)
    }

    async fn get_snapshot_builder(&mut self) -> Self::SnapshotBuilder {
        self.clone()
    }

    async fn begin_receiving_snapshot(&mut self) -> Result<Box<Cursor<Vec<u8>>>, StorageError<u64>> {
        Ok(Box::new(Cursor::new(Vec::new())))
    }

    async fn install_snapshot(
        &mut self,
        _meta: &SnapshotMeta<u64, openraft::BasicNode>,
        _snapshot: Box<Cursor<Vec<u8>>>,
    ) -> Result<(), StorageError<u64>> {
        Ok(())
    }

    async fn get_current_snapshot(&mut self) -> Result<Option<Snapshot<TypeConfig>>, StorageError<u64>> {
        Ok(None)
    }
}

#[async_trait]
impl openraft::storage::RaftSnapshotBuilder<TypeConfig> for MemStorage {
    async fn build_snapshot(&mut self) -> Result<Snapshot<TypeConfig>, StorageError<u64>> {
         let last_applied = *self.sm_store.last_applied_log.read().await;
         let last_membership = self.sm_store.last_membership.read().await.clone();

         Ok(Snapshot {
            meta: SnapshotMeta {
                last_log_id: last_applied,
                last_membership,
                snapshot_id: "0".to_string(),
            },
            snapshot: Box::new(Cursor::new(Vec::new())),
        })
    }
}

#[async_trait]
impl openraft::RaftLogReader<TypeConfig> for MemStorage {
    async fn try_get_log_entries<RB: std::ops::RangeBounds<u64> + Clone + std::fmt::Debug + Send>(
        &mut self,
        range: RB,
    ) -> Result<Vec<Entry<TypeConfig>>, StorageError<u64>> {
        let log = self.log_store.log.read().await;
        let response = log.range(range).map(|(_, val)| val.clone()).collect();
        Ok(response)
    }

    // Moved from RaftStorage (incorrect place) to RaftLogReader (correct place for OpenRaft 0.8)
    async fn get_log_state(&mut self) -> Result<LogState<TypeConfig>, StorageError<u64>> {
        let log = self.log_store.log.read().await;
        let last = log.iter().last().map(|(_, ent)| ent.log_id);
        Ok(LogState {
            last_purged_log_id: None,
            last_log_id: last,
        })
    }
}
