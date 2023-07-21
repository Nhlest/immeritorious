use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use immeritorious_common::units::{Side, Unit, UnitType};

pub fn into_sprite_sheet_bundle(
  unit_type: UnitType,
  texture_atlas_handle: Handle<TextureAtlas>,
  transform: Transform,
  side: Side,
) -> SpriteSheetBundle {
  let tile = match unit_type {
    UnitType::Soldier => 83 + 23 * side.0 as usize,
    UnitType::Farmer => 82 + 23 * side.0 as usize,
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
  side: Side,
  location: (u32, u32),
) -> Entity {
  commands
    .spawn((
      into_sprite_sheet_bundle(
        unit.t,
        texture_atlas_handle,
        Transform::from_xyz(
          location.0 as f32 * 16.0 - 16.0 * 8.0 + 8.0,
          location.1 as f32 * 16.0 - 16.0 * 8.0 + 8.0,
          1.0,
        ),
        side,
      ),
      unit,
      side,
      TilePos::new(location.0, location.1),
    ))
    .id()
}
