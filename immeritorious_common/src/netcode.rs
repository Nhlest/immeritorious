use crate::units::Unit;
use crate::Passibility;
use bevy::prelude::*;
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

#[derive(Debug, Component, Serialize, Deserialize, Clone, Deref)]
pub struct Pos(pub (u32, u32));

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Tile {
  pub tile: i64,
}

#[derive(Debug, Serialize, Deserialize, Component)]
pub enum ServerMessage {
  InitSession {
    map: Vec<(Tile, Pos, Passibility)>,
    units: Vec<(Unit, Pos)>,
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
pub enum PlayerCommand {}
