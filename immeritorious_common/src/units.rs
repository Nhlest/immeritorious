use bevy::prelude::*;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum UnitType {
  Soldier,
  Farmer,
}

#[derive(Component)]
pub struct ActorMarker;

#[derive(Component)]
pub struct Building;

#[derive(Debug, Component, Serialize, Deserialize, Clone)]
pub struct Unit {
  pub t: UnitType,
}
