use crate::actor::RenderableSprite;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;
use immeritorious_common::buildings::{Building, BuildingType};
use immeritorious_common::units::Side;

impl RenderableSprite for Building {
  fn sprite_id(&self, _side: &Side) -> usize {
    match self.t {
      BuildingType::Sovereignty => 7 * 23 + 5,
      BuildingType::Farm => 7 * 23 + 6,
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
