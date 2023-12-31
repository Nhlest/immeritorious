use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use immeritorious_common::units::{Side, Unit, UnitType};

pub trait RenderableSprite {
  fn sprite_id(&self, side: &Side) -> usize;
  fn into_sprite_sheet_bundle(
    &self,
    texture_atlas_handle: Handle<TextureAtlas>,
    transform: Transform,
    side: Side,
  ) -> SpriteSheetBundle;
  fn spawn_renderable(
    &self,
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    side: Side,
    location: (u32, u32),
  ) -> Entity;
}

impl RenderableSprite for Unit {
  fn sprite_id(&self, side: &Side) -> usize {
    match self.t {
      UnitType::Soldier => 83 + 23 * side.0 as usize,
      UnitType::Farmer => 82 + 23 * side.0 as usize,
    }
  }
  fn into_sprite_sheet_bundle(
    &self,
    texture_atlas_handle: Handle<TextureAtlas>,
    transform: Transform,
    side: Side,
  ) -> SpriteSheetBundle {
    let tile = self.sprite_id(&side);
    SpriteSheetBundle {
      sprite: TextureAtlasSprite::new(tile),
      texture_atlas: texture_atlas_handle,
      transform,
      ..default()
    }
  }
  fn spawn_renderable(
    &self,
    commands: &mut Commands,
    texture_atlas_handle: Handle<TextureAtlas>,
    side: Side,
    location: (u32, u32),
  ) -> Entity {
    commands
      .spawn((
        self.into_sprite_sheet_bundle(
          texture_atlas_handle,
          Transform::from_xyz(
            location.0 as f32 * 16.0 - 16.0 * 8.0 + 8.0,
            location.1 as f32 * 16.0 - 16.0 * 8.0 + 8.0,
            1.0,
          ),
          side,
        ),
        self.clone(),
        side,
        TilePos::new(location.0, location.1),
      ))
      .id()
  }
}
