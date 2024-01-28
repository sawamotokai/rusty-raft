use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

struct Server {
    id: Uuid,
    state: Box<dyn State>,
    peers: Vec<Rc<RefCell<Server>>>,
}

trait State {
    fn append_entries(&self) -> Result<Box<dyn State>, &str>;
    fn request_vote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<Box<dyn State>, &str>;
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
    fn append_entries(&self) -> Result<Box<dyn State>, &str> {
        // if given term is greater, become a follower
        Err("Not implemented")
    }
    fn request_vote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<Box<dyn State>, &str> {
        Err("Not implemented")
    }
}

impl State for Follower {
    fn append_entries(&self) -> Result<Box<dyn State>, &str> {
        Err("Not implemented")
    }
    fn request_vote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<Box<dyn State>, &str> {
        if self.term >= term {
            println!("Rejecting vote from {}", candidateId);
            return Err("Not implemented");
        }
        self.term = term;
        Err("Not implemented")
    }
}

impl State for Candidate {
    fn append_entries(&self) -> Result<Box<dyn State>, &str> {
        Err("Not implemented")
    }
    fn request_vote(
        &mut self,
        term: i32,
        candidateId: Uuid,
        lastLogIndex: i32,
        lastLogTerm: i32,
    ) -> Result<Box<dyn State>, &str> {
        Err("Not implemented")
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
        self.state = Box::new(Candidate { term: 1 });
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
}
