use crate::actor::{Brain, BrainState, Cooldown, CooldownCommand};
use crate::side::{ClientId, SideName};
use crate::tilemap::ServerMap;
use bevy::prelude::*;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::Neighbors;
use bevy_ecs_tilemap::prelude::*;
use bevy_renet::renet::transport::{NetcodeServerTransport, NetcodeTransportError, ServerAuthentication, ServerConfig};
use bevy_renet::renet::{ConnectionConfig, DefaultChannel, RenetServer, ServerEvent};
use immeritorious_common::netcode::{ClientMessage, Pos, Sendable, ServerMessage, Tile, PROTOCOL_ID};
use immeritorious_common::units::{Side, Unit, UnitType};
use immeritorious_common::Passibility;
use pathfinding::prelude::astar;
use std::net::UdpSocket;
use std::time::SystemTime;

pub struct ImmeritoriousServerPlugin;

#[derive(Resource, Deref, DerefMut)]
pub struct Tick(pub u64);

impl Plugin for ImmeritoriousServerPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<NetcodeTransportError>();

    app.add_systems(
      FixedUpdate,
      (
        Self::increment_tick,
        Self::clear_cooldowns,
        apply_deferred,
        (Self::process_connections, Self::update_system, Self::process_brains),
      )
        .chain(),
    );

    app.insert_resource(FixedTime::new_from_secs(1.0 / 20.0));

    app.add_systems(Startup, Self::initiate_map);
    app.insert_resource(Tick(0));
  }
}

impl ImmeritoriousServerPlugin {
  fn initiate_map(mut commands: Commands) {
    ServerMap::load_from_ldtk("immeritorious_client/assets/map.ldtk", &mut commands);
    commands.spawn((Side(0), Unit { t: UnitType::Soldier }, Pos((4, 4)), Brain::default()));
    commands.spawn((Side(0), Unit { t: UnitType::Soldier }, Pos((7, 3)), Brain::default()));
    commands.spawn((Side(1), Unit { t: UnitType::Soldier }, Pos((12, 10)), Brain::default()));
  }
  fn process_connections(
    mut commands: Commands,
    mut server: ResMut<RenetServer>,
    mut server_events: EventReader<ServerEvent>,
    tiles: Query<(&Tile, &Pos, &Passibility)>,
    units: Query<(Entity, &Side, &Unit, &Pos)>,
    tile_storage: Query<&TileStorage>,
    sides: Query<&Side, With<ClientId>>,
  ) {
    let tile_storage = tile_storage.single();
    for event in server_events.iter() {
      match event {
        ServerEvent::ClientConnected { client_id } => {
          let clients_side = sides
            .iter()
            .max_by(|s1, s2| s1.0.cmp(&s2.0))
            .map(|s| Side(s.0 + 1))
            .unwrap_or(Side(0));
          let tiles = tile_storage
            .iter()
            .flatten()
            .map(|e| tiles.get(*e).unwrap())
            .map(|(tile, pos, passibility)| (tile.clone(), pos.clone(), passibility.clone()))
            .collect::<Vec<_>>();
          let units = units
            .iter()
            .map(|(entity, side, unit, pos)| (entity, side.clone(), unit.clone(), pos.clone()))
            .collect::<Vec<_>>();
          let side_name = SideName(format!("<UNNAMED> {}", clients_side.0).to_string());
          println!("{} has connected", side_name.0);
          commands.spawn((ClientId(*client_id), clients_side.clone(), side_name));
          server.send_to(
            *client_id,
            &ServerMessage::InitSession {
              map: tiles,
              units,
              clients_side,
            },
          );
        }
        ServerEvent::ClientDisconnected { .. } => {}
      }
    }
  }
  fn update_system(
    mut server: ResMut<RenetServer>,
    units: Query<(Entity, &Side, &Unit, &Pos)>,
    brains: Query<(&mut Brain, &Side)>,
    sides: Query<(&Side, &ClientId)>,
    mut nicknames: Query<(&ClientId, &mut SideName)>,
  ) {
    for client_id in server.clients_id().into_iter() {
      while let Some(message) = server.receive_message(client_id, DefaultChannel::ReliableOrdered) {
        let player_message: ClientMessage = bincode::deserialize(&message).unwrap();
        match player_message {
          ClientMessage::MoveTo(units, pos) => {
            let side = sides
              .iter()
              .find(|(_, client_id_haystack)| client_id_haystack.0 as u64 == client_id)
              .map(|(side, _)| side)
              .unwrap();
            units
              .iter()
              .map(|unit| unsafe { brains.get_unchecked(*unit) }) // TODO: remove unsafe, simple iterator?
              .flatten()
              .filter(|(_, side_haystack)| *side_haystack == side)
              .for_each(|(mut brain, _)| brain.state = BrainState::MovingTo(TilePos::new(pos.0 .0, pos.0 .1)));
          }
          ClientMessage::Authenticate(nickname) => {
            let mut old_nickname = nicknames
              .iter_mut()
              .find(|(client_id_haystack, _)| client_id_haystack.0 == client_id)
              .map(|(_, nickname)| nickname)
              .unwrap();
            old_nickname.0 = nickname;
            println!("{} authenticated as {}", client_id, old_nickname.0);
          }
        }
      }
    }
    server.broadcast(&ServerMessage::UpdateFrame {
      units: units.iter().map(|(entity, _, _, pos)| (entity, pos.clone())).collect(),
    });
  }
  fn process_brains(
    mut commands: Commands,
    mut brains: Query<(Entity, &mut Brain, &mut Pos, &Unit), Without<Cooldown>>,
    tile_storage: Query<&TileStorage>,
    tiles: Query<&Passibility>,
  ) {
    let tile_storage = tile_storage.single();
    for (entity, mut brain, mut pos, _unit) in &mut brains {
      match brain.state {
        BrainState::Idle => {}
        BrainState::MovingTo(pos_to) => {
          // TODO: this is garbage
          let tile_pos: TilePos = pos.as_ref().into();
          let path = astar(
            &tile_pos,
            |a| {
              Neighbors::get_square_neighboring_positions(a, &TilemapSize { x: 16, y: 16 }, true)
                .iter()
                .filter(|a| tiles.get(tile_storage.get(a).unwrap()).unwrap() == &Passibility::Passable)
                .map(|x| {
                  (*x, {
                    let d = x.x.abs_diff(a.x) + x.y.abs_diff(a.y);
                    if d == 1 {
                      10
                    } else {
                      14
                    }
                  })
                })
                .collect::<Vec<_>>()
            },
            |a| 10 * (a.x.abs_diff(pos_to.x) + a.y.abs_diff(pos_to.y)),
            |a| *a == pos_to,
          );
          match path {
            None => {}
            Some((p, _)) if p.len() == 1 => {
              brain.state = BrainState::Idle;
            }
            Some((p, _)) => {
              let next = p[1];
              let d = tile_pos.x.abs_diff(next.x) + tile_pos.y.abs_diff(next.y);
              *pos = next.into();
              if pos.as_ref() == &pos_to.into() {
                brain.state = BrainState::Idle;
              }
              commands.entity(entity).add(CooldownCommand(if d == 1 { 5 } else { 7 }));
            }
          }
        }
      }
    }
  }
  fn increment_tick(mut tick: ResMut<Tick>) {
    tick.0 += 1;
  }
  fn clear_cooldowns(mut commands: Commands, cooldowns: Query<(Entity, &Cooldown)>, tick: Res<Tick>) {
    for (e, cd) in &cooldowns {
      if cd.0 <= tick.0 {
        commands.entity(e).remove::<Cooldown>();
      }
    }
  }
}

pub fn new_renet_server(ip: &str) -> (RenetServer, NetcodeServerTransport) {
  let server = RenetServer::new(ConnectionConfig::default());

  let public_addr = ip.parse().unwrap();
  let socket = UdpSocket::bind(public_addr).unwrap();
  let server_config = ServerConfig {
    max_clients: 64,
    protocol_id: PROTOCOL_ID,
    public_addr,
    authentication: ServerAuthentication::Unsecure,
  };
  let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap();

  let transport = NetcodeServerTransport::new(current_time, server_config, socket).unwrap();

  (server, transport)
}
