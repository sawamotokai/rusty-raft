use crate::states::{follower::FollowerMode, states::ServerMode};
use raft::raft_server::{Raft, RaftServer};
use raft::{AppendEntriesReq, AppendEntriesRes, RequestVoteReq, RequestVoteRes};
use rand::Rng;
use std::env;
use std::net::SocketAddr;
use std::{
    sync::Mutex,
    time::{Duration, SystemTime},
};
use tonic::{transport, Request, Response, Status};
use uuid::Uuid;

pub mod states;

pub mod raft {
    tonic::include_proto!("raft"); // The string specified here must match the proto package name
}

pub struct Server {
    id: Uuid,
    mode: Mutex<Box<dyn ServerMode>>,
    peers: Vec<SocketAddr>,
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
        self.update_state(self.mode.lock().unwrap().request_vote_rpc(
            req.get_ref().leader_term as usize,
            Uuid::parse_str(&req.get_ref().leader_id).unwrap(),
            req.get_ref().prev_log_index as usize,
            req.get_ref().prev_log_term as usize,
        ));
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
        self.update_state(self.mode.lock().unwrap().append_entries_rpc(
            req.get_ref().leader_term as usize,
            Uuid::parse_str(&req.get_ref().leader_id).unwrap(),
            req.get_ref().prev_log_index as usize,
            req.get_ref().prev_log_term as usize,
            Box::new(req.get_ref().entries.clone()),
            req.get_ref().leader_commit as usize,
        ));
        Ok(Response::new(res))
    }
}

impl Server {
    pub fn new(peer_endpoints: Vec<SocketAddr>) -> Self {
        Server {
            id: Uuid::new_v4(),
            mode: Mutex::new(Box::new(FollowerMode::new())),
            peers: peer_endpoints,
            last_heartbeat: None,
            election_timeout_mills: Duration::from_millis(rand::thread_rng().gen_range(150..=300)),
        }
    }

    pub fn begin_election(&mut self) {
        self.update_state(self.mode.lock().unwrap().begin_election());
    }

    pub fn check_heatbeat(&mut self) {
        println!("[{}] Checking heartbeat", self.id);
        self.update_state(
            self.mode
                .lock()
                .unwrap()
                .check_heartbeat(self.last_heartbeat, self.election_timeout_mills),
        );
    }

    fn update_state(&self, result: Result<Box<dyn ServerMode>, String>) {
        match result {
            Ok(new_state) => *self.mode.lock().unwrap() = new_state,
            Err(err) => println!("{}", err),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Main function");
    let peer_endpoints: Result<Vec<SocketAddr>, _> = env::var("PEER_ENDPOINTS")?
        .split(',')
        .map(str::parse)
        .collect();
    let peer_addrs = peer_endpoints?;
    println!("{:?}", peer_addrs);
    let self_addr = env::var("SELF_ENDPOINT")?.parse()?;
    println!("{:?}", self_addr);
    transport::Server::builder()
        .add_service(RaftServer::new(Server::new(peer_addrs)))
        .serve(self_addr)
        .await?;
    println!("After server");
    Ok(())
}
