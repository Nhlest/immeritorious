use crate::client::ServerEntities;
use crate::game::{Cursor, ImmeritoriousState};
use crate::side::{MySide, PrimeSelection, UnitSelection};
use crate::tilemap::{distance_manhattan, SpriteSheetAtlasHandle};
use crate::ui::{UnitButton, UnitButtonRow};
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_renet::renet::RenetClient;
use immeritorious_common::netcode::{ClientMessage, Pos, Sendable};
use immeritorious_common::units::{Side, Unit};

pub struct ImmeritoriousInputPlugin;

#[derive(Hash, Debug, PartialEq, Eq, Clone, SystemSet)]
pub struct InputSet;

impl Plugin for ImmeritoriousInputPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(
      Update,
      (Self::cursor_input, Self::unit_selection_input)
        .run_if(in_state(ImmeritoriousState::ConnectedInGame))
        .in_set(InputSet),
    );
  }
}

impl ImmeritoriousInputPlugin {
  fn unit_selection_input(
    mut commands: Commands,
    cursor: Query<&Transform, With<Cursor>>,
    input: Res<Input<KeyCode>>,
    mut unit_selection: ResMut<UnitSelection>,
    prime_selection: Option<Res<PrimeSelection>>,
    my_side: Res<MySide>,
    units: Query<(Entity, &TilePos, &Side), With<Unit>>,
    unit_buttons: Query<(Entity, &UnitButton)>,
    unit_buttons_container: Query<Entity, With<UnitButtonRow>>,
    texture_atlas_handle: Res<SpriteSheetAtlasHandle>,
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
        .map(|(entity, pos, _)| (entity, distance_manhattan(pos, &cursor_tile_pos)))
        .min_by(|(_, d1), (_, d2)| d1.cmp(d2))
        .map(|(entity, _)| entity)
      {
        let unit_buttons_container_entity = unit_buttons_container.single();
        unit_selection.units.push(closest_own_unit);
        commands.entity(unit_buttons_container_entity).with_children(|c| {
          UnitButton(closest_own_unit).spawn(c, texture_atlas_handle.0.clone());
        });
        commands.insert_resource(PrimeSelection(closest_own_unit));
      }
    }
    if input.just_pressed(KeyCode::O) {
      commands.remove_resource::<PrimeSelection>();
      unit_selection.units.clear();
      unit_buttons
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn_recursive());
    }
    if input.just_pressed(KeyCode::Comma) {
      if !unit_selection.units.is_empty() {
        let mut prime_selection = prime_selection
          .as_ref()
          .map(|prime_selection_resource| {
            let e = prime_selection_resource.0;
            unit_selection
              .units
              .iter()
              .position(|s| *s == e)
              .unwrap_or(unit_selection.len() - 1)
          })
          .unwrap_or(unit_selection.units.len() - 1)
          - 1;
        if prime_selection >= unit_selection.units.len() {
          prime_selection = unit_selection.units.len() - 1;
        }
        commands.insert_resource(PrimeSelection(unit_selection.units[prime_selection]));
      }
    }
    if input.just_pressed(KeyCode::Period) {
      if !unit_selection.units.is_empty() {
        let mut prime_selection = prime_selection
          .as_ref()
          .map(|prime_selection_resource| {
            let e = prime_selection_resource.0;
            unit_selection
              .units
              .iter()
              .position(|s| *s == e)
              .unwrap_or(unit_selection.len() - 1)
          })
          .unwrap_or(unit_selection.units.len() - 1)
          + 1;
        if prime_selection >= unit_selection.units.len() {
          prime_selection = 0;
        }
        commands.insert_resource(PrimeSelection(unit_selection.units[prime_selection]));
      }
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
