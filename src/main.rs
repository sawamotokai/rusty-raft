mod command;
mod server;
mod states;

use server::Server;
use std::{env, sync::Arc, thread};

fn main() {
    let servers = Server::create_servers(3);
    let handles = servers
        .iter()
        .map(|server| {
            println!("Spawining");
            let server_clone = Arc::clone(&server);
            thread::spawn(move || {
                let lock = server_clone.lock();
                match lock {
                    Ok(mut guard) => guard.start(),
                    Err(err) => println!("{}", err),
                }
            })
        })
        .collect::<Vec<_>>();
    handles.into_iter().for_each(|h| h.join().unwrap());
}
