use crate::client::ServerEntities;
use crate::game::{Cursor, ImmeritoriousState};
use crate::side::{MySide, UnitSelection};
use crate::tilemap::distance;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_renet::renet::RenetClient;
use immeritorious_common::netcode::{ClientMessage, Pos, Sendable};
use immeritorious_common::units::{Side, Unit};

pub struct ImmeritoriousInputPlugin;

impl Plugin for ImmeritoriousInputPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (Self::cursor_input, Self::unit_selection_input).run_if(in_state(ImmeritoriousState::ConnectedInGame)),
    );
  }
}

impl ImmeritoriousInputPlugin {
  fn unit_selection_input(
    cursor: Query<&Transform, With<Cursor>>,
    input: Res<Input<KeyCode>>,
    mut unit_selection: ResMut<UnitSelection>,
    my_side: Res<MySide>,
    units: Query<(Entity, &TilePos, &Side), With<Unit>>,
  ) {
    let cursor = cursor.single();
    let cursor_tile_pos = TilePos::new(
      (cursor.translation.x / 16.0 + 8.0) as u32,
      (cursor.translation.y / 16.0 + 8.0) as u32,
    );
    if input.just_pressed(KeyCode::I) {
      if let Some(closest_own_unit) = units
        .iter()
        .filter(|(_, _, side)| *side == &my_side.0)
        .filter(|(entity, _, _)| !unit_selection.contains(entity))
        .map(|(entity, pos, _)| (entity, distance(pos, &cursor_tile_pos)))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .map(|(entity, _)| entity)
      {
        unit_selection.units.push(closest_own_unit);
      }
    }
    if input.just_pressed(KeyCode::O) {
      unit_selection.units.clear();
    }
  }
  fn cursor_input(
    mut cursor: Query<&mut Transform, With<Cursor>>,
    input: Res<Input<KeyCode>>,
    mut client: ResMut<RenetClient>,
    unit_selection: Res<UnitSelection>,
    server_entities: Res<ServerEntities>,
  ) {
    let mut cursor = cursor.single_mut();
    let (mut cx, mut cy) = (
      (cursor.translation.x / 16.0 + 8.0) as i32,
      (cursor.translation.y / 16.0 + 8.0) as i32,
    );
    if input.just_pressed(KeyCode::Q) {
      cx -= 1;
      cy += 1;
    }
    if input.just_pressed(KeyCode::W) {
      cy += 1;
    }
    if input.just_pressed(KeyCode::E) {
      cx += 1;
      cy += 1;
    }
    if input.just_pressed(KeyCode::A) {
      cx -= 1;
    }
    if input.just_pressed(KeyCode::D) {
      cx += 1;
    }
    if input.just_pressed(KeyCode::Z) {
      cx -= 1;
      cy -= 1;
    }
    if input.just_pressed(KeyCode::X) {
      cy -= 1;
    }
    if input.just_pressed(KeyCode::C) {
      cx += 1;
      cy -= 1;
    }
    if input.just_pressed(KeyCode::Space) {
      client.send(&ClientMessage::MoveTo(
        unit_selection
          .units
          .iter()
          .map(|e| server_entities.get_by_right(e))
          .flatten()
          .cloned()
          .collect(),
        Pos((cx as u32, cy as u32)),
      ));
    }
    cursor.translation = Vec3::new((cx as f32 - 8.0) * 16.0 + 8.0, (cy as f32 - 8.0) * 16.0 + 8.0, 2.0);
  }
}
