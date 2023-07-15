use crate::array::Array;
use crate::prelude::TextureAtlasHandle;
use bevy::prelude::*;
use ldtk_rust::{EnumTagValue, LayerInstance};

pub struct TileMapPlugin;

#[derive(Resource)]
pub struct TileMapMap {
  map: Array<Entity>,
}

impl TileMapMap {
  pub fn with_size(
    x: usize,
    y: usize,
    commands: &mut Commands,
    texture_atlas_handle: &TextureAtlasHandle,
    layer: &LayerInstance,
    enums: &Vec<EnumTagValue>,
  ) -> Self {
    let mut array = Array::populate(x, y, Entity::from_bits(0));
    for ix in 0..x {
      for iy in 0..y {
        let tile = &layer.grid_tiles[iy * 16 + ix];
        // let pas = &enums[0];
        // let sol = &enums[1];
        array[(ix, iy)] = commands
          .spawn(SpriteSheetBundle {
            sprite: TextureAtlasSprite::new(tile.t as usize),
            texture_atlas: texture_atlas_handle.0.clone(),
            transform: Transform::from_xyz(
              ix as f32 * 16.0 - x as f32 * 8.0,
              iy as f32 * 16.0 - y as f32 * 8.0,
              0.0,
            ),
            ..default()
          })
          .id();
      }
    }
    Self { map: array }
  }
}

impl Plugin for TileMapPlugin {
  fn build(&self, app: &mut App) {
    // app.insert_resource(TileMapMap::with_size(10, 10));
  }
}
