use crate::raft::{append_entries_req::Command, raft_client::RaftClient};
use futures::executor::block_on;
use std::{
    sync::Arc,
    sync::Mutex,
    time::{Duration, SystemTime},
};
use tonic::transport::Channel;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
pub struct CommonState {
    // persited states
    pub current_term: usize,
    pub voted_for: Option<Uuid>,
    pub log: Arc<Mutex<Vec<Command>>>,
    pub commit_index: i32, // (volatile) highest log entry known to be commited
    pub last_applied: i32, // (volatile) highest log entry applied on the server
}

pub trait ServerMode: Send + Sync {
    fn append_entries_rpc(
        &self,
        leader_term: usize,
        leader_id: Uuid,
        prev_log_index: usize, // idnex of immediately preceeding entry
        prev_log_term: usize,  // term of immediately preceeding entry
        entries: Box<Vec<Command>>,
        leader_commit: usize, // leader's commit index
    ) -> Result<Box<dyn ServerMode>, String>;
    fn request_vote_rpc(
        &self,
        leader_term: usize,
        candidate_id: Uuid,
        last_log_index: usize,
        last_log_term: usize,
    ) -> Result<Box<dyn ServerMode>, String>;
    fn get_term(&self) -> usize;
    fn get_voted_for(&self) -> Option<Uuid>;
    fn set_term(&mut self, term: usize);
    fn set_voted_for(&mut self, server_voted_for: Uuid);
    fn begin_election(&self) -> Result<Box<dyn ServerMode>, String>;
    fn check_heartbeat(
        &self,
        heatbeat: Option<SystemTime>,
        election_timeout: Duration,
    ) -> Result<Box<dyn ServerMode>, String>;

    fn create_client(
        &self,
        endpoint: String,
    ) -> Result<RaftClient<Channel>, Box<dyn std::error::Error>> {
        let connect = RaftClient::connect(endpoint);
        let client = block_on(connect)?;
        Ok(client)
    }
}
