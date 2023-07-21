use crate::server::{new_renet_server, ImmeritoriousServerPlugin};
use bevy::prelude::*;
use bevy_renet::transport::NetcodeServerPlugin;
use bevy_renet::RenetServerPlugin;

mod actor;
mod server;
mod side;
mod tilemap;

pub fn start_server(ip: &str) {
  let (server, transport) = new_renet_server(ip);
  App::new()
    .add_plugins(MinimalPlugins)
    .add_plugins(NetcodeServerPlugin)
    .add_plugins(RenetServerPlugin)
    .insert_resource(server)
    .insert_resource(transport)
    .add_plugins(ImmeritoriousServerPlugin)
    .run();
}
