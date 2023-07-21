use crate::units::Unit;
use crate::Passibility;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
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
pub const RELIABLE_CHANNEL_MAX_LENGTH: u64 = 10240;

pub enum ServerChannel {
  GameEvent,
  GameFrame,
}

#[derive(Component, Debug, Deserialize, Serialize, Clone, Copy)]
pub struct PolarRotation {
  pub phi: f32,
  pub theta: f32,
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
    units: Vec<(Entity, Unit, Pos)>,
  },
  UpdateFrame {
    units: Vec<(Entity, Pos)>,
  },
}

impl ServerMessage {
  pub fn cast(&self) -> Vec<u8> {
    serialize(self).unwrap()
  }
}

// impl Display for ServerMessage {
//   fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
//     match self {
//       ServerMessage::PlayerSpawn { .. } => f.write_str("PlayerSpawn"),
//     }
//   }
// }

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct NetworkFrame {
  pub tick: u32,
}

pub enum ClientChannel {
  ClientCommand,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum PlayerCommand {
  MoveTo(Pos),
}
impl PlayerCommand {
  pub fn cast(&self) -> Vec<u8> {
    serialize(self).unwrap()
  }
}
