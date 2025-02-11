use crate::game::game::{game_handle, game_update, Game};
use crate::game::resources::game_state::{GameState, I};
use easy_imgui::image::ImageReader;
use easy_imgui_renderer::glow::{HasContext, PixelUnpackData};
use easy_imgui_renderer::{glow, Renderer};
use easy_imgui_window::MainWindow;
use easy_imgui_window::MainWindowStatus;
use glutin::prelude::GlSurface;
use std::rc::Rc;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub type GameLoop = game_loop::GameLoop<Game, game_loop::Time, (Arc<Window>, MainWindowStatus)>;

pub struct Init {
  pub game_loop: Option<GameLoop>,
}

impl ApplicationHandler for Init {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let wattrs = Window::default_attributes().with_title("Comintern Sim");
    let window = MainWindow::new::<()>(event_loop, wattrs).unwrap();
    let gl = Rc::new(window.create_gl_context());
    let (gl_context, surface, window) = unsafe { window.into_pieces() };
    let window = Arc::new(window);

    let tex = unsafe { gl.create_texture().unwrap() } ;
    let i = ImageReader::open("bg.png").unwrap().decode().unwrap().to_rgba8();
    unsafe {
      gl.bind_texture(glow::TEXTURE_2D, Some(tex));
      gl.tex_parameter_i32( glow::TEXTURE_2D, glow::TEXTURE_WRAP_S, glow::CLAMP_TO_EDGE as i32, );
      gl.tex_parameter_i32( glow::TEXTURE_2D, glow::TEXTURE_WRAP_T, glow::CLAMP_TO_EDGE as i32, );
      gl.tex_parameter_i32( glow::TEXTURE_2D, glow::TEXTURE_MIN_FILTER, glow::LINEAR as i32, );
      gl.tex_parameter_i32( glow::TEXTURE_2D, glow::TEXTURE_MAG_FILTER, glow::LINEAR as i32, );
      gl.tex_parameter_i32(glow::TEXTURE_2D, glow::TEXTURE_MAX_LEVEL, 0);
      gl.tex_image_2d(glow::TEXTURE_2D, 0, glow::RGBA as i32, i.width() as i32, i.height() as i32, 0, glow::RGBA, glow::UNSIGNED_BYTE, PixelUnpackData::Slice(Some(std::slice::from_raw_parts(i.as_ptr(), (i.width() * i.height() * 4) as usize))));
      gl.bind_texture(glow::TEXTURE_2D, None);
      let a = gl.get_error();
      println!("{}", a);
    }

    let mut renderer = Renderer::new(gl.clone()).unwrap();
    renderer.set_background_color(None);
    let scale = window.scale_factor();
    let size = window.inner_size().to_logical::<f32>(scale);
    let size = easy_imgui::Vector2::new(size.width, size.height);
    renderer.set_size(size, scale as f32);

    let window_status = MainWindowStatus::default();

    let id = Renderer::map_tex(tex);

    let mut game = GameState::new();
    game.world.insert_resource(I(id));

    let game = Game {
      gl: gl.clone(),
      surface,
      gl_context,
      renderer,
      app: game
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

