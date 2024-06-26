use log::*;
use netcode::server::{ClientId, Event, Server};
use proto::{message::Variant, CreateInstance, Join, Message, Welcome};
use std::{collections::HashMap, time::Duration};
use uuid::Uuid;

struct GameInstance {
    pub id: Uuid,
    pub name: String,
    pub creator: Uuid,
    pub recv:Vec<(Uuid, Variant)>,
    pub sent:Vec<(Uuid, Variant)>
}
impl GameInstance {
    pub fn push_msg(&mut self, player_id:Uuid, msg:Variant) {
        self.recv.push((player_id, msg));
    }
    pub fn pop_msgs(&mut self) -> Vec<(Uuid, Variant)> {
        let popped = self.sent.clone();
        self.sent.clear();
        return popped;
    }
}

struct Player {
    pub id: Uuid,
    pub client_id: Option<ClientId>,
    pub name: String,
    pub instance_id: Option<Uuid>,
}
trait Players {
    fn find_player(&self, client_id: ClientId) -> Option<&Player>;
    fn find_player_mut(&mut self, client_id: ClientId) -> Option<&mut Player>;
}
impl Players for HashMap<Uuid, Player> {
    fn find_player(&self, client_id: ClientId) -> Option<&Player> {
        self.iter().find(|x|x.1.client_id == Some(client_id)).map(|x|x.1)
    }
    
    fn find_player_mut(&mut self, client_id: ClientId) -> Option<&mut Player> {
        self.iter_mut().find(|x|x.1.client_id == Some(client_id)).map(|x|x.1)
    }
}

struct Context {
    pub server: Server<proto::Message>,
    pub instances: HashMap<Uuid, GameInstance>,
    pub players: HashMap<Uuid, Player>,
}

fn on_create_instance(c:&mut Context, ci:CreateInstance, client_id: ClientId) {
    let Some(player) = c.players.find_player_mut(client_id) else { return };
    match player.instance_id {
        Some(id) => {
            // TODO handle case where player has an instance
        },
        None => {
            let inst_id = Uuid::new_v4();
            let inst = GameInstance {
                id: inst_id,
                name: ci.name.clone(),
                creator: player.id.clone(),
                recv: Default::default(),
                sent: Default::default(),
            };
            c.instances.insert(inst_id, inst);
            player.instance_id = Some(inst_id);
            c.server.send(client_id, Variant::Welcome(Welcome { current_instance: inst_id.to_string() }).into());
        },
    }

}

fn on_refresh_lobby(c:&mut Context, client_id: ClientId) {
    let mut instances:proto::Instances = Default::default();
    c.instances.values().for_each(|i|{
        instances.instances.push(proto::Instance {
            id: i.id.to_string(),
            name: i.name.clone(),
        });
    });
    let variant:proto::Variant = Variant::Instances(instances);
    let msg = proto::Message::from(variant);
    c.server.send(client_id, msg);
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
                    Variant::RefreshLobby(_) => {
                        on_refresh_lobby(c, client_id);
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
