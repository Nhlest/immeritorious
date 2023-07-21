use crate::client::new_renet_client;
use crate::tilemap::TextureAtlasHandle;
use crate::ui::{regular_button, text_field_button, ButtonTag};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ecs_tilemap::prelude::TilePos;
use bevy_renet::renet::RenetClient;
use immeritorious_common::netcode::{PlayerCommand, Pos, Sendable};
use immeritorious_common::units::Unit;
use immeritorious_server::start_server;
use std::thread;

#[derive(Component)]
pub struct Cursor;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum ImmeritoriousState {
  #[default]
  MainMenu,
  Connecting,
  ConnectedInGame,
}

pub struct ImmeritoriousGamePlugin;

impl Plugin for ImmeritoriousGamePlugin {
  fn build(&self, app: &mut App) {
    app.add_state::<ImmeritoriousState>();
    app.add_systems(Startup, Self::spawn_camera);
    app.add_systems(
      Update,
      (Self::cursor, Self::update_transforms).run_if(in_state(ImmeritoriousState::ConnectedInGame)),
    );
    app.add_systems(Update, Self::main_menu.run_if(in_state(ImmeritoriousState::MainMenu)));
    app.add_systems(OnEnter(ImmeritoriousState::MainMenu), Self::init_main_menu);
    app.add_systems(OnExit(ImmeritoriousState::MainMenu), Self::clear_main_menu_ui);
    app.add_systems(OnEnter(ImmeritoriousState::ConnectedInGame), Self::spawn_cursor);
  }
}

impl ImmeritoriousGamePlugin {
  fn cursor(
    mut cursor: Query<&mut Transform, With<Cursor>>,
    input: Res<Input<KeyCode>>,
    mut client: ResMut<RenetClient>, // mut brains: Query<&mut Brain>,
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
      let tile_pos_to = Pos((cx as u32, cy as u32));
      client.send(&PlayerCommand::MoveTo(tile_pos_to));
    }
    cursor.translation = Vec3::new((cx as f32 - 8.0) * 16.0 + 8.0, (cy as f32 - 8.0) * 16.0 + 8.0, 2.0);
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
  fn init_main_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands
      .spawn(NodeBundle {
        style: Style {
          width: Val::Percent(100.0),
          height: Val::Percent(100.0),
          bottom: Val::Percent(25.0),
          flex_direction: FlexDirection::Column,
          justify_content: JustifyContent::Center,
          align_items: AlignItems::Center,
          ..default()
        },
        ..default()
      })
      .with_children(|c| {
        text_field_button::<"IP">(c, asset_server.as_ref(), "127.0.0.1:5050");
        text_field_button::<"Name">(c, asset_server.as_ref(), "Player");
        regular_button::<"Connect">(c, asset_server.as_ref(), "Connect");
        regular_button::<"Start & Connect">(c, asset_server.as_ref(), "Start & Connect");
        regular_button::<"Quit">(c, asset_server.as_ref(), "Quit");
      });
  }
  fn main_menu(
    mut commands: Commands,
    ip_text: Query<&Text, With<ButtonTag<"IP">>>,
    connect_button: Query<&Interaction, With<ButtonTag<"Connect">>>,
    start_connect_button: Query<&Interaction, With<ButtonTag<"Start & Connect">>>,
    quit: Query<&Interaction, With<ButtonTag<"Quit">>>,
    mut app_state: ResMut<NextState<ImmeritoriousState>>,
    mut app_exit: EventWriter<AppExit>,
  ) {
    if *quit.single() == Interaction::Pressed {
      app_exit.send(AppExit);
    }
    if *connect_button.single() == Interaction::Pressed || *start_connect_button.single() == Interaction::Pressed {
      let ip = ip_text.single().sections.first().as_ref().unwrap().value.as_str();
      let ip_str = ip.to_string();
      if *start_connect_button.single() == Interaction::Pressed {
        thread::spawn(move || {
          start_server(ip_str.as_str());
        });
      }
      let (client, transport) = new_renet_client(ip);
      commands.insert_resource(client);
      commands.insert_resource(transport);
      app_state.set(ImmeritoriousState::Connecting);
    }
  }
  fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
      projection: OrthographicProjection {
        scaling_mode: ScalingMode::FixedVertical(16.0 * 16.0),
        ..default()
      },
      transform: Transform::from_xyz(-42.0, 0.0, 1000.0 - 0.1),
      ..default()
    });
  }
  fn clear_main_menu_ui(mut commands: Commands, ui: Query<Entity, With<Node>>) {
    ui.iter().for_each(|e| commands.entity(e).despawn());
  }
  fn spawn_cursor(mut commands: Commands, texture_atlas_handle: Res<TextureAtlasHandle>) {
    commands.spawn((
      SpriteSheetBundle {
        sprite: TextureAtlasSprite::new(175),
        texture_atlas: texture_atlas_handle.clone(),
        transform: Transform::from_xyz(8.0, 8.0, 2.0),
        ..default()
      },
      Cursor,
    ));
  }
}
