use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use immeritorious_common::netcode::{Pos, Tile};
use immeritorious_common::Passibility;
use ldtk_rust::Project;

pub struct ServerMap {}

impl ServerMap {
  pub fn load_from_ldtk(file: &str, commands: &mut Commands) -> Self {
    let tilemap_entity = commands.spawn_empty().id();

    let ldtk = Project::new(file);
    let layers = ldtk.levels[0].layer_instances.as_ref().unwrap();
    let layer = &layers[0];
    let enums = &ldtk.defs.tilesets[0].enum_tags;

    let mut tile_storage = TileStorage::empty(TilemapSize { x: 16, y: 16 });

    for ix in 0..tile_storage.size.x {
      for iy in 0..tile_storage.size.y {
        let tile = &layer.grid_tiles[(iy * 16 + ix) as usize];
        let sol = &enums[1];
        let tile_pos = TilePos::new(ix, iy);
        let pass = if sol.tile_ids.contains(&tile.t) {
          Passibility::Solid
        } else {
          Passibility::Passable
        };
        tile_storage.set(
          &tile_pos,
          commands.spawn((Tile { tile: tile.t }, Pos((ix, iy)), pass)).id(),
        );
      }
    }
    commands.entity(tilemap_entity).insert(tile_storage);
    Self {}
  }
}
