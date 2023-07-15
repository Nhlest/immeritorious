use bevy_ecs::bundle::Bundle;
use bevy_ecs::component::Component;

#[derive(Component)]
pub struct Transform {
  pub x: f32,
  pub y: f32,
  pub z: f32,
}

impl Transform {
  pub fn new (x: f32, y: f32, z: f32) -> Self {
    Self { x, y, z }
  }

  pub fn zero() -> Self {
    Self::new(0.0, 0.0, 0.0)
  }
}

type CellColor = (f32, f32, f32);

#[derive(Component)]
pub struct Cell {
  pub cell: CellColor,
}

impl Cell {
}

#[derive(Component)]
pub struct Drag {
  pub x : f32,
  pub y : f32,
}

impl Drag {
  pub fn new(x: f32, y: f32) -> Self {
    Self { x, y }
  }
}

#[derive(Bundle)]
pub struct CellTransform {
  pub transform: Transform,
  pub cell: Cell,
  pub drag: Drag
}

impl CellTransform {
  pub fn new(transform: Transform, cell: Cell) -> Self { Self { transform, cell, drag: Drag::new(0.0, 0.0) } }
}

#[derive(Bundle)]
pub struct CellBundle<T: Sync + Send + 'static + Bundle> {
  transform: Transform,
  t: T
}

impl<T: Sync + Send + 'static + Bundle> CellBundle<T> {
  pub fn new(transform: Transform, t: T) -> Self {
    Self {
      transform,
      t
    }
  }
}