use bevy::prelude::*;
use immeritorious_common::units::Side;

#[derive(Resource)]
pub struct MySide(pub Side);

#[derive(Resource)]
pub struct MySideName(pub String);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct UnitSelection {
  pub units: Vec<Entity>,
}
