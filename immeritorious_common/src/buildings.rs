use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct Building {
  pub t: BuildingType,
}

#[derive(Clone)]
pub enum BuildingType {
  Sovereignty,
  Farm,
}
