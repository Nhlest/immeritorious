use bevy::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum UnitType {
  Soldier,
  Farmer,
}

#[derive(Component)]
pub struct ActorMarker;

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Unit {
  pub t: UnitType,
}

impl Unit {
  pub fn get_max_hp(&self) -> u16 {
    match self.t {
      UnitType::Soldier => 32,
      UnitType::Farmer => 17,
    }
  }
}

#[derive(Debug, Component, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub struct Side(pub u8);

#[derive(Debug, Clone, Serialize, Deserialize, Component)]
pub struct HP(pub u16);
