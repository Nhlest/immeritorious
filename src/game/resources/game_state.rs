use bevy_ecs::event::Events;
use bevy_ecs::prelude::{Schedule, World};
use crate::game::game::Schedul;
use crate::game::resources::state::State;
use crate::game::systems::ui::game_ui::{dropped_events, Dropped};

pub struct GameState {
  pub world: World,
}

impl GameState {
  pub fn new() -> Self {
    let mut world = World::new();
    world.insert_resource(State::MainMenu);
    let mut sch = Schedule::new(Schedul);
    sch.add_systems(dropped_events);
    world.add_schedule(sch);
    world.insert_resource(Events::<Dropped>::default());
    Self {
      world,
    }
  }
}
