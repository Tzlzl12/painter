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
  /// Sets the data points from separate x and y coordinate slices.
  ///
  /// # Arguments
  ///
  /// * `x` - A slice of x-coordinates.
  /// * `y` - A slice of y-coordinates.
  pub fn set_data(&self, x: &[f32], y: &[f32]) {
    self.x.replace(x.to_vec());
    self.y.replace(y.to_vec());
  }
  /// Sets the data points from x-coordinates and a function.
  ///
  /// # Arguments
  ///
  /// * `x` - A slice of x-coordinates.
  /// * `f` - A function that maps each x-coordinate to a y-coordinate.
  pub fn set_fn(&self, x: &[f32], f: impl Fn(f32) -> f32) {
    self.x.replace(x.to_vec());
    self.y.replace(x.iter().map(|&v| f(v)).collect());
  }
  /// Sets the data points from a parameter t and functions for x and y.
  ///
  /// # Arguments
  ///
  /// * `t` - A slice of parameter values.
  /// * `fx` - A function that maps each t value to an x-coordinate.
  /// * `fy` - A function that maps each t value to a y-coordinate.
  pub fn set_parametric(&self, t: &[f32], fx: impl Fn(f32) -> f32, fy: impl Fn(f32) -> f32) {
    self.x.replace(t.iter().map(|&v| fx(v)).collect());
    self.y.replace(t.iter().map(|&v| fy(v)).collect());
  }
}

impl Drawable for Curve {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &Transform) {
    let config = self.config.borrow();
    if config.is_hidden || self.x.borrow().is_empty() || self.y.borrow().is_empty() {
      return;
    }

    let mut pb = PathBuilder::new();
    let x_ref = self.x.borrow();
    let y_ref = self.y.borrow();

    // 1. 直接使用原始数据坐标（逻辑坐标）
    for i in 0..x_ref.len() {
      let mut point = Point::from_xy(x_ref[i], y_ref[i]);
      ts.map_point(&mut point);
      if i == 0 {
        pb.move_to(point.x, point.y);
      } else {
        pb.line_to(point.x, point.y);
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
      let stroke = Stroke {
        width: config.stroke_width, // 抵消变换带来的缩放
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
  fn name(&self) -> String {
    self.name.clone()
  }
  fn get_color(&self) -> [u8; 4] {
    self.config.borrow().color
  }
  fn set_color(&self, color: [u8; 4]) {
    self.config.borrow_mut().color = color;
  }
}
