use crate::game::game::Schedul;
use crate::game::resources::state::State;
use crate::game::systems::ui::game_ui::{dropped_events, Dropped};
use bevy_ecs::event::Events;
use bevy_ecs::prelude::{Schedule, World};
use bevy_ecs::system::Resource;
use easy_imgui::TextureId;

pub struct GameState {
  pub world: World,
}

#[derive(Resource)]
pub struct I(pub TextureId);

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
