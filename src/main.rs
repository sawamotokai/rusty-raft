mod command;
mod server;
mod states;

use server::Server;

fn main() {
    let servers = Server::create_servers(3);
    servers
        .iter()
        .for_each(|server| server.borrow_mut().start());
}
