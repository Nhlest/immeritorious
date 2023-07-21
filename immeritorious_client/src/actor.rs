use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use immeritorious_common::units::{Unit, UnitType};

pub fn into_sprite_sheet_bundle(
  unit_type: UnitType,
  texture_atlas_handle: Handle<TextureAtlas>,
  transform: Transform,
) -> SpriteSheetBundle {
  let tile = match unit_type {
    UnitType::Soldier => 85,
    UnitType::Farmer => 83,
  };
  SpriteSheetBundle {
    sprite: TextureAtlasSprite::new(tile),
    texture_atlas: texture_atlas_handle,
    transform,
    ..default()
  }
}

pub fn spawn_unit(
  commands: &mut Commands,
  texture_atlas_handle: Handle<TextureAtlas>,
  unit: Unit,
  location: (u32, u32),
) {
  commands.spawn((
    into_sprite_sheet_bundle(
      unit.t,
      texture_atlas_handle,
      Transform::from_xyz(
        location.0 as f32 * 16.0 - 16 as f32 * 8.0 + 8.0,
        location.1 as f32 * 16.0 - 16 as f32 * 8.0 + 8.0,
        1.0,
      ),
    ),
    unit,
    TilePos::new(location.0, location.1),
  ));
}
