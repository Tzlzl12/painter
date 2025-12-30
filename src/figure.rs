use std::{num::NonZeroU32, rc::Rc};

use winit::{
  application::ApplicationHandler,
  dpi::{LogicalSize, PhysicalSize},
  event::*,
  event_loop::{ActiveEventLoop, EventLoop},
  keyboard::{KeyCode, PhysicalKey},
  window::Window,
};

use tiny_skia::{Color, Pixmap};

use softbuffer::{Context, Surface};

use crate::{axis::Axis, color};

pub struct Figure {
  window: Option<Rc<Window>>,
  context: Option<Context<Rc<Window>>>,
  surface: Option<Surface<Rc<Window>, Rc<Window>>>,

  pixmap: Pixmap,
  axes: Vec<Axis>,
  config: Config,
}

pub struct Config {
  title: String,
  size: (u32, u32),
  layout: (u32, u32),
}
impl Default for Config {
  fn default() -> Self {
    Self {
      title: String::from("Painter"),
      size: (600, 400),
      layout: (1, 1),
    }
  }
}

impl Figure {
  pub fn new(config: Config) -> Self {
    let (width, height) = config.size;
    let mut axes = Vec::new();
    axes.push(Axis::new(0., 0., (0., 0.)));
    Self {
      window: None,
      context: None,
      surface: None,
      pixmap: Pixmap::new(width, height).unwrap(),
      axes,
      config,
    }
  }
  pub fn show(&mut self) {
    let event_loop = EventLoop::new().unwrap();
    let _ = event_loop.run_app(self);
  }

  fn resize(&mut self, size: PhysicalSize<u32>) {
    let w = size.width;
    let h = size.height;

    if let Some(surface) = &mut self.surface {
      let w = NonZeroU32::new(w).unwrap();
      let h = NonZeroU32::new(h).unwrap();

      if let Err(e) = surface.resize(w, h) {
        println!("resize error: {}", e);
        return;
      }
    }

    // update the inner state
    let pixmap = Pixmap::new(w, h).unwrap();
    self.pixmap = pixmap;
    self.config.size = size.into();
    self.change_axis_size(size);
  }
  fn change_axis_size(&mut self, size: PhysicalSize<u32>) {
    let w = size.width as f32;
    let h = size.height as f32;

    let (rows, cols) = self.config.layout;

    let each_w = w / cols as f32;
    let each_h = h / rows as f32;

    // 更新每个 Axis 的位置和大小
    for (index, a) in self.axes.iter_mut().enumerate() {
      let r = (index as u32) / cols;
      let c = (index as u32) % cols;

      // 更新 Axis 内部的 viewport 或位置属性
      a.change_veiwport((c as f32 * each_w, r as f32 * each_h), (each_w, each_h));
    }
  }
  pub fn add_subplot(&mut self, layout: (u32, u32)) {
    self.config.layout = layout;

    let (rows, cols) = self.config.layout;

    for _ in 0..rows * cols {
      self.axes.push(Axis::new(0.0, 0.0, (0.0, 0.0)));
    }
  }
  pub fn nth(&mut self, index: usize) -> Option<&mut Axis> {
    self.axes.get_mut(index)
  }
}

/// ===========Window Handler==============
impl ApplicationHandler for Figure {
  fn resumed(&mut self, event_loop: &ActiveEventLoop) {
    let config = &mut self.config;

    let attr = Window::default_attributes()
      .with_title(&config.title)
      .with_inner_size(LogicalSize::new(config.size.0 as f64, config.size.1 as f64));
    let window = Rc::new(event_loop.create_window(attr).unwrap());

    // Context 需要一个 HasDisplayHandle。Rc<Window> 满足要求。
    let context = Context::new(window.clone()).expect("Failed to create context");

    // Surface 需要 Context 的引用，以及一个 HasWindowHandle。
    // 这里第二个参数也传入 window.clone()。
    let surface = Surface::new(&context, window.clone()).expect("Failed to create surface");
    let size = window.as_ref().inner_size();
    self.config.size = (size.width, size.height);

    // if have subplot, create axes, if not, create one
    let (rows, cols) = self.config.layout;
    let (total_w, total_h) = self.config.size;
    let each_w = (total_w / cols) as f32;
    let each_h = (total_h / rows) as f32;

    for r in 0..rows {
      for c in 0..cols {
        // 计算每个子图的左上角坐标
        let x = c as f32 * each_w;
        let y = r as f32 * each_h;
        self.axes[(r * rows + c) as usize].change_veiwport((x, y), (each_w, each_h));
        // local_axes.push(Axis::new(x, y, (each_w, each_h)));
      }
    }
    let pixmap = Pixmap::new(total_w, total_h).unwrap();
    self.pixmap = pixmap;
    self.window = Some(window);
    self.context = Some(context);
    self.surface = Some(surface);
  }

  fn window_event(
    &mut self, event_loop: &ActiveEventLoop, _window_id: winit::window::WindowId,
    event: WindowEvent,
  ) {
    match event {
      WindowEvent::CloseRequested => event_loop.exit(),
      WindowEvent::RedrawRequested => {
        let (Some(window), Some(surface)) = (&self.window, &mut self.surface) else {
          return;
        };

        let size = window.inner_size();

        let (w, h) = (size.width, size.height);
        if w == 0 || h == 0 {
          return;
        }

        // ================draw into pixmap====

        let bg = color::get_bg();
        self
          .pixmap
          .fill(Color::from_rgba8(bg[0], bg[1], bg[2], bg[3]));
        //
        // draw axes
        for a in &mut self.axes {
          // println!("{} {} {:?}", a.x, a.y, a.veiwport);
          a.render(&mut self.pixmap);
        }
        //===========pixmap to buffer ===============
        let mut buffer = match surface.buffer_mut() {
          Ok(b) => b,
          Err(e) => {
            println!("surface buffer error: {e}");
            return;
          }
        };

        // 使用 zip 将 pixmap 像素和 buffer 像素一一对应
        for (target, src) in buffer.iter_mut().zip(self.pixmap.pixels().iter()) {
          let r = src.red() as u32;
          let g = src.green() as u32;
          let b = src.blue() as u32;

          // softbuffer 默认格式通常是 0x00RRGGBB
          *target = b | (g << 8) | (r << 16);
        }
        let _ = buffer.present();
      }
      // ==========================================
      WindowEvent::Resized(size) => {
        if size.width == 0 || size.height == 0 {
          return;
        }
        self.resize(size)
      }
      WindowEvent::KeyboardInput {
        event:
          KeyEvent {
            physical_key: PhysicalKey::Code(code),
            state: key_state,
            ..
          },
        ..
      } => match (code, key_state.is_pressed()) {
        (KeyCode::Escape, true) => event_loop.exit(),
        _ => {}
      },
      _ => {}
    }
  }
}
