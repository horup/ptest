use log::*;
use netcode::server::{ClientId, Event, Server};
use proto::{message::Variant, CreateInstance, Join, Message, Welcome};
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

struct GameInstance {
    pub id: Uuid,
    pub name: String,
    pub creator: Uuid,
}

struct Player {
    pub id: Uuid,
    pub client_id: Option<ClientId>,
    pub name: String,
    pub instance_id: Option<Uuid>,
}
trait Players {
    fn find_player(&self, client_id: ClientId) -> Option<&Player>;
}
impl Players for HashMap<Uuid, Player> {
    fn find_player(&self, client_id: ClientId) -> Option<&Player> {
        self.iter().find(|x|x.1.client_id == Some(client_id)).map(|x|x.1)
    }
}

struct Context {
    pub server: Server<proto::Message>,
    pub instances: HashMap<Uuid, GameInstance>,
    pub players: HashMap<Uuid, Player>,
}

fn on_create_instance(c:&mut Context, ci:CreateInstance, client_id: ClientId) {
    let player = c.players.find_player(client_id);
}

fn on_join(c: &mut Context, j: Join, client_id: ClientId) {
    let Ok(id) = Uuid::parse_str(&j.id) else {
        return;
    };
    // clear client_id from players
    for (_, player) in c.players.iter_mut() {
        if player.client_id == Some(client_id) {
            player.client_id = None;
        }
    }

    // create player struct if not existing
    if c.players.contains_key(&id) == false {
        let player = Player {
            id,
            client_id: None,
            name: "".to_owned(),
            instance_id: None,
        };
        c.players.insert(id.to_owned(), player);
    }

    // assign client to player
    let player = c.players.get_mut(&id).unwrap();
    if player.client_id.is_some() {
        // TODO, disconnect existing client
    }
    player.name = j.name.clone();
    player.client_id = Some(client_id);

    c.server.send(
        client_id,
        Variant::Welcome(Welcome {
            current_instance: player.instance_id.unwrap_or_default().to_string(),
        })
        .into(),
    );
}

fn tick(c: &mut Context) {
    let messages = c.server.poll();
    for e in messages {
        match e {
            Event::ClientConnected { client_id } => {
                info!("{:?} connected", client_id);
            }
            Event::ClientDisconnected { client_id } => {
                info!("{:?} disconnected", client_id);
            }
            Event::Message { client_id, msg } => {
                info!("{:?}>{:?}", client_id, msg);
                let Some(msg) = msg.variant else { continue };
                match msg {
                    proto::message::Variant::Join(j) => {
                        on_join(c, j, client_id);
                    }
                    Variant::CreateInstance(ci) => {
                        on_create_instance(c, ci, client_id);
                    }
                    _ => {}
                }
            }
        }
    }
}

#[tokio::main]
async fn main() {
    simple_logger::SimpleLogger::new()
        .with_level(LevelFilter::Debug)
        .init()
        .unwrap();
    let mut context = Context {
        server: Default::default(),
        instances: HashMap::default(),
        players: Default::default(),
    };

    context.server.start(8080).await;

    loop {
        tick(&mut context);
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
