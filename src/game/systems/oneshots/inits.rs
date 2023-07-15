use crate::game::components::cell::{CellBundle, Transform};
use crate::game::components::province::ProvinceBundle;
use bevy_ecs::prelude::World;

pub fn spawn_map(world: &mut World) {
  world.spawn(CellBundle::new(Transform::zero(), ProvinceBundle::new("Beijing".to_string(), 5, 10)));
}
