use crate::prelude::TextureHandle;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use immeritorious_common::netcode::{Pos, Tile};
use immeritorious_common::Passibility;

pub struct TileMapPlugin;

#[derive(Resource)]
pub struct TileMapMap {}

impl TileMapMap {
  pub fn load_from_network(
    tiles: Vec<(Tile, Pos, Passibility)>,
    commands: &mut Commands,
    texture_handle: &TextureHandle,
  ) -> Self {
    let tilemap_entity = commands.spawn_empty().id();

    let mut tile_storage = TileStorage::empty(TilemapSize { x: 16, y: 16 });

    for (tile, pos, passibility) in tiles.into_iter() {
      let tile_pos = TilePos::new(pos.0 .0, pos.0 .1);
      tile_storage.set(
        &tile_pos,
        commands
          .spawn((
            TileBundle {
              position: tile_pos,
              tilemap_id: TilemapId(tilemap_entity),
              texture_index: TileTextureIndex(tile.tile as u32),
              ..default()
            },
            passibility,
          ))
          .id(),
      );
    }
    let grid_size = TilemapGridSize { x: 16.0, y: 16.0 };
    let tile_size = TilemapTileSize { x: 16.0, y: 16.0 };
    let size = tile_storage.size;
    commands.entity(tilemap_entity).insert(TilemapBundle {
      transform: get_tilemap_center_transform(&tile_storage.size, &grid_size, &TilemapType::default(), 0.0),
      grid_size,
      size,
      tile_size,
      storage: tile_storage,
      texture: TilemapTexture::Single(texture_handle.0.clone()),
      ..default()
    });
    Self {}
  }
}

impl Plugin for TileMapPlugin {
  fn build(&self, _app: &mut App) {
    // app.insert_resource(TileMapMap::with_size(10, 10));
  }
}
