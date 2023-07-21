use bevy::prelude::*;
use serde_derive::{Deserialize, Serialize};

pub mod netcode;
pub mod units;

#[derive(Debug, Serialize, Deserialize, Component, PartialEq, Clone)]
pub enum Passibility {
  Passable,
  Solid,
}
