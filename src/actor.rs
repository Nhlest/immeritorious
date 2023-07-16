use bevy::prelude::*;
use bevy_ecs_tilemap::prelude::TilePos;

#[derive(Component)]
pub struct ActorMarker;

#[derive(Component)]
pub struct Building;

#[derive(Component)]
pub struct Unit {
  pub t: UnitType,
}

#[derive(Component)]
pub struct Cooldown(pub u64);

#[derive(Copy, Clone)]
pub enum UnitType {
  Soldier,
  Farmer,
}

#[derive(Default)]
pub enum BrainState {
  #[default]
  Idle,
  MovingTo(TilePos),
}

#[derive(Component, Default)]
pub struct Brain {
  pub state: BrainState,
}

impl UnitType {
  pub fn into_sprite_sheet_bundle(
    self,
    texture_atlas_handle: Handle<TextureAtlas>,
    transform: Transform,
  ) -> SpriteSheetBundle {
    let tile = match self {
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
}

pub fn spawn_unit(
  commands: &mut Commands,
  texture_atlas_handle: Handle<TextureAtlas>,
  unit: Unit,
  location: (u32, u32),
) {
  commands.spawn((
    unit.t.into_sprite_sheet_bundle(
      texture_atlas_handle,
      Transform::from_xyz(
        location.0 as f32 * 16.0 - 16 as f32 * 8.0 + 8.0,
        location.1 as f32 * 16.0 - 16 as f32 * 8.0 + 8.0,
        1.0,
      ),
    ),
    unit,
    TilePos::new(location.0, location.1),
    Brain::default(),
  ));
}
