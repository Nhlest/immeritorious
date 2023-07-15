use easy_imgui_renderer::{glow, Renderer};
use easy_imgui_window::{MainWindow, MainWindowStatus};
use std::rc::Rc;
use std::sync::Arc;
use easy_imgui_renderer::glow::HasContext;
use glutin::prelude::GlSurface;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;
use crate::game::game::{game_handle, game_update, Game};
use crate::game::resources::game_state::GameState;

pub type GameLoop = game_loop::GameLoop<Game, game_loop::Time, (Arc<Window>, MainWindowStatus)>;

pub struct Init {
  pub game_loop: Option<GameLoop>,
}

impl ApplicationHandler for Init {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let wattrs = Window::default_attributes().with_title("game_loop");
    let window = MainWindow::new::<()>(event_loop, wattrs).unwrap();
    let gl = Rc::new(window.create_gl_context());
    let (gl_context, surface, window) = unsafe { window.into_pieces() };
    let window = Arc::new(window);

    let mut renderer = Renderer::new(gl.clone()).unwrap();
    renderer.set_background_color(None);
    let scale = window.scale_factor();
    let size = window.inner_size().to_logical::<f32>(scale);
    let size = easy_imgui::Vector2::new(size.width, size.height);
    renderer.set_size(size, scale as f32);

    let window_status = MainWindowStatus::default();

    let game = Game {
      gl: gl.clone(),
      surface,
      gl_context,
      renderer,
      app: GameState::new()
    };
    let game_loop = game_loop::GameLoop::new(game, 60, 0.1, (window, window_status));

    self.game_loop = Some(game_loop);
  }
  fn window_event(
    &mut self,
    event_loop: &ActiveEventLoop,
    window_id: winit::window::WindowId,
    event: winit::event::WindowEvent,
  ) {
    let Some(game_loop) = self.game_loop.as_mut() else {
      return;
    };
    if game_loop.window.0.id() != window_id {
      return;
    }
    game_handle(game_loop, &event);
    match event {
      winit::event::WindowEvent::RedrawRequested => {
        if !game_loop.next_frame(game_update, game_render) {
          event_loop.exit();
        }
      }
      _ => {}
    }
  }
  fn about_to_wait(&mut self, _event_loop: &ActiveEventLoop) {
    let Some(game_loop) = self.game_loop.as_mut() else {
      return;
    };
    game_loop.window.0.request_redraw();
  }
}

pub fn game_render(g: &mut GameLoop) {
  unsafe {
    use glutin::context::PossiblyCurrentGlContext;

    g.game.gl_context.make_current(&g.game.surface).unwrap();
    g.game.gl.clear_color(0.5, 0.3, 0.2, 1.0);
    g.game.gl.clear(glow::COLOR_BUFFER_BIT);

    g.game.renderer.do_frame(&mut g.game.app);

    g.window.0.pre_present_notify();
    g.game.surface.swap_buffers(&g.game.gl_context).unwrap();
  }
}

