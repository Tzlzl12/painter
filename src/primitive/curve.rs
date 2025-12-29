use tiny_skia::{Paint, PathBuilder, Point, Stroke, Transform};

use crate::drawable::{Bound, Drawable};

pub struct Curve {
  name: String,
  x: Vec<f32>,
  y: Vec<f32>,
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
      x: Vec::new(),
      y: Vec::new(),
      config,
    }
  }
  pub fn add_data(&mut self, x: &[f32], y: &[f32]) {
    self.x.extend_from_slice(x);
    self.y.extend_from_slice(y);
  }
}

impl Drawable for Curve {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &Transform) {
    if self.config.is_hidden || self.x.is_empty() || self.y.is_empty() {
      return;
    }

    let mut pb = PathBuilder::new();

    let mut first = true;
    for (&x, &y) in self.x.iter().zip(self.y.iter()) {
      let mut p = Point::from_xy(x, y);
      ts.map_point(&mut p);
      if first {
        pb.move_to(p.x, p.y);
        first = false;
      } else {
        pb.line_to(p.x, p.y);
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

      let scale = ts.get_scale();
      let stroke = Stroke {
        width: self.config.stroke_width,
        line_cap: tiny_skia::LineCap::Round,
        line_join: tiny_skia::LineJoin::Round,
        ..Stroke::default()
      };
      pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
  }
  fn bound(&self) -> Option<Bound> {
    if self.x.is_empty() || self.y.is_empty() {
      return None;
    }
    let x_min = self.x.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let x_max = self.x.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = self.y.iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let y_max = self.y.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));

    Some(Bound {
      x_min,
      x_max,
      y_min,
      y_max,
    })
  }
}
