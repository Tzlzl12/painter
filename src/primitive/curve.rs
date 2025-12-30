use std::cell::RefCell;

use tiny_skia::{Paint, PathBuilder, Point, Stroke, Transform};

use super::config::Config;
use crate::drawable::{Bound, Drawable};

pub struct Curve {
  name: String,
  x: RefCell<Vec<f32>>,
  y: RefCell<Vec<f32>>,
  config: RefCell<Config>,
}

impl Curve {
  pub fn new(name: String, config: Config) -> Self {
    Self {
      name,
      x: RefCell::new(Vec::new()),
      y: RefCell::new(Vec::new()),
      config: RefCell::new(config),
    }
  }
  fn _add_data(&self, x: &[f32], y: &[f32]) {
    self.x.borrow_mut().extend_from_slice(x);
    self.y.borrow_mut().extend_from_slice(y);
  }
  pub fn add_data(&self, x: &[f32], y: &[f32]) {
    self._add_data(x, y);
  }
  pub fn set_data(&self, x: &[f32], y: &[f32]) {
    self.x.borrow_mut().clear();
    self.y.borrow_mut().clear();
    self._add_data(x, y);
  }
  pub fn set_fn(&self, x: &[f32], f: impl Fn(f32) -> f32) {
    let mut x_vec = self.x.borrow_mut();
    let mut y_vec = self.y.borrow_mut();

    x_vec.clear();
    y_vec.clear();

    x_vec.extend_from_slice(x);
    y_vec.extend(x.iter().map(|&v| f(v)));
  }
  pub fn set_parametric(&self, t: &[f32], fx: impl Fn(f32) -> f32, fy: impl Fn(f32) -> f32) {
    let mut x_vec = self.x.borrow_mut();
    let mut y_vec = self.y.borrow_mut();
    x_vec.clear();
    y_vec.clear();

    x_vec.extend(t.iter().map(|&v| fx(v)));
    y_vec.extend(t.iter().map(|&v| fy(v)));
  }
}

impl Drawable for Curve {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &Transform) {
    let config = self.config.borrow();
    if config.is_hidden || self.x.borrow().is_empty() || self.y.borrow().is_empty() {
      return;
    }

    let mut pb = PathBuilder::new();

    let mut first = true;
    for (&x, &y) in self.x.borrow().iter().zip(self.y.borrow().iter()) {
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
        config.color[0],
        config.color[1],
        config.color[2],
        config.color[3],
      );
      paint.anti_alias = true;

      let scale = ts.get_scale();
      let stroke = Stroke {
        width: config.stroke_width,
        line_cap: tiny_skia::LineCap::Round,
        line_join: tiny_skia::LineJoin::Round,
        ..Stroke::default()
      };
      pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
  }
  fn bound(&self) -> Option<Bound> {
    if self.x.borrow().is_empty() || self.y.borrow().is_empty() {
      return None;
    }
    let x_min = self.x.borrow().iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let x_max = self
      .x
      .borrow()
      .iter()
      .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = self.y.borrow().iter().fold(f32::INFINITY, |a, &b| a.min(b));
    let y_max = self
      .y
      .borrow()
      .iter()
      .fold(f32::NEG_INFINITY, |a, &b| a.max(b));

    Some(Bound {
      x_min,
      x_max,
      y_min,
      y_max,
    })
  }
  fn get_color(&self) -> [u8; 4] {
    self.config.borrow().color
  }
  fn set_color(&self, color: [u8; 4]) {
    self.config.borrow_mut().color = color;
  }
}
