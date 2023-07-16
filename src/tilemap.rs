use crate::prelude::TextureHandle;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use ldtk_rust::Project;

pub struct TileMapPlugin;

#[derive(Resource)]
pub struct TileMapMap {}

impl TileMapMap {
  pub fn load_from_ldtk(file: &str, commands: &mut Commands, texture_handle: &TextureHandle) -> Self {
    let tilemap_entity = commands.spawn_empty().id();

    let ldtk = Project::new(file);
    let layers = ldtk.levels[0].layer_instances.as_ref().unwrap();
    let layer = &layers[0];
    let enums = &ldtk.defs.tilesets[0].enum_tags;

    let mut tile_storage = TileStorage::empty(TilemapSize { x: 16, y: 16 });

    for ix in 0..tile_storage.size.x {
      for iy in 0..tile_storage.size.y {
        let tile = &layer.grid_tiles[(iy * 16 + ix) as usize];
        // let pas = &enums[0];
        // let sol = &enums[1];
        let tile_pos = TilePos::new(ix, iy);
        tile_storage.set(
          &tile_pos,
          commands
            .spawn(TileBundle {
              position: tile_pos,
              tilemap_id: TilemapId(tilemap_entity),
              texture_index: TileTextureIndex(tile.t as u32),
              ..default()
            })
            .id(),
        );
      }
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
  fn build(&self, app: &mut App) {
    // app.insert_resource(TileMapMap::with_size(10, 10));
  }
}
