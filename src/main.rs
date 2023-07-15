use crate::game::init::Init;
use easy_imgui::cgmath::{InnerSpace, MetricSpace};
use easy_imgui_window::{
  easy_imgui_renderer::glow::{self, HasContext},
  glutin::prelude::GlSurface,
  winit,
};
use winit::{
  application::ApplicationHandler,
  event_loop::EventLoop,
};

pub mod game;

fn main() {
  let event_loop = EventLoop::new().unwrap();

  let mut init = Init { game_loop: None };
  event_loop.run_app(&mut init).unwrap();
}

