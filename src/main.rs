use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

struct Server {
    id: Uuid,
    state: Box<dyn State>,
    peers: Vec<Rc<RefCell<Server>>>,
}

trait State {
    fn append_entries(&mut self) -> Result<Box<dyn State>, String>;
    fn request_vote(
        &mut self,
        term: i32,
        candidate_id: Uuid,
        last_log_index: i32,
        last_log_term: i32,
    ) -> Result<Box<dyn State>, String>;
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
    fn append_entries(&mut self) -> Result<Box<dyn State>, String> {
        // if given term is greater, become a follower
        Err("Not implemented".to_string())
    }
    fn request_vote(
        &mut self,
        term: i32,
        candidate_id: Uuid,
        last_log_index: i32,
        last_log_term: i32,
    ) -> Result<Box<dyn State>, String> {
        Err("Not implemented".to_string())
    }
}

impl State for Follower {
    fn append_entries(&mut self) -> Result<Box<dyn State>, String> {
        Err("Not implemented".to_string())
    }
    fn request_vote(
        &mut self,
        term: i32,
        candidate_id: Uuid,
        last_log_index: i32,
        last_log_term: i32,
    ) -> Result<Box<dyn State>, String> {
        if self.term >= term {
            println!("Rejecting vote from {}", candidate_id);
            return Err("Not implemented".to_string());
        }
        self.term = term;
        Err("Not implemented".to_string())
    }
}

impl State for Candidate {
    fn append_entries(&mut self) -> Result<Box<dyn State>, String> {
        Err("Not implemented".to_string())
    }
    fn request_vote(
        &mut self,
        term: i32,
        candidate_id: Uuid,
        last_log_index: i32,
        last_log_term: i32,
    ) -> Result<Box<dyn State>, String> {
        Err("Not implemented".to_string())
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

    pub fn start(&self) {
        println!("Server {} started...", self.id);
        // start thread to check heartbeats
    }

    fn update_state(&mut self, result: Result<Box<dyn State>, String>) {
        match result {
            Ok(new_state) => self.state = new_state,
            Err(err) => println!("{}", err),
        }
    }

    pub fn append_entries(&mut self) {
        let result: Result<Box<dyn State>, String> = self.state.append_entries();
        self.update_state(result);
    }

    pub fn request_vote(
        &mut self,
        term: i32,
        candidate_id: Uuid,
        last_log_index: i32,
        last_log_term: i32,
    ) {
        let result: Result<Box<dyn State>, String> =
            self.state
                .request_vote(term, candidate_id, last_log_index, last_log_term);
        self.update_state(result);
    }
}

fn main() {
    let servers = Server::create_servers(3);
    servers.iter().for_each(|server| server.borrow().start());
}
