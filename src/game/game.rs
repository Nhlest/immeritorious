use crate::game::render::GameLoop;
use crate::game::resources::game_state::GameState;
use bevy_ecs::schedule::ScheduleLabel;
use easy_imgui_renderer::glow::Context;
use easy_imgui_renderer::Renderer;
use easy_imgui_window::EventFlags;
use std::rc::Rc;

pub struct Game {
  pub gl: Rc<Context>,
  pub renderer: Renderer,
  pub surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
  pub gl_context: glutin::context::PossiblyCurrentContext,
  pub app: GameState,
}

pub fn game_update(_: &mut GameLoop) {
}

#[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
pub struct Schedul;

pub fn game_handle(g: &mut GameLoop, event: &winit::event::WindowEvent) {
  use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
  };
  let mut wr = easy_imgui_window::MainWindowPieces::new(&g.window.0, &g.game.surface, &g.game.gl_context);
  let imgui_wants = easy_imgui_window::window_event(
    &mut wr,
    &mut g.game.renderer,
    &mut g.window.1,
    &mut g.game.app,
    event,
    EventFlags::DoNotRender,
  );
  let world = &mut g.game.app.world;
  world.run_schedule(Schedul);
  if imgui_wants.window_closed {
    g.exit();
    return;
  }
  match event {
    WindowEvent::KeyboardInput {
      event:
      KeyEvent {
        physical_key: PhysicalKey::Code(code),
        state: ElementState::Pressed,
        ..
      },
      ..
    } if !imgui_wants.want_capture_keyboard => match code {
      KeyCode::Escape => {
        g.exit();
      }
      _ => {}
    },
    WindowEvent::CursorMoved { .. } if !imgui_wants.want_capture_mouse => {
    }
    _ => {}
  }
}

