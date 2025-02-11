use bevy_ecs::component::Component;
use bevy_ecs::prelude::{Bundle, Entity, Mut, QueryState};
use bevy_ecs::world::unsafe_world_cell::UnsafeWorldCell;
use easy_imgui::{lbl, Color, ColorId, Cond, DragDropAcceptFlags, ItemId, StyleValue, StyleVar, Ui, Vector2, WindowFlags};
use easy_imgui::ColorId::DragDropTarget;
use crate::game::components::actor::Actor;
use crate::game::components::cell::Transform;
use crate::game::resources::game_state::GameState;

#[derive(Component)]
pub struct Province {
  pub name: String
}

impl Province {
  pub fn new(name: String) -> Self {
    Self {
      name: name.into()
    }
  }
}

#[derive(Component)]
pub struct Workers {
  pub current: u32,
  pub max: u32,
}

impl Workers {
  pub fn new(current: u32, max: u32) -> Self {
    Self {
      current, max
    }
  }
}

#[derive(Bundle)]
pub struct ProvinceBundle {
  pub province: Province,
  pub workers: Workers
}

impl ProvinceBundle {
  pub fn new(name: String, current: u32, max: u32) -> Self {
    Self {
      province: Province::new(name),
      workers: Workers::new(current, max),
    }
  }
}

const WINDOW_STYLES: ((ColorId, Color), (ColorId, Color), (StyleVar, StyleValue)) = (
  (ColorId::TitleBgActive, Color::new(0.2, 0.2, 0.2, 1.0)),
  (ColorId::TitleBg, Color::new(0.2, 0.2, 0.2, 1.0)),
  (StyleVar::ItemSpacing, StyleValue::Vec2(Vector2::new(2.0, 2.0)))
);

impl ProvinceBundle {
  pub fn render(ui: &Ui<GameState>, province: &Province, mut workers: Mut<Workers>, transform: &Transform, query: &mut QueryState<&mut Actor>, cel: UnsafeWorldCell) -> (Vector2, Vector2) {
    ui.set_next_window_pos(Vector2::new(transform.x, transform.y), Cond::FirstUseEver, Vector2::new(0.0, 0.0));
    let mut mn = Vector2::new(0.0, 0.0);
    let mut mx = Vector2::new(0.0, 0.0);
    ui.with_push(WINDOW_STYLES, ||ui.window_config(lbl(&province.name)).flags(WindowFlags::NoResize).with(|| {
      for i in 0..workers.max {
        ui.with_push(ItemId(i as usize), || {
          if if i < workers.current {
            ui.with_push(((ColorId::Button, Color::RED), (ColorId::ButtonHovered, Color::new(0.7, 0.1, 0.1, 1.0))), || ui.button(lbl(" ")))
          } else {
            ui.button(lbl(" "))
          } {
            workers.current = i + 1;
          }
        });
        ui.same_line();
      }
      if ui.button(lbl("-")) { workers.current -= 1; }
      ui.with_drag_drop_target(|d| {
        if let Some(a) = d.by_type("oke", DragDropAcceptFlags::None) {
          let entity = Entity::from_bits(u64::from_le_bytes(a.data().try_into().unwrap()));
          let mut actor = query.get_mut(unsafe { cel.world_mut() }, entity).unwrap();
          actor.energy.current -= 1;
          println!("{:?}", a.data());
          workers.current -= 1;
        }
      });
      if ui.is_item_hovered() { ui.with_tooltip(|| { ui.text("Reduce workers"); }); }
      ui.same_line();
      if ui.button(lbl("+")) { workers.current += 1; }
      ui.with_drag_drop_target(|d| {
        if let Some(a) = d.by_type("oke", DragDropAcceptFlags::None) {
          println!("{:?}", a.data());
          workers.current += 1;
        }
      });
      if ui.is_item_hovered() { ui.with_tooltip(|| { ui.text("Increase workers"); }); }

      mn = ui.get_window_pos();
      let wh = ui.get_window_height();
      let ww = ui.get_window_width();
      mx = Vector2::new(mn.x + ww, mn.y + wh);
    }));
    return (mn, mx);
  }
}