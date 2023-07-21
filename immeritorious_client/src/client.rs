use crate::actor::spawn_unit;
use crate::prelude::{ImmeritoriousState, TextureAtlasHandle, TextureHandle};
use crate::tilemap::TileMapMap;
use bevy::prelude::*;
use bevy_renet::renet::transport::{ClientAuthentication, NetcodeClientTransport};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetClient};
use bincode::deserialize;
use immeritorious_common::netcode::{ServerMessage, PROTOCOL_ID};
use std::net::UdpSocket;
use std::time::SystemTime;

pub struct ImmeritoriousClientPlugin;

impl Plugin for ImmeritoriousClientPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      Self::update_system
        .run_if(in_state(ImmeritoriousState::Connecting).or_else(in_state(ImmeritoriousState::ConnectedInGame))),
    );
  }
}

impl ImmeritoriousClientPlugin {
  fn update_system(
    mut commands: Commands,
    mut client: ResMut<RenetClient>,
    mut next_state: ResMut<NextState<ImmeritoriousState>>,
    texture_handle: Res<TextureHandle>,
    texture_atlas_handle: Res<TextureAtlasHandle>,
  ) {
    // let message = bincode::serialize(&PlayerCommand::T).unwrap();
    // client.send_message(DefaultChannel::Unreliable, message);
    while let Some(message) = client.receive_message(DefaultChannel::ReliableOrdered) {
      let message: ServerMessage = deserialize(&message).unwrap();
      match message {
        ServerMessage::InitSession { map, units } => {
          TileMapMap::load_from_network(map, &mut commands, texture_handle.as_ref());
          for (unit, pos) in units.into_iter() {
            spawn_unit(
              &mut commands,
              texture_atlas_handle.0.clone(),
              unit,
              (pos.0 .0, pos.0 .1),
            );
          }
          next_state.set(ImmeritoriousState::ConnectedInGame);
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
