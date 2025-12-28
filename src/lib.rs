#![allow(unused)]
use std::{num::NonZeroU32, rc::Rc};

use winit::{
  application::ApplicationHandler,
  event::*,
  event_loop::{ActiveEventLoop, EventLoop},
  window::Window,
};

use tiny_skia::{Color, FillRule, Paint, PathBuilder, Pixmap, Transform};

use softbuffer::{Context, Surface};

mod line;
#[derive(Default)]
pub struct Figure {
  window: Option<Rc<Window>>,
  context: Option<Context<Rc<Window>>>,
  surface: Option<Surface<Rc<Window>, Rc<Window>>>,
}

impl Figure {
  pub fn new() -> Self {
    Self {
      window: None,
      context: None,
      surface: None,
    }
  }
}

impl ApplicationHandler for Figure {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let attr = Window::default_attributes().with_title("Plotter");
    let window = Rc::new(event_loop.create_window(attr).unwrap());

    // Context 需要一个 HasDisplayHandle。Rc<Window> 满足要求。
    let context = Context::new(window.clone()).expect("Failed to create context");

    // Surface 需要 Context 的引用，以及一个 HasWindowHandle。
    // 这里第二个参数也传入 window.clone()。
    let surface = Surface::new(&context, window.clone()).expect("Failed to create surface");

    self.window = Some(window);
    self.context = Some(context);
    self.surface = Some(surface);
  }

  fn window_event(
    &mut self, event_loop: &ActiveEventLoop, window_id: winit::window::WindowId, event: WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::RedrawRequested => {
        let (Some(window), Some(surface)) = (&self.window, &mut self.surface) else {
          return;
        };
        let size = window.inner_size();

        surface
          .resize(
            NonZeroU32::new(size.width).unwrap(),
            NonZeroU32::new(size.height).unwrap(),
          )
          .unwrap();

        let mut pixmap = Pixmap::new(size.width, size.height).unwrap();

        pixmap.fill(Color::from_rgba8(30, 30, 30, 255));

        let mut buffer = surface.buffer_mut().unwrap();
        for (index, pixel) in pixmap.pixels().iter().enumerate() {
          let r = pixel.red() as u32;
          let g = pixel.green() as u32;
          let b = pixel.blue() as u32;

          buffer[index] = b | (g << 8) | (r << 16);
        }
        buffer.present().unwrap();
      }
      _ => {}
    }
  }
}
