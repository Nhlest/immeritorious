use crate::units::{Side, Unit, HP};
use crate::Passibility;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_renet::renet::{DefaultChannel, RenetClient, RenetServer};
use bincode::serialize;
use serde::{Deserialize, Serialize};

// -------------------------------------------------------------------------------------------
// -- ###  #    #   ###   ####   #####  #     #  #####  #    #  #####        #     #  ##### --
// --  #   #    #  #   #  #   #  #      ##   ##  #      #    #    #          ##   ##  #     --
// --  #   ##   #  #      #   #  #      # # # #  #      ##   #    #          # # # #  #     --
// --  #   # #  #  #      #   #  #      #  #  #  #      # #  #    #          #  #  #  #     --
// --  #   #  # #  #      ####   ####   #     #  ####   #  # #    #          #     #  ####  --
// --  #   #   ##  #      ##     #      #     #  #      #   ##    #          #     #  #     --
// --  #   #    #  #      # #    #      #     #  #      #    #    #          #     #  #     --
// --  #   #    #  #   #  #  #   #      #     #  #      #    #    #          #     #  #     --
// -- ###  #    #   ###   #   #  #####  #     #  #####  #    #    #          #     #  ##### --
// -------------------------------------------------------------------------------------------
pub const PROTOCOL_ID: u64 = 42;

pub trait Sendable {
  fn send<T: Serialize>(&mut self, _message: &T) {}
  fn send_to<T: Serialize>(&mut self, _client_id: u64, _message: &T) {}
  fn broadcast<T: Serialize>(&mut self, _message: &T) {}
}

impl Sendable for RenetClient {
  fn send<T: Serialize>(&mut self, message: &T) {
    self.send_message(DefaultChannel::ReliableOrdered, serialize(message).unwrap())
  }
}

impl Sendable for RenetServer {
  fn send_to<T: Serialize>(&mut self, client_id: u64, message: &T) {
    self.send_message(client_id, DefaultChannel::ReliableOrdered, serialize(message).unwrap())
  }
  fn broadcast<T: Serialize>(&mut self, message: &T) {
    self.broadcast_message(DefaultChannel::ReliableOrdered, serialize(message).unwrap())
  }
}

#[derive(Debug, Component, Serialize, Deserialize, Clone, Deref, PartialEq)]
pub struct Pos(pub (u32, u32));

impl Into<TilePos> for &Pos {
  fn into(self) -> TilePos {
    TilePos::new(self.0 .0, self.0 .1)
  }
}

impl From<TilePos> for Pos {
  fn from(value: TilePos) -> Self {
    Pos((value.x, value.y))
  }
}

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Tile {
  pub tile: i64,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessage {
  InitSession {
    map: Vec<(Tile, Pos, Passibility)>,
    units: Vec<(Entity, Side, Unit, HP, Pos)>,
    clients_side: Side,
  },
  UpdateFrame {
    units: Vec<(Entity, Pos)>,
  },
  ChangeHP {
    entity: Entity,
    new_hp: HP,
  },
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ClientMessage {
  Authenticate(String),
  MoveTo(Vec<Entity>, Pos),
}
