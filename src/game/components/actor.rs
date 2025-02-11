use bevy_ecs::prelude::*;

pub struct Energy {
  pub current: u32,
  pub max: u32
}

#[derive(Component)]
pub struct Actor {
  pub name: String,
  pub energy: Energy
}

impl Actor {
  pub fn new(name: String, energy: u32) -> Self {
    Self {
      name, energy: Energy { current: energy, max: energy }
    }
  }
}