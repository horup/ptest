use std::time::Duration;
use netcode::server::{Event, Server};

#[tokio::main]
async fn main() {
    let mut server: Server<proto::Message> = Server::default();
    server.start(8080).await;

    loop {
        for e in server.poll() {
            match e {
                Event::ClientConnected { client_id } => {
                    println!("{:?} connected", client_id);
                }
                Event::ClientDisconnected { client_id } => {
                    println!("{:?} disconnected", client_id);
                }
                Event::Message { client_id, msg } => {
                    println!("{:?}>{:?}", client_id, msg);
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
