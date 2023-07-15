use bevy_ecs::prelude::World;
use easy_imgui::{lbl, Ui};
use crate::game::resources::game_state::GameState;
use crate::game::resources::state::State;
use crate::game::systems::oneshots::inits::spawn_map;

pub fn main_menu_ui(world: &mut World, ui: &Ui<GameState>) {
  ui.window_config(lbl("Test")).with(||{
    if ui.button(lbl("New Game")) {
      world.insert_resource(State::Game);
      spawn_map(world);
    }
  }).unwrap();
}

