use crate::game::ImmeritoriousGamePlugin;
use crate::tilemap::TileMapPlugin;
use crate::ui::UiPlugin;
use bevy::prelude::*;

pub struct PreludePlugin;

impl Plugin for PreludePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(TileMapPlugin);
    app.add_plugins(ImmeritoriousGamePlugin);
    app.add_plugins(UiPlugin);
  }
}
