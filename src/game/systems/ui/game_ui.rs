use bevy_ecs::prelude::World;
use easy_imgui::{lbl, Color, ColorId, Cond, FocusedFlags, MouseButton, Ui, Vector2, WindowFlags};
use crate::game::components::cell::{Cell, Drag, Transform};
use crate::game::resources::game_state::GameState;

pub fn game_ui(world: &mut World, ui: &Ui<GameState>) {
  for (transform, cell, mut drag) in world.query::<(&Transform, &Cell, &mut Drag)>().iter_mut(world) {
    ui.set_next_window_pos(Vector2::new(transform.x + drag.x, transform.y + drag.y), Cond::Always, Vector2::new(0.0, 0.0));
    ui.with_push((ColorId::WindowBg, Color::new(cell.cell.0, cell.cell.1, cell.cell.2, 1.0)), || {
      ui.window_config(lbl(cell.name.as_str())).flags(WindowFlags::NoTitleBar | WindowFlags::NoMove | WindowFlags::NoResize).with(|| {
        if !ui.is_mouse_dragging(MouseButton::Left) {  } // Save drag and preserve it while mouse is dragging
        if ui.is_window_hovered(FocusedFlags::RootAndChildWindows) && ui.is_mouse_dragging(MouseButton::Left) {
          drag.x = ui.get_mouse_drag_delta(MouseButton::Left).x;
          drag.y = ui.get_mouse_drag_delta(MouseButton::Left).y;
        } else {
          drag.x = 0.0;
          drag.y = 0.0;
        }
      });
    });
  }
}

