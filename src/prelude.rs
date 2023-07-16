use crate::actor::{spawn_unit, Unit, UnitType};
use crate::tilemap::TileMapMap;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;

pub struct PreludePlugin;

#[derive(Resource, Deref)]
pub struct TextureHandle(pub Handle<Image>);

#[derive(Resource, Deref)]
pub struct TextureAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct Cursor;

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
      commands.insert_resource(ClearColor(Color::DARK_GRAY));
      commands.insert_resource(TextureHandle(texture_handle.clone()));
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
        (5, 5),
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
    fn cursor(mut cursor: Query<&mut Transform, With<Cursor>>, input: Res<Input<KeyCode>>) {
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
      cursor.translation = Vec3::new((cx as f32 - 8.0) * 16.0 + 8.0, (cy as f32 - 8.0) * 16.0 + 8.0, 2.0);
    }
    app.add_systems(Update, (interaction_color, cursor));
    app.add_systems(
      Startup,
      (load_sprite_sheet, apply_deferred, (initiate_tile_map, spawn_units, ui)).chain(),
    );
  }
}
