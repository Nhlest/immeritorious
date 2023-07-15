use bevy_ecs::prelude::{Resource};

#[derive(Resource)]
pub enum State {
  MainMenu,
  Game
}

