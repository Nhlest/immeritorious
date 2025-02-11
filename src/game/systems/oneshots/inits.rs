use crate::game::components::cell::{CellBundle, Transform};
use crate::game::components::province::ProvinceBundle;
use bevy_ecs::prelude::World;
use crate::game::components::actor::Actor;

pub fn spawn_map(world: &mut World) {
  world.spawn(CellBundle::new(Transform::zero(), ProvinceBundle::new("Beijing".to_string(), 5, 10)));
  world.spawn(CellBundle::new(Transform::new(150.0, 150.0, 0.0), ProvinceBundle::new("Nanking".to_string(), 1, 15)));

  world.spawn(Actor::new("Xi Jiang".to_string(), 5));
}
