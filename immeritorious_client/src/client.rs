use crate::actor::spawn_unit;
use crate::game::ImmeritoriousState;
use crate::tilemap::{TextureAtlasHandle, TextureHandle, TileMapMap};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use bincode::deserialize;
use immeritorious_common::netcode::{ServerMessage, PROTOCOL_ID};
use std::net::UdpSocket;
use std::time::SystemTime;

#[derive(Resource, Deref, DerefMut)]
pub struct ServerEntities(pub HashMap<Entity, Entity>);

pub struct ImmeritoriousClientPlugin;

impl Plugin for ImmeritoriousClientPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      Self::update_system_pre_init.run_if(in_state(ImmeritoriousState::Connecting)),
    );
    app.add_systems(
      Update,
      Self::update_system_post_init.run_if(in_state(ImmeritoriousState::ConnectedInGame)),
    );
    app.insert_resource(ServerEntities(HashMap::new()));
  }
}

impl ImmeritoriousClientPlugin {
  fn update_system_pre_init(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut next_state: ResMut<NextState<ImmeritoriousState>>,
    texture_handle: Res<TextureHandle>,
    texture_atlas_handle: Res<TextureAtlasHandle>,
    mut server_entities: ResMut<ServerEntities>,
  ) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
      let message: ServerMessage = deserialize(&message).unwrap();
      match message {
        ServerMessage::InitSession { map, units } => {
          TileMapMap::load_from_network(map, &mut commands, texture_handle.as_ref());
          for (entity, unit, pos) in units.into_iter() {
            let local_entity = spawn_unit(
              &mut commands,
              texture_atlas_handle.0.clone(),
              unit,
              (pos.0 .0, pos.0 .1),
            );
            server_entities.insert(entity, local_entity);
          }
          next_state.set(ImmeritoriousState::ConnectedInGame);
        }
        _ => {}
      }
    }
  }
  fn update_system_post_init(
    mut client: ResMut<RenetClient>,
    server_entities: ResMut<ServerEntities>,
    mut positions: Query<&mut TilePos>,
  ) {
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
      let message: ServerMessage = deserialize(&message).unwrap();
      match message {
        ServerMessage::InitSession { .. } => {}
        ServerMessage::UpdateFrame { units } => {
          for (server_entity, pos) in units {
            let mut tile_pos = positions
              .get_mut(*server_entities.get(&server_entity).unwrap())
              .unwrap();
            tile_pos.x = pos.0 .0;
            tile_pos.y = pos.0 .1;
          }
        }
      }
    }
  }
}

pub fn new_renet_client(ip: &str) -> (RenetClient, NetcodeClientTransport) {
  let client = RenetClient::new(ConnectionConfig::default());

  let server_addr = ip.parse().unwrap();
  let socket = UdpSocket::bind("127.0.0.1:0").unwrap();
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();
  let client_id = current_time.as_millis() as u64;
  let authentication = ClientAuthentication::Unsecure {
    client_id,
    protocol_id: PROTOCOL_ID,
    server_addr,
    user_data: None,
  };

  let transport = NetcodeClientTransport::new(current_time, authentication, socket).unwrap();

  (client, transport)
}
