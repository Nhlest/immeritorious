use crate::game::components::cell::{Cell, Transform};
use crate::game::components::province::{Province, ProvinceBundle, Workers};
use crate::game::resources::game_state::{GameState, I};
use bevy_ecs::prelude::*;
use easy_imgui::{Color, Ui, Vector2};

pub fn dropped_events(
  mut dropped: EventReader<Dropped>,
  mut cells: Query<&mut Cell>
) {
  for ev in dropped.read() {
    cells.get_mut(ev.1).unwrap().cell.2 = 0.7;
  }
}

#[derive(Event)]
#[allow(dead_code)]
pub struct Dropped(Entity, Entity);

pub fn game_ui(world: &mut World, ui: &Ui<GameState>) {
  let i = &world.get_resource::<I>().unwrap().0;
  ui.background_draw_list().add_image(*i, Vector2::new(0.0, 0.0), Vector2::new(800.0, 600.0), Vector2::new(0.0, 0.0), Vector2::new(1.0, 1.0), Color::WHITE);
  for (transform, province, workers) in world.query::<(&Transform, &Province, &mut Workers)>().iter_mut(world) {
    ProvinceBundle::render(ui, province, workers, transform);
  }
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
