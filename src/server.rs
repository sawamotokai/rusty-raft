use crate::command::Command;
use crate::states::follower::FollowerMode;
use crate::states::states::ServerMode;
use rand::Rng;
use std::{
    sync::Arc,
    sync::Mutex,
    thread::{sleep, yield_now},
    time::{Duration, SystemTime},
    vec,
};
use uuid::Uuid;

pub struct Server {
    id: Uuid,
    mode: Box<dyn ServerMode>,
    peers: Vec<Arc<Mutex<Server>>>,
    election_timeout_mills: Duration,
    last_heartbeat: Option<SystemTime>,
}

impl Server {
    pub fn create_servers(num: usize) -> Vec<Arc<Mutex<Server>>> {
        let mut servers: Vec<Arc<Mutex<Server>>> = (0..num)
            .map(|_| Arc::new(Mutex::new(Server::new())))
            .collect();
        let cloned = servers.clone();
        servers
            .iter_mut()
            .for_each(|s| s.lock().unwrap().peers = cloned.clone());
        return servers;
    }

    pub fn new() -> Self {
        Server {
            id: Uuid::new_v4(),
            mode: Box::new(FollowerMode::new()),
            peers: vec![],
            last_heartbeat: None,
            election_timeout_mills: Duration::from_millis(rand::thread_rng().gen_range(150..=300)),
        }
    }

    pub fn start(&mut self) {
        println!("Server {} started...", self.id);
        // start thread to check heartbeats
        loop {
            sleep(self.election_timeout_mills);
            self.check_heatbeat();
            yield_now();
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
        self.update_state(self.mode.append_entries_rpc(
            leader_term,
            leader_id,
            prev_log_index,
            prev_log_term,
            entries,
            leader_commit,
        ));
    }

    pub fn request_vote_rpc(
        &mut self,
        term: usize,
        candidate_id: Uuid,
        last_log_index: usize,
        last_log_term: usize,
    ) {
        self.update_state(self.mode.request_vote_rpc(
            term,
            candidate_id,
            last_log_index,
            last_log_term,
        ));
    }

    pub fn begin_election(&mut self) {
        self.update_state(self.mode.begin_election());
    }

    pub fn check_heatbeat(&mut self) {
        println!("[{}] Checking heartbeat", self.id);
        self.update_state(
            self.mode
                .check_heartbeat(self.last_heartbeat, self.election_timeout_mills),
        );
    }

    fn update_state(&mut self, result: Result<Box<dyn ServerMode>, String>) {
        match result {
            Ok(new_state) => self.mode = new_state,
            Err(err) => println!("{}", err),
        }
    }
}
