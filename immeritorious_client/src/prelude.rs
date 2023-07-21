use crate::client::new_renet_client;
use crate::ui::{
  highlight_active_button, regular_button, text_button_activate, text_field_button, text_input, ButtonTag,
};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ecs_tilemap::prelude::TilePos;
use immeritorious_common::units::Unit;
use immeritorious_server::start_server;
use std::thread;
use bevy_renet::renet::{DefaultChannel, RenetClient};
use immeritorious_common::netcode::{PlayerCommand, Pos};

pub struct PreludePlugin;

#[derive(Resource, Deref)]
pub struct TextureHandle(pub Handle<Image>);

#[derive(Resource, Deref)]
pub struct TextureAtlasHandle(pub Handle<TextureAtlas>);

#[derive(Component)]
pub struct Cursor;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum ImmeritoriousState {
  #[default]
  MainMenu,
  Connecting,
  ConnectedInGame,
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
    fn cursor(
      mut cursor: Query<&mut Transform, With<Cursor>>,
      input: Res<Input<KeyCode>>,
      mut client: ResMut<RenetClient>
      // mut brains: Query<&mut Brain>,
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
        client.send_message(DefaultChannel::ReliableOrdered, PlayerCommand::MoveTo(tile_pos_to).cast());
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
    app.add_systems(Startup, spawn_camera);
    app.add_systems(
      Update,
      (main_menu, text_input, text_button_activate, highlight_active_button)
        .run_if(in_state(ImmeritoriousState::MainMenu)),
    );
    app.add_systems(
      Update,
      (cursor, (update_transforms).chain()).run_if(in_state(ImmeritoriousState::ConnectedInGame)),
    );
    app.add_systems(OnEnter(ImmeritoriousState::Connecting), load_sprite_sheet);
    app.add_systems(OnEnter(ImmeritoriousState::MainMenu), init_main_menu);
    app.add_systems(OnExit(ImmeritoriousState::MainMenu), clear_main_menu_ui);
  }
}
