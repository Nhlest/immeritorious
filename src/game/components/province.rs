use bevy_ecs::component::Component;
use bevy_ecs::prelude::Bundle;

#[derive(Component)]
pub struct Province {
  pub name: String
}

impl Province {
  pub fn new(name: String) -> Self {
    Self {
      name: name.into()
    }
  }
}

#[derive(Component)]
pub struct Workers {
  pub current: u32,
  pub max: u32,
}

impl Workers {
  pub fn new(current: u32, max: u32) -> Self {
    Self {
      current, max
    }
  }
}

#[derive(Bundle)]
pub struct ProvinceBundle {
  pub province: Province,
  pub workers: Workers
}

impl ProvinceBundle {
  pub fn new(name: String, current: u32, max: u32) -> Self {
    Self {
      province: Province::new(name),
      workers: Workers::new(current, max),
    }
  }
}