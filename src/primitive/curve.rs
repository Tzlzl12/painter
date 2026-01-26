use tiny_skia::{Paint, PathBuilder, Point, Stroke, Transform};

use super::config::Config;
use crate::drawable::{Bound, Drawable};

pub struct Curve {
  name: String,
  x: Vec<f32>,
  y: Vec<f32>,
  config: Config,
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
  fn _add_data(&mut self, x: &[f32], y: &[f32]) {
    self.x.extend_from_slice(x);
    self.y.extend_from_slice(y);
  }
  pub fn add_data(&mut self, x: &[f32], y: &[f32]) {
    self._add_data(x, y);
  }
  /// Sets the data points from separate x and y coordinate slices.
  ///
  /// # Arguments
  ///
  /// * `x` - A slice of x-coordinates.
  /// * `y` - A slice of y-coordinates.
  pub fn set_data(&mut self, x: &[f32], y: &[f32]) {
    self.x = x.to_vec();
    self.y = y.to_vec();
  }
  /// Sets the data points from x-coordinates and a function.
  ///
  /// # Arguments
  ///
  /// * `x` - A slice of x-coordinates.
  /// * `f` - A function that maps each x-coordinate to a y-coordinate.
  pub fn set_fn(&mut self, x: &[f32], f: impl Fn(f32) -> f32) {
    self.x = x.to_vec();
    self.y = x.iter().map(|&v| f(v)).collect();
  }
  /// Sets the data points from a parameter t and functions for x and y.
  ///
  /// # Arguments
  ///
  /// * `t` - A slice of parameter values.
  /// * `fx` - A function that maps each t value to an x-coordinate.
  /// * `fy` - A function that maps each t value to a y-coordinate.
  pub fn set_parametric(&mut self, t: &[f32], fx: impl Fn(f32) -> f32, fy: impl Fn(f32) -> f32) {
    self.x = t.iter().map(|&v| fx(v)).collect();
    self.y = t.iter().map(|&v| fy(v)).collect();
  }
}

impl Drawable for Curve {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &Transform) {
    if self.config.is_hidden || self.x.is_empty() || self.y.is_empty() {
      return;
    }

    let mut pb = PathBuilder::new();

    for i in 0..self.x.len() {
      let mut point = Point::from_xy(self.x[i], self.y[i]);
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
        self.config.color[0],
        self.config.color[1],
        self.config.color[2],
        self.config.color[3],
      );
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
  fn name(&self) -> String {
    self.name.clone()
  }
  fn get_color(&self) -> [u8; 4] {
    self.config.color
  }
  fn set_color(&mut self, color: [u8; 4]) {
    self.config.color = color;
  }
}
