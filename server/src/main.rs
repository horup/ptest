use std::{collections::HashMap, time::Duration};
use netcode::server::{Event, Server};
use uuid::Uuid;

struct GameInstance {
    pub id:Uuid,
    pub creator:String
}

#[tokio::main]
async fn main() {
    let mut server: Server<proto::Message> = Server::default();
    let mut instances = HashMap::default() as HashMap<Uuid, GameInstance>;
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
                    let Some(msg) = msg.variant else { continue };
                    match msg {
                        
                        _=>{}
                    }
                }
            }
        }
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
