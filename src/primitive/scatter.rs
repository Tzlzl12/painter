use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Point, Transform};

use crate::{
  drawable::{Bound, Drawable},
  primitive::Config,
};

/// Scatter
/// * `y` is the value in y coordinate
/// * `value`  is shown by the circle size
/// * `forth_dim` is shown by the circle color(not implement)
pub struct Scatter {
  name: String,
  x: Vec<f32>,
  y: Vec<f32>,
  value: Option<Vec<f32>>,
  forth_dim: Option<Vec<f32>>,
  config: Config,
}

impl Scatter {
  pub fn new(name: String, config: Config) -> Self {
    Self {
      name,
      x: Vec::new(),
      y: Vec::new(),
      value: None,
      forth_dim: None,
      config,
    }
  }
  pub fn set_x(&mut self, x: &[f32]) {
    self.x = x.to_vec();
  }
  /// set y values
  /// make sure `x` has been set
  pub fn set_y(&mut self, y: &[f32]) {
    if self.x.is_empty() {
      return;
    }
    let n = self.x.len();
    self.y = y.iter().take(n).cloned().collect();
  }
  /// set value dimension, show by size
  /// make sure `x` has been set
  pub fn set_value(&mut self, values: &[f32]) {
    if self.x.is_empty() {
      return;
    }
    let n = self.x.len();
    self.value = Some(values.iter().take(n).cloned().collect());
  }
  /// set forth dimension, show by colors
  /// make sure `x` has been set
  pub fn set_forth_dim(&mut self, values: &[f32]) {
    if self.x.is_empty() {
      return;
    }
    let n = self.x.len();
    self.forth_dim = Some(values.iter().take(n).cloned().collect());
  }

  //===== functions for change data =======

  /// Change y values
  /// make sure `x` has been set
  pub fn change_y(&mut self, y: &[f32]) {
    if self.x.is_empty() {
      return;
    }
    let n = self.x.len();
    self.y = y.iter().take(n).cloned().collect();
  }
  /// Change value dimension
  /// make sure `x` has been set
  pub fn change_values(&mut self, values: &[f32]) {
    if self.x.is_empty() {
      return;
    }
    let n = self.x.len();
    self.value = Some(values.iter().take(n).cloned().collect());
  }
  /// Change forth dimension
  /// make sure `x` has been set
  pub fn change_forth_dim(&mut self, values: &[f32]) {
    if self.x.is_empty() {
      return;
    }
    let n = self.x.len();
    self.forth_dim = Some(values.iter().take(n).cloned().collect());
  }

  /// Set data prototype with y values, x start position and step
  /// If x is already set, only y will be updated
  pub fn set_data_prototype(&mut self, y: &[f32], x_start: f32, step: f32) {
    let n = y.len();
    if n == 0 {
      return;
    }

    if !self.x.is_empty() {
      self.set_y(y);
      return;
    }

    self.x = (0..=n).map(|v| v as f32 * step + x_start).collect();
    self.set_y(y);
  }
  /// Set data with y values and step starting from 0
  pub fn set_date_with_step(&mut self, y: &[f32], step: f32) {
    self.set_data_prototype(y, 0., step);
  }
  /// Set normalized data with y values starting from 0 with step 1
  pub fn set_data_norm(&mut self, y: &[f32]) {
    self.set_data_prototype(y, 0., 1.);
  }
}

impl Drawable for Scatter {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform) {
    if self.x.len() != self.y.len() || self.x.is_empty() {
      return;
    }

    const RADIUS: f32 = 5.;

    let mut paint = Paint::default();
    let [r, g, b, a] = self.config.color;
    paint.anti_alias = true;

    let values;
    if self.value.is_none() {
      values = vec![1.0; self.x.len()];
    } else {
      values = self.value.as_ref().unwrap().clone();
    }

    let mean = values.iter().sum::<f32>() / values.len() as f32;

    for i in 0..self.x.len() {
      let mut center = Point::from_xy(self.x[i], self.y[i]);
      // switch paint to for range to prepare for setting color based on forth_dim
      paint.set_color_rgba8(r, g, b, a);
      ts.map_point(&mut center);

      let radius = (values[i] / mean).clamp(1., 6.) * RADIUS;

      // 简单圆形散点
      if let Some(circle) = PathBuilder::from_circle(center.x, center.y, radius) {
        pixmap.fill_path(
          &circle,
          &paint,
          FillRule::Winding,
          Transform::identity(),
          None,
        );
      }
    }
  }

  fn bound(&self) -> Option<Bound> {
    if self.x.is_empty() || self.y.is_empty() || self.x.len() != self.y.len() {
      return None;
    }
    let padding = 1.0;

    let mut x_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;
    let mut y_min = f32::INFINITY;
    let mut y_max = f32::NEG_INFINITY;

    for (&xv, &yv) in self.x.iter().zip(self.y.iter()) {
      x_min = x_min.min(xv);
      x_max = x_max.max(xv);
      y_min = y_min.min(yv);
      y_max = y_max.max(yv);
    }

    // 重点 2：对称补偿
    Some(Bound {
      x_min: if x_min == 0. { 0. } else { x_min - padding },
      x_max: x_max + padding,
      y_min: if y_min == 0. { 0. } else { y_min - padding },
      y_max: y_max + padding,
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
