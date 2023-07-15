use bevy_ecs::prelude::{Schedule, World};
use crate::game::game::Schedul;
use crate::game::resources::state::State;

pub struct GameState {
  pub world: World,
}

impl GameState {
  pub fn new() -> Self {
    let mut world = World::new();
    world.insert_resource(State::MainMenu);
    let sch = Schedule::new(Schedul);
    world.add_schedule(sch);
    Self {
      world,
    }
  }
}
