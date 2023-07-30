#![allow(incomplete_features)]
#![feature(adt_const_params)]

use crate::client::ImmeritoriousClientPlugin;
use crate::prelude::PreludePlugin;
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowResolution};
use bevy_ecs_tilemap::prelude::*;
use bevy_renet::transport::NetcodeClientPlugin;
use bevy_renet::RenetClientPlugin;

mod actor;
mod buildings;
mod client;
mod game;
mod input;
mod prelude;
mod side;
mod tilemap;
mod ui;

fn main() {
  App::new()
    .add_plugins(
      DefaultPlugins
        .set(WindowPlugin {
          primary_window: Some(Window {
            resolution: WindowResolution::new(1024.0, 768.0),
            title: "Immeritorious".to_string(),
            resizable: false,
            ..default()
          }),
          exit_condition: ExitCondition::OnPrimaryClosed,
          close_when_requested: true,
        })
        .set(ImagePlugin::default_nearest()),
    )
    .add_plugins(PreludePlugin)
    .add_plugins(TilemapPlugin)
    .add_plugins(RenetClientPlugin)
    .add_plugins(NetcodeClientPlugin)
    .add_plugins(ImmeritoriousClientPlugin)
    .run();
}
