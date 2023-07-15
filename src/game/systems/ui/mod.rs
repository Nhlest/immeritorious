use crate::game::resources::game_state::GameState;
use crate::game::resources::state::State;
use crate::game::systems::ui::game_ui::game_ui;
use crate::game::systems::ui::main_menu_ui::main_menu_ui;
use easy_imgui::{Ui, UiBuilder};

pub mod game_ui;
pub mod main_menu_ui;

impl UiBuilder for GameState {
  fn do_ui(&mut self, ui: &Ui<Self>) {
    let world = &mut self.world;
    match world.resource::<State>() {
      State::MainMenu => {
        main_menu_ui(world, ui);
      }
      State::Game => {
        game_ui(world, ui);
      }
    }
  }
}
