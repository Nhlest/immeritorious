use crate::game::ImmeritoriousState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use immeritorious_common::netcode::{Pos, Tile};
use immeritorious_common::Passibility;

#[derive(Resource, Deref)]
pub struct TextureHandle(pub Handle<Image>);

#[derive(Resource, Deref)]
pub struct TextureAtlasHandle(pub Handle<TextureAtlas>);

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(OnEnter(ImmeritoriousState::Connecting), Self::load_sprite_sheet);
  }
}

impl TileMapPlugin {
  fn load_sprite_sheet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
  ) {
    let texture_handle = asset_server.load("spritesheet_01.png");

    let texture_atlas = TextureAtlas::from_grid(texture_handle.clone(), Vec2::new(16.0, 16.0), 23, 9, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.insert_resource(TextureHandle(texture_handle));
    commands.insert_resource(TextureAtlasHandle(texture_atlas_handle.clone()));
  }
}

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
