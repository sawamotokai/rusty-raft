use super::follower::FollowerMode;
use super::states::{CommonState, ServerMode};
use crate::raft::append_entries_req::Command;
use std::time::{Duration, SystemTime};
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
struct LeaderMode {
    pub common_state: CommonState,
    pub next_index: Vec<usize>, // (volatile) for each peer, index where leaeder needs to start sending logs from on
    pub match_index: Vec<usize>, // (volatile) for each peer, highest entry leader knows was
                                // replicated?
}

impl ServerMode for LeaderMode {
    fn append_entries_rpc(
        &self,
        leader_term: usize,
        leader_id: Uuid,
        prev_log_index: usize, // idnex of immediately preceeding entry
        prev_log_term: usize,  // term of immediately preceeding entry
        entries: Box<Vec<Command>>,
        leader_commit: usize, // leader's commit index
    ) -> Result<Box<dyn ServerMode>, String> {
        // if given term is greater, become a follower
        if leader_term > self.get_term() {
            let new_state = Box::new(FollowerMode {
                common_state: self.common_state.to_owned(),
            });
            return Ok(new_state);
        }
        Err("Not implemented".to_string())
    }
    fn request_vote_rpc(
        &self,
        leader_term: usize,
        candidate_id: Uuid,
        last_log_index: usize,
        last_log_term: usize,
    ) -> Result<Box<dyn ServerMode>, String> {
        if leader_term > self.get_term() {
            let new_state = Box::new(FollowerMode {
                common_state: self.common_state.to_owned(),
            });
            return Ok(new_state);
        }
        Err("Not implemented".to_string())
    }
    fn get_term(&self) -> usize {
        self.common_state.current_term
    }
    fn get_voted_for(&self) -> Option<Uuid> {
        self.common_state.voted_for
    }
    fn set_term(&mut self, term: usize) {
        self.common_state.current_term = term;
    }
    fn set_voted_for(&mut self, server_voted_for: Uuid) {
        self.common_state.voted_for = Some(server_voted_for);
    }

    fn begin_election(&self) -> Result<Box<dyn ServerMode>, String> {
        Err("Not implemented".to_string())
    }

    fn check_heartbeat(
        &self,
        _last_heartbeat: Option<SystemTime>,
        _timeout_mills: Duration,
    ) -> Result<Box<dyn ServerMode>, String> {
        Ok(Box::new(self.clone()))
    }
}
