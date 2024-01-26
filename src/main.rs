use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

struct Server {
    id: Uuid,
    state: Box<dyn State>,
    peers: Vec<Rc<RefCell<Server>>>,
}

trait State {
    fn appendEntries(&self);
    fn requestVote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<(), String>;
    fn getTerm(&self) -> i32;
}

struct Leader {
    term: i32,
}
struct Follower {
    term: i32,
}
struct Candidate {
    term: i32,
}

impl State for Leader {
    fn appendEntries(&self) {
        // if given term is greater, become a follower
    }
    fn requestVote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<(), String> {
        Ok(())
    }
    fn getTerm(&self) -> i32 {
        self.term
    }
}

impl State for Follower {
    fn appendEntries(&self) {}
    fn requestVote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<(), String> {
        if self.term >= term {
            println!("Rejecting vote from {}", candidateId);
            return Err("error".to_owned());
        }
        self.term = term;
        Ok(())
    }
    fn getTerm(&self) -> i32 {
        self.term
    }
}

impl State for Candidate {
    fn appendEntries(&self) {}
    fn requestVote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<(), String> {
        Ok(())
    }
    fn getTerm(&self) -> i32 {
        self.term
    }
}

impl Server {
    fn create_servers(num: i32) -> Vec<Rc<RefCell<Server>>> {
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
            state: Box::new(Follower { term: 0 }),
            peers: vec![],
        }
    }

    pub fn elect_leader(&mut self) {
        self.state = Box::new(Candidate {
            term: self.state.getTerm() + 1,
        });
        // call RPC
    }

    pub fn start(&self) {
        println!("Server {} started...", self.id);
        // start thread to check heartbeats
    }
}

fn main() {
    let servers = Server::create_servers(3);
    servers.iter().for_each(|server| server.borrow().start());
    println!("Hello, world!");
}
