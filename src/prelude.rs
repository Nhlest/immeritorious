use crate::actor::{spawn_unit, Brain, BrainState, Cooldown, Unit, UnitType};
use crate::tilemap::{Passibility, TileMapMap};
use bevy::ecs::system::EntityCommand;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ecs_tilemap::helpers::square_grid::neighbors::*;
use bevy_ecs_tilemap::map::TilemapSize;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_ecs_tilemap::tiles::TileStorage;
use pathfinding::prelude::*;

pub struct PreludePlugin;

#[derive(Resource, Deref)]
pub struct TextureHandle(pub Handle<Image>);

#[derive(Resource, Deref)]
pub struct TextureAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct Cursor;

#[derive(Resource, Deref, DerefMut)]
pub struct Tick(pub u64);

pub struct CooldownCommand(pub u64);

impl EntityCommand for CooldownCommand {
  fn apply(self, id: Entity, world: &mut World) {
    let wait_till = world.resource::<Tick>().0 + self.0;
    world.entity_mut(id).insert(Cooldown(wait_till));
  }
}

impl Plugin for PreludePlugin {
  fn build(&self, app: &mut App) {
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
      commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
          scaling_mode: ScalingMode::FixedVertical(16.0 * 16.0),
          ..default()
        },
        transform: Transform::from_xyz(-42.0, 0.0, 1000.0 - 0.1),
        ..default()
      });
      commands.spawn((
        SpriteSheetBundle {
          sprite: TextureAtlasSprite::new(175),
          texture_atlas: texture_atlas_handle,
          transform: Transform::from_xyz(8.0, 8.0, 2.0),
          ..default()
        },
        Cursor,
      ));
    }
    fn initiate_tile_map(mut commands: Commands, texture_handle: Res<TextureHandle>) {
      let tile_map = TileMapMap::load_from_ldtk("assets/map.ldtk", &mut commands, &texture_handle);
      commands.insert_resource(tile_map);
    }
    fn spawn_units(mut commands: Commands, texture_atlas_handle: Res<TextureAtlasHandle>) {
      spawn_unit(
        &mut commands,
        texture_atlas_handle.0.clone(),
        Unit { t: UnitType::Soldier },
        (4, 4),
      );
    }
    fn ui(mut commands: Commands, asset_server: ResMut<AssetServer>) {
      commands
        .spawn(NodeBundle {
          style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            flex_direction: FlexDirection::Column,
            ..default()
          },
          ..default()
        })
        .with_children(|c| {
          c.spawn(ButtonBundle {
            style: Style {
              width: Val::Px(100.0),
              height: Val::Px(50.0),
              border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
              ..default()
            },
            background_color: Color::GREEN.into(),
            border_color: Color::DARK_GREEN.into(),
            ..default()
          });
          c.spawn(ButtonBundle {
            style: Style {
              width: Val::Px(100.0),
              height: Val::Px(50.0),
              border: UiRect::new(Val::Px(2.0), Val::Px(2.0), Val::Px(2.0), Val::Px(2.0)),
              justify_content: JustifyContent::Center,
              align_items: AlignItems::Center,
              ..default()
            },
            background_color: Color::GREEN.into(),
            border_color: Color::DARK_GREEN.into(),
            ..default()
          })
          .with_children(|c| {
            c.spawn(TextBundle::from_section(
              "Pepoga",
              TextStyle {
                font: asset_server.load("quardratic.ttf"),
                font_size: 22.0,
                color: Color::BLACK,
              },
            ));
          });
        });
    }
    fn interaction_color(mut buttons: Query<(&mut BackgroundColor, &Interaction), With<Button>>) {
      for (mut bg, i) in &mut buttons {
        *bg = match i {
          Interaction::Pressed => Color::RED.into(),
          Interaction::Hovered => Color::CYAN.into(),
          Interaction::None => Color::GREEN.into(),
        };
      }
    }
    fn cursor(
      mut cursor: Query<&mut Transform, With<Cursor>>,
      input: Res<Input<KeyCode>>,
      mut brains: Query<&mut Brain>,
    ) {
      let mut cursor = cursor.single_mut();
      let (mut cx, mut cy) = (
        (cursor.translation.x / 16.0 + 8.0) as i32,
        (cursor.translation.y / 16.0 + 8.0) as i32,
      );
      if input.just_pressed(KeyCode::Q) {
        cx -= 1;
        cy += 1;
      }
      if input.just_pressed(KeyCode::W) {
        cy += 1;
      }
      if input.just_pressed(KeyCode::E) {
        cx += 1;
        cy += 1;
      }
      if input.just_pressed(KeyCode::A) {
        cx -= 1;
      }
      if input.just_pressed(KeyCode::D) {
        cx += 1;
      }
      if input.just_pressed(KeyCode::Z) {
        cx -= 1;
        cy -= 1;
      }
      if input.just_pressed(KeyCode::X) {
        cy -= 1;
      }
      if input.just_pressed(KeyCode::C) {
        cx += 1;
        cy -= 1;
      }
      if input.just_pressed(KeyCode::Space) {
        let tile_pos_to = TilePos::new(cx as u32, cy as u32);
        brains.single_mut().state = BrainState::MovingTo(tile_pos_to);
      }
      cursor.translation = Vec3::new((cx as f32 - 8.0) * 16.0 + 8.0, (cy as f32 - 8.0) * 16.0 + 8.0, 2.0);
    }
    fn process_brains(
      mut commands: Commands,
      mut brains: Query<(Entity, &mut Brain, &mut TilePos, &Unit), Without<Cooldown>>,
      tile_storage: Query<&TileStorage>,
      tiles: Query<&Passibility>,
    ) {
      let tile_storage = tile_storage.single();
      for (entity, mut brain, mut tile_pos, unit) in &mut brains {
        match brain.state {
          BrainState::Idle => {}
          BrainState::MovingTo(pos_to) => {
            let path = astar(
              tile_pos.as_ref(),
              |a| {
                Neighbors::get_square_neighboring_positions(a, &TilemapSize { x: 16, y: 16 }, true)
                  .iter()
                  .filter(|a| tiles.get(tile_storage.get(a).unwrap()).unwrap() == &Passibility::Passable)
                  .map(|x| {
                    (*x, {
                      let d = x.x.abs_diff(a.x) + x.y.abs_diff(a.y);
                      if d == 1 {
                        10
                      } else {
                        14
                      }
                    })
                  })
                  .collect::<Vec<_>>()
              },
              |a| 10 * (a.x.abs_diff(pos_to.x) + a.y.abs_diff(pos_to.y)),
              |a| *a == pos_to,
            );
            match path {
              None => {}
              Some((p, _)) => {
                let next = p[1];
                let d = tile_pos.x.abs_diff(next.x) + tile_pos.y.abs_diff(next.y);
                *tile_pos = next;
                if tile_pos.as_ref() == &pos_to {
                  brain.state = BrainState::Idle;
                }
                commands.entity(entity).add(CooldownCommand(if d == 1 { 5 } else { 7 }));
              }
            }
          }
        }
      }
    }
    fn increment_tick(mut tick: ResMut<Tick>) {
      tick.0 += 1;
    }
    fn update_transforms(mut transforms: Query<(&mut Transform, &TilePos), With<Unit>>) {
      for (mut transform, tile_pos) in &mut transforms {
        transform.translation = Vec3::new(
          (tile_pos.x as f32 - 8.0) * 16.0 + 8.0,
          (tile_pos.y as f32 - 8.0) * 16.0 + 8.0,
          1.0,
        );
      }
    }
    fn clear_cooldowns(mut commands: Commands, cooldowns: Query<(Entity, &Cooldown)>, tick: Res<Tick>) {
      for (e, cd) in &cooldowns {
        if cd.0 <= tick.0 {
          commands.entity(e).remove::<Cooldown>();
        }
      }
    }
    app.add_systems(FixedUpdate, (increment_tick, clear_cooldowns).chain());
    app.add_systems(
      Update,
      (interaction_color, cursor, (process_brains, update_transforms).chain()),
    );
    app.add_systems(
      Startup,
      (load_sprite_sheet, apply_deferred, (initiate_tile_map, spawn_units, ui)).chain(),
    );
    app.insert_resource(FixedTime::new_from_secs(1.0 / 20.0));
    app.insert_resource(Tick(0));
  }
}
