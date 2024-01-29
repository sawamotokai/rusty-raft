use super::super::command::Command;
use super::states::{CommonState, ServerMode};
use std::time::{Duration, SystemTime};
use uuid::Uuid;
#[derive(Debug, Default, Clone)]
pub struct FollowerMode {
    pub common_state: CommonState,
}

impl FollowerMode {
    pub fn new() -> Self {
        FollowerMode {
            common_state: CommonState {
                commit_index: -1,
                last_applied: -1,
                ..Default::default()
            },
        }
    }
}

impl ServerMode for FollowerMode {
    fn append_entries_rpc(
        &self,
        leader_term: usize,
        leader_id: Uuid,
        prev_log_index: usize, // idnex of immediately preceeding entry
        prev_log_term: usize,  // term of immediately preceeding entry
        entries: Box<Vec<Command>>,
        leader_commit: usize, // leader's commit index
    ) -> Result<Box<dyn ServerMode>, String> {
        let mut new_state = Box::new(self.clone());
        if leader_term < self.get_term() {
            return Err("leader is behind".to_string());
        }
        if prev_log_index as usize >= self.common_state.log.borrow().len()
            || prev_log_term != self.common_state.log.borrow()[prev_log_index].term
        {
            return Err("prev entry doesn't match".to_string());
        }
        let first_index = prev_log_index + 1;
        for (i, entry) in entries.iter().enumerate() {
            let index = i + first_index;
            if self.common_state.log.borrow().len() <= index {
                new_state
                    .common_state
                    .log
                    .borrow_mut()
                    .push(entry.to_owned());
            } else if self.common_state.log.borrow()[index].term != entry.term {
                new_state.common_state.log.borrow_mut()[index] = entry.to_owned();
            } else {
                println!("Checking commands identity");
                assert_eq!(self.common_state.log.borrow()[index], *entry);
            }
        }
        let last_index = self.common_state.log.borrow().len() - 1;
        if leader_commit > self.common_state.commit_index as usize {
            new_state.common_state.commit_index = std::cmp::min(leader_commit, last_index) as i32;
        }
        Ok(new_state)
    }

    fn request_vote_rpc(
        &self,
        term: usize,
        candidate_id: Uuid,
        last_log_index: usize,
        last_log_term: usize,
    ) -> Result<Box<dyn ServerMode>, String> {
        if self.get_term() >= term {
            println!("Rejecting vote from {}", candidate_id);
            return Err("Not implemented".to_string());
        }
        let mut new_state = Box::new(self.clone());
        new_state.set_term(term);
        Ok(new_state)
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
        last_heartbeat: Option<SystemTime>,
        election_timeout: Duration,
    ) -> Result<Box<dyn ServerMode>, String> {
        match last_heartbeat {
            None => self.begin_election(),
            Some(heartbeat) => {
                if SystemTime::now().duration_since(heartbeat).unwrap() > election_timeout {
                    self.begin_election()
                } else {
                    Ok(Box::new(self.clone()) as Box<dyn ServerMode>)
                }
            }
        }
    }
}
