use std::sync::PoisonError;

use tiny_skia::{Paint, PathBuilder, Point, Stroke, Transform};

use crate::drawable::Drawable;

pub struct Curve {
  name: String,
  data: Vec<(f32, f32)>,
  config: Config,
}

pub struct Config {
  is_hidden: bool,
  color: [u8; 4],
  stroke_width: f32,
}
impl Default for Config {
  fn default() -> Self {
    Self {
      is_hidden: false,
      color: [255, 255, 255, 255],
      stroke_width: 2.0,
    }
  }
}

impl Curve {
  pub fn new(name: String, config: Config) -> Self {
    Self {
      name,
      data: Vec::new(),
      config,
    }
  }
  pub fn add_data(&mut self, data: Vec<(f32, f32)>) {
    self.data.extend(data);
  }
}

impl Drawable for Curve {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &Transform) {
    if self.config.is_hidden || self.data.is_empty() {
      return;
    }

    let mut pb = PathBuilder::new();

    for (i, &(x, y)) in self.data.iter().enumerate() {
      if i == 0 {
        pb.move_to(x, y);
      } else {
        pb.line_to(x, y);
      }
    }

    if let Some(path) = pb.finish() {
      let mut paint = Paint::default();
      paint.set_color_rgba8(
        self.config.color[0],
        self.config.color[1],
        self.config.color[2],
        self.config.color[3],
      );
      paint.anti_alias = true;

      let stroke = Stroke {
        width: self.config.stroke_width,
        ..Stroke::default()
      };
      pixmap.stroke_path(&path, &paint, &stroke, *ts, None);
    }
  }
}
