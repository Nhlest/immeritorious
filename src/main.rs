use crate::prelude::PreludePlugin;
use bevy::prelude::*;
use bevy::window::{ExitCondition, WindowResolution};
use bevy_ecs_tilemap::prelude::*;

mod actor;
mod prelude;
mod tilemap;

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
    .run();
}
