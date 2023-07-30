use crate::actor::RenderableSprite;
use crate::client::new_renet_client;
use crate::input::InputSet;
use crate::side::{MySideName, PrimeSelection, UnitSelection};
use crate::tilemap::SpriteSheetAtlasHandle;
use crate::ui::{percentage_bar, regular_button, text_field_button, ButtonTag, UiTag, UnitButton, UnitButtonRow};
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy::render::camera::ScalingMode;
use bevy_ecs_tilemap::prelude::TilePos;
use immeritorious_common::units::{Side, Unit, HP};
use immeritorious_server::start_server;
use std::thread;

#[derive(Component)]
pub struct Cursor;

#[derive(Component)]
pub struct Selector;

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
    app.init_resource::<UnitSelection>();
    app.add_systems(Startup, Self::spawn_camera);
    app.add_systems(
      Update,
      (
        Self::update_transforms,
        Self::highlight_selected_units,
        Self::update_unit_buttons,
        Self::update_unit_stats,
      )
        .run_if(in_state(ImmeritoriousState::ConnectedInGame))
        .after(InputSet),
    );
    app.add_systems(Update, Self::main_menu.run_if(in_state(ImmeritoriousState::MainMenu)));
    app.add_systems(OnEnter(ImmeritoriousState::MainMenu), Self::init_main_menu);
    app.add_systems(OnExit(ImmeritoriousState::MainMenu), Self::clear_ui);
    app.add_systems(OnEnter(ImmeritoriousState::ConnectedInGame), Self::init_game_ui);
    app.add_systems(OnExit(ImmeritoriousState::ConnectedInGame), Self::clear_ui);
    app.add_systems(OnEnter(ImmeritoriousState::ConnectedInGame), Self::spawn_cursor);
  }
}

impl ImmeritoriousGamePlugin {
  fn update_unit_stats(
    mut commands: Commands,
    hp_bar_parent: Query<Entity, With<UiTag<"HpBarParent">>>,
    hp_bar: Query<Entity, With<UiTag<"HpBar">>>,
    asset_server: Res<AssetServer>,
    hp: Query<(&Unit, Ref<HP>)>,
    prime_selection: Option<Res<PrimeSelection>>,
  ) {
    if let Some(prime_selection_resource) = prime_selection {
      if prime_selection_resource.is_changed()
        || prime_selection_resource.is_added()
        || hp.get(prime_selection_resource.0).unwrap().1.is_changed()
      {
        hp_bar.for_each(|e| commands.entity(e).despawn_recursive());
        let prime_selection_entity = prime_selection_resource.0;
        let (unit, hp) = hp.get(prime_selection_entity).unwrap();
        let hp_bar_parent_entity = hp_bar_parent.single();
        let entity = percentage_bar(
          &mut commands.spawn_empty(),
          asset_server.as_ref(),
          hp.0,
          unit.get_max_hp(),
        );
        commands.entity(hp_bar_parent_entity).insert_children(0, &[entity]);
      }
    } else {
      hp_bar.for_each(|e| commands.entity(e).despawn_recursive());
    }
  }
  fn update_unit_buttons(
    units: Query<(Entity, &Side, &Unit)>,
    mut unit_buttons: Query<(&mut BorderColor, &UnitButton)>,
    mut unit_ui_atlas_images: Query<(&Parent, &mut UiTextureAtlasImage, &mut Visibility)>,
    prime_selection: Option<Res<PrimeSelection>>,
  ) {
    for (parent_e, mut ui_texture, mut visibility) in &mut unit_ui_atlas_images {
      let (mut border_color, unit_button) = unit_buttons.get_mut(parent_e.get()).unwrap();
      let (e, side, unit) = units.get(unit_button.0).unwrap();
      if prime_selection
        .as_ref()
        .map(|prime_selection_resource| prime_selection_resource.0 == e)
        .unwrap_or(false)
      {
        border_color.0 = Color::RED;
      } else {
        border_color.0 = Color::BLUE;
      }
      let sprite_id = unit.sprite_id(side);
      ui_texture.index = sprite_id;
      *visibility = Visibility::Visible;
    }
  }
  fn highlight_selected_units(
    mut commands: Commands,
    unit_selection: Res<UnitSelection>,
    texture_atlas_handle: Res<SpriteSheetAtlasHandle>,
    selectors: Query<(Entity, &Parent), With<Selector>>,
    prime_selection: Option<Res<PrimeSelection>>,
  ) {
    if prime_selection
      .as_ref()
      .map(|prime_selection_resource| prime_selection_resource.is_changed() || prime_selection_resource.is_added())
      .unwrap_or(false)
      || unit_selection.is_changed()
    {
      selectors
        .iter()
        .for_each(|(e, _)| commands.entity(e).despawn_recursive());

      let selected = prime_selection.map(|prime_selection_resource| prime_selection_resource.0);

      unit_selection.units.iter().for_each(|e| {
        commands.entity(*e).with_children(|c| {
          c.spawn((
            SpriteSheetBundle {
              sprite: TextureAtlasSprite::new(if selected.map(|s| s == *e).unwrap_or(false) {
                174
              } else {
                173
              }),
              texture_atlas: texture_atlas_handle.clone(),
              ..default()
            },
            Selector,
          ));
        });
      });
    }
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
  fn init_game_ui(mut commands: Commands) {
    commands
      .spawn((
        NodeBundle {
          style: Style {
            width: Val::Percent(24.5),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexStart,
            align_items: AlignItems::FlexStart,
            ..default()
          },
          ..default()
        },
        UiTag::<"HpBarParent">,
      ))
      .with_children(|c| {
        c.spawn(NodeBundle {
          style: Style {
            width: Val::Percent(20.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::FlexEnd,
            align_items: AlignItems::FlexStart,
            ..default()
          },
          ..default()
        })
        .with_children(|c| {
          c.spawn((
            NodeBundle {
              style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                flex_direction: FlexDirection::Row,
                justify_content: JustifyContent::FlexStart,
                align_items: AlignItems::FlexEnd,
                ..default()
              },
              ..default()
            },
            UnitButtonRow,
          ));
        });
      });
  }
  fn main_menu(
    mut commands: Commands,
    mut app_state: ResMut<NextState<ImmeritoriousState>>,
    mut app_exit: EventWriter<AppExit>,
    ip_text: Query<&Text, With<ButtonTag<"IP">>>,
    nickname: Query<&Text, With<ButtonTag<"Name">>>,
    connect_button: Query<&Interaction, With<ButtonTag<"Connect">>>,
    start_connect_button: Query<&Interaction, With<ButtonTag<"Start & Connect">>>,
    quit_button: Query<&Interaction, With<ButtonTag<"Quit">>>,
  ) {
    if *quit_button.single() == Interaction::Pressed {
      app_exit.send(AppExit);
    }
    if *connect_button.single() == Interaction::Pressed || *start_connect_button.single() == Interaction::Pressed {
      commands.insert_resource(MySideName(nickname.single().sections[0].value.clone()));
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
  fn clear_ui(mut commands: Commands, ui: Query<Entity, With<Node>>) {
    ui.iter().for_each(|e| commands.entity(e).despawn());
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
  fn spawn_cursor(mut commands: Commands, texture_atlas_handle: Res<SpriteSheetAtlasHandle>) {
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
