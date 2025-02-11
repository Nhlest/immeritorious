use crate::game::components::cell::{Cell, Transform};
use crate::game::components::province::{Province, ProvinceBundle, Workers};
use crate::game::resources::game_state::{GameState, I};
use bevy_ecs::prelude::*;
use easy_imgui::{lbl, Color, ColorId, DragDropPayloadCond, DragDropSourceFlags, DrawFlags, ItemId, StyleValue, StyleVar, Ui, Vector2};
use easy_imgui::cgmath::num_traits::float::FloatCore;
use crate::game::components::actor;
use crate::game::components::actor::Actor;

pub fn actors_ui(world: &mut World, ui: &Ui<GameState>) {
  ui.window_config(lbl("Actors")).with(|| {
    for (entity, actor) in world.query::<(Entity, &Actor)>().iter_mut(world) {
      ui.button(lbl(actor.name.as_str()));
      ui.with_drag_drop_source(DragDropSourceFlags::NoDisableHover, |d| {
        ui.text("Attempt influence");
        d.set("oke", &entity.to_bits().to_le_bytes(), DragDropPayloadCond::Once);
      });
      ui.same_line();
      for i in 0..actor.energy.max {
        ui.with_push((ItemId(i as usize), (StyleVar::ItemSpacing, StyleValue::Vec2(Vector2::new(1.0, 0.0)))), || {
          ui.set_next_item_width(5.0);
          ui.with_push((ColorId::Button, if i < actor.energy.current { Color::YELLOW } else { Color::BLACK }), || {
            ui.button(lbl(""));
            ui.same_line();
          });
        })
      }
    }
  });
}

pub fn game_ui(world: &mut World, ui: &Ui<GameState>) {
  let i = &world.get_resource::<I>().unwrap().0;
  ui.background_draw_list().add_image(*i, Vector2::new(0.0, 0.0), Vector2::new(800.0, 600.0), Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), Color::WHITE);
  // ui.background_draw_list().add_bezier_quadratic(Vector2::new(100.0, 100.0), Vector2::new(50.0, 150.0), Vector2::new(400.0, 400.0), Color::CYAN, 2.0, 100);
  let mut coords = vec![];
  actors_ui(world, ui);
  let mut query = world.query::<&mut Actor>();
  let cel = world.as_unsafe_world_cell();
  for (transform, province, workers) in unsafe { cel.world_mut() }.query::<(&Transform, &Province, &mut Workers)>().iter_mut(unsafe { cel.world_mut() } ) {
    let (mn, mx) = ProvinceBundle::render(ui, province, workers, transform, &mut query, cel);
    coords.push((mn, mx));
    ui.foreground_draw_list().add_rect(mn, mx, Color::RED, 1.0, DrawFlags::None, 3.0);
  }
  let first = coords[0];
  let second = coords[1];
  let first_size = first.1 - first.0;
  let second_size = second.1 - second.0;
  let mut right_middle = first.0 + first_size;
  right_middle.y -= (first_size.y / 2.0);
  let mut left_middle = second.0;
  left_middle.y += (second_size.y / 2.0);
  let dif = right_middle.x - left_middle.x;
  let middle_point_1 = right_middle + Vector2::new(200.0_f32.clamp(0.0, dif.clamp(0.0, dif.abs())), 0.0);
  let middle_point_2 = left_middle - Vector2::new(200.0_f32.clamp(0.0, dif.clamp(0.0, dif.abs())), 0.0);
  ui.foreground_draw_list().add_bezier_cubic(right_middle, middle_point_1, middle_point_2, left_middle, Color::BLACK, 1.0, 50);
  world.change_tick();
}

// -- Time capsule for the draggable logic
// let mut event = None;

// ui.set_next_window_pos(Vector2::new(transform.x + drag.x, transform.y + drag.y), Cond::Always, Vector2::new(0.0, 0.0));
// ui.with_push(((ColorId::WindowBg, Color::new(cell.cell.0, cell.cell.1, cell.cell.2, 1.0)), (StyleVar::WindowPadding, StyleValue::Vec2(Vector2::new(0.0, 0.0)))), || {
//   ui.window_config(lbl(entity.to_string())).flags(WindowFlags::NoTitleBar | WindowFlags::NoMove | WindowFlags::NoResize).with(|| {
//     ui.child_config(lbl("dnd")).with(|| {
//       ui.with_drag_drop_source(DragDropSourceFlags::None, |d| {
//         ui.button(lbl("!"));
//         d.set("oke", &entity.to_bits().to_le_bytes(), DragDropPayloadCond::Once);
//       });
//     });
//     ui.with_drag_drop_target(|t|{
//       if let Some(a) = t.by_type("oke", DragDropAcceptFlags::None) {
//         let entity_source = Entity::from_bits(u64::from_le_bytes(a.data().try_into().unwrap()));
//         event = Some(Dropped(entity_source, entity));
//       }
//     });
//     if !ui.is_mouse_dragging(MouseButton::Left) {  } // Save drag and preserve it while mouse is dragging
//     if ui.is_window_hovered(FocusedFlags::RootAndChildWindows) && ui.is_mouse_dragging(MouseButton::Left) {
//       // drag.x = ui.get_mouse_drag_delta(MouseButton::Left).x;
//       // drag.y = ui.get_mouse_drag_delta(MouseButton::Left).y;
//     } else {
//       drag.x = 0.0;
//       drag.y = 0.0;
//     }
//   });
// });

// event.map(|event| { world.send_event(event) });
// let a = world.run_system_once(dropped_events).unwrap();
