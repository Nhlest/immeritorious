use crate::game::ImmeritoriousState;
use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::*;
use immeritorious_common::netcode::{Pos, Tile};
use immeritorious_common::Passibility;

#[derive(Resource, Deref)]
pub struct SpriteSheetTextureHandle(pub Handle<Image>);

#[derive(Resource, Deref)]
pub struct SpriteSheetAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Resource, Deref)]
pub struct ButtonsAtlasHandle(pub Handle<TextureAtlas>);

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
    let spritesheet_handle = asset_server.load("spritesheet_01.png");
    let buttons_handle = asset_server.load("buttons.png");

    let spritesheet_atlas =
      TextureAtlas::from_grid(spritesheet_handle.clone(), Vec2::new(16.0, 16.0), 23, 9, None, None);
    let buttons_atlas = TextureAtlas::from_grid(buttons_handle.clone(), Vec2::new(8.0, 8.0), 34, 24, None, None);
    let spritesheet_atlas_handle = texture_atlases.add(spritesheet_atlas);
    let buttons_atlas_handle = texture_atlases.add(buttons_atlas);
    commands.insert_resource(ClearColor(Color::BLACK));
    commands.insert_resource(SpriteSheetTextureHandle(spritesheet_handle));
    commands.insert_resource(SpriteSheetAtlasHandle(spritesheet_atlas_handle));
    commands.insert_resource(ButtonsAtlasHandle(buttons_atlas_handle));
  }
}

#[derive(Resource)]
pub struct TileMapMap {}

impl TileMapMap {
  pub fn load_from_network(
    tiles: Vec<(Tile, Pos, Passibility)>,
    commands: &mut Commands,
    texture_handle: &SpriteSheetTextureHandle,
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

pub fn distance(from: &TilePos, to: &TilePos) -> u32 {
  ((from.x.abs_diff(to.x).pow(2) + from.y.abs_diff(to.y).pow(2)) as f32).sqrt() as u32
}

pub fn distance_manhattan(from: &TilePos, to: &TilePos) -> u32 {
  from.x.abs_diff(to.x) + from.y.abs_diff(to.y)
}
