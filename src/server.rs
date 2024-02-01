use crate::states::follower::FollowerMode;
use crate::states::states::ServerMode;
use raft::raft_server::{Raft, RaftServer};
use raft::{
    request_vote_req::Command, AppendEntriesReq, AppendEntriesRes, RequestVoteReq, RequestVoteRes,
};
use rand::Rng;
use std::{
    sync::Arc,
    sync::Mutex,
    thread::{sleep, yield_now},
    time::{Duration, SystemTime},
    vec,
};
use tonic::{transport, Request, Response, Status};
use uuid::Uuid;

pub mod command;
pub mod states;

pub mod raft {
    tonic::include_proto!("raft"); // The string specified here must match the proto package name
}

pub struct Server {
    id: Uuid,
    mode: Box<dyn ServerMode>,
    peers: Vec<Arc<Mutex<Server>>>,
    election_timeout_mills: Duration,
    last_heartbeat: Option<SystemTime>,
}

#[tonic::async_trait]
impl Raft for Server {
    async fn request_vote(
        &self,
        req: Request<RequestVoteReq>,
    ) -> Result<Response<RequestVoteRes>, Status> {
        println!("[Req] {:?}", req);
        let res = RequestVoteRes {
            success: true,
            term: 0,
        };
        Ok(Response::new(res))
    }
    async fn append_entries(
        &self,
        req: Request<AppendEntriesReq>,
    ) -> Result<Response<AppendEntriesRes>, Status> {
        println!("[Req] {:?}", req);
        let res = AppendEntriesRes {
            success: true,
            term: 0,
        };
        Ok(Response::new(res))
    }
}

impl Server {
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

    // pub fn append_entries_rpc(
    //     &mut self,
    //     leader_term: usize,
    //     leader_id: Uuid,
    //     prev_log_index: usize, // idnex of immediately preceeding entry
    //     prev_log_term: usize,  // term of immediately preceeding entry
    //     entries: Box<Vec<Command>>,
    //     leader_commit: usize, // leader's commit index
    // ) {
    //     self.update_state(self.mode.append_entries_rpc(
    //         leader_term,
    //         leader_id,
    //         prev_log_index,
    //         prev_log_term,
    //         entries,
    //         leader_commit,
    //     ));
    // }
    //
    // pub fn request_vote_rpc(
    //     &mut self,
    //     term: usize,
    //     candidate_id: Uuid,
    //     last_log_index: usize,
    //     last_log_term: usize,
    // ) {
    //     self.update_state(self.mode.request_vote_rpc(
    //         term,
    //         candidate_id,
    //         last_log_index,
    //         last_log_term,
    //     ));
    // }

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

fn main() {
    println!("Main function");
}
