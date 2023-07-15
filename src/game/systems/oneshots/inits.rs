use bevy_ecs::prelude::World;
use crate::game::components::cell::{Cell, CellTransform, Transform};

pub fn spawn_map(world: &mut World) {
  world.spawn(CellTransform::new(Transform::zero(), Cell { cell: (1.0, 0.0, 0.0), name: "RED".to_string() }));
  world.spawn(CellTransform::new(Transform::new(100.0, 0.0, 0.0), Cell { cell: (0.0, 0.0, 1.0), name: "BLUE".to_string() }));
  world.spawn(CellTransform::new(Transform::new(100.0, 100.0, 0.0), Cell { cell: (0.0, 1.0, 1.0), name: "KEK".to_string() }));
}
