use super::super::command::Command;
use super::follower::FollowerMode;
use super::states::{CommonState, ServerMode};
use std::time::{Duration, SystemTime};
use uuid::Uuid;
#[derive(Debug, Default, Clone)]
struct CandidateMode {
    pub common_state: CommonState,
}

impl ServerMode for CandidateMode {
    fn append_entries_rpc(
        &self,
        leader_term: usize,
        leader_id: Uuid,
        prev_log_index: usize, // idnex of immediately preceeding entry
        prev_log_term: usize,  // term of immediately preceeding entry
        entries: Box<Vec<Command>>,
        leader_commit: usize, // leader's commit index
    ) -> Result<Box<dyn ServerMode>, String> {
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
        heartbeat: Option<SystemTime>,
        election_timeout: Duration,
    ) -> Result<Box<dyn ServerMode>, String> {
        Err("Not implemented".to_string())
    }
}
