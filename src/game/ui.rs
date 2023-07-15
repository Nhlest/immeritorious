use crate::game::state::GameState;

impl easy_imgui::UiBuilder for GameState {
  fn do_ui(&mut self, ui: &easy_imgui::Ui<Self>) {
    use easy_imgui::*;
    for card in &self.cards {
      ui.window_config(lbl(&card.name))
        .flags(WindowFlags::NoResize)
        .with(|| {
          ui.button(lbl("T"));
        });
    }
  }
}

