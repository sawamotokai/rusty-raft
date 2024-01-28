use std::{any::TypeId, cell::RefCell, rc::Rc, vec};

use uuid::Uuid;

struct Server {
    id: Uuid,
    mode: Box<dyn ServerMode>,
    peers: Vec<Rc<RefCell<Server>>>,
}

trait ServerMode {
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
    fn check_heatbeat(&self) -> Result<Box<dyn ServerMode>, String>;
}

#[derive(Debug, Default, PartialEq, Eq, PartialOrd, Ord, Clone)]
struct Command {
    index: usize,
    term: usize,
    content: String,
}

#[derive(Debug, Default, Clone)]
struct CommonState {
    // persited states
    current_term: usize,
    voted_for: Option<Uuid>,
    log: Rc<RefCell<Vec<Command>>>,
    commit_index: i32, // (volatile) highest log entry known to be commited
    last_applied: i32, // (volatile) highest log entry applied on the server
}

#[derive(Debug, Default, Clone)]
struct LeaderMode {
    common_state: CommonState,
    next_index: Vec<usize>, // (volatile) for each peer, index where leaeder needs to start sending logs from on
    match_index: Vec<usize>, // (volatile) for each peer, highest entry leader knows was
                            // replicated?
}

#[derive(Debug, Default, Clone)]
struct FollowerMode {
    common_state: CommonState,
}

#[derive(Debug, Default, Clone)]
struct CandidateMode {
    common_state: CommonState,
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
    fn check_heatbeat(&self) -> Result<Box<dyn ServerMode>, String> {
        Err("Not implemented".to_string())
    }
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
    fn check_heatbeat(&self) -> Result<Box<dyn ServerMode>, String> {
        Err("Not implemented".to_string())
    }
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
    fn check_heatbeat(&self) -> Result<Box<dyn ServerMode>, String> {
        Err("Not implemented".to_string())
    }
}

impl Server {
    fn create_servers(num: usize) -> Vec<Rc<RefCell<Server>>> {
        let mut servers: Vec<Rc<RefCell<Server>>> = (0..num)
            .map(|_| Rc::new(RefCell::new(Server::new())))
            .collect();
        let cloned = servers.clone();
        servers
            .iter_mut()
            .for_each(|s| s.borrow_mut().peers = cloned.clone());
        return servers;
    }

    pub fn new() -> Self {
        Server {
            id: Uuid::new_v4(),
            mode: Box::new(FollowerMode::new()),
            peers: vec![],
        }
    }

    pub fn start(&self) {
        println!("Server {} started...", self.id);
        // start thread to check heartbeats
    }

    fn update_state(&mut self, result: Result<Box<dyn ServerMode>, String>) {
        match result {
            Ok(new_state) => self.mode = new_state,
            Err(err) => println!("{}", err),
        }
    }

    pub fn append_entries_rpc(
        &mut self,
        leader_term: usize,
        leader_id: Uuid,
        prev_log_index: usize, // idnex of immediately preceeding entry
        prev_log_term: usize,  // term of immediately preceeding entry
        entries: Box<Vec<Command>>,
        leader_commit: usize, // leader's commit index
    ) {
        let result: Result<Box<dyn ServerMode>, String> = self.mode.append_entries_rpc(
            leader_term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        );
        self.update_state(result);
    }

    pub fn request_vote_rpc(
        &mut self,
        term: usize,
        candidate_id: Uuid,
        last_log_index: usize,
        last_log_term: usize,
    ) {
        let result: Result<Box<dyn ServerMode>, String> =
            self.mode
                .request_vote_rpc(term, candidate_id, last_log_index, last_log_term);
        self.update_state(result);
    }

    fn begin_election(&mut self) {}
    fn check_heatbeat(&mut self) {}
}

fn main() {
    let servers = Server::create_servers(3);
    servers.iter().for_each(|server| server.borrow().start());
}
