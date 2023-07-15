use std::rc::Rc;
use easy_imgui_renderer::{glow, Renderer};
use easy_imgui_renderer::glow::{Context, HasContext};
use easy_imgui_window::EventFlags;
use glutin::prelude::GlSurface;
use crate::game::state::GameState;
use crate::game::init::GameLoop;

pub mod init;
pub mod state;
pub mod ui;
pub mod card;

pub struct Game {
  gl: Rc<Context>,
  renderer: Renderer,
  surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
  gl_context: glutin::context::PossiblyCurrentContext,
  app: GameState,
}

fn game_update(_g: &mut GameLoop) {}

fn game_render(g: &mut GameLoop) {
  unsafe {
    use glutin::context::PossiblyCurrentGlContext;

    g.game.gl_context.make_current(&g.game.surface).unwrap();
    g.game
      .gl
      .clear_color(0.5, 0.3, 0.2, 1.0);
    g.game.gl.clear(glow::COLOR_BUFFER_BIT);

    g.game.renderer.do_frame(&mut g.game.app);

    g.window.0.pre_present_notify();
    g.game.surface.swap_buffers(&g.game.gl_context).unwrap();
  }
}

fn game_handle(g: &mut GameLoop, event: &winit::event::WindowEvent) {
  use winit::{
    event::{ElementState, KeyEvent, WindowEvent},
    keyboard::{KeyCode, PhysicalKey},
  };
  let mut wr =
    easy_imgui_window::MainWindowPieces::new(&g.window.0, &g.game.surface, &g.game.gl_context);
  let imgui_wants = easy_imgui_window::window_event(
    &mut wr,
    &mut g.game.renderer,
    &mut g.window.1,
    &mut g.game.app,
    event,
    EventFlags::DoNotRender,
  );
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
    WindowEvent::CursorMoved { position, .. } if !imgui_wants.want_capture_mouse => {
      let scale = g.window.0.scale_factor();
      let size = g.window.0.inner_size();
      let size = size.to_logical::<f32>(scale);
      let position = position.to_logical::<f32>(scale);
    }
    _ => {}
  }
}

