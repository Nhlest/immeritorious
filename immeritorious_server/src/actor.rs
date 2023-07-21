use crate::server::Tick;
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;

#[derive(Component)]
pub struct Cooldown(pub u64);

#[derive(Default)]
pub enum BrainState {
  #[default]
  Idle,
  MovingTo(TilePos),
}

#[derive(Component, Default)]
pub struct Brain {
  pub state: BrainState,
}

pub struct CooldownCommand(pub u64);

impl EntityCommand for CooldownCommand {
  fn apply(self, id: Entity, world: &mut World) {
    let wait_till = world.resource::<Tick>().0 + self.0;
    world.entity_mut(id).insert(Cooldown(wait_till));
  }
}
