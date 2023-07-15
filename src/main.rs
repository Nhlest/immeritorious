use crate::game::render::Init;
use easy_imgui_window::{
  winit,
};
use winit::{
  event_loop::EventLoop,
};

pub mod game;

fn main() {
  let event_loop = EventLoop::new().unwrap();
  let mut init = Init { game_loop: None };
  event_loop.run_app(&mut init).unwrap();
}

