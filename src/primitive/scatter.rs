use std::cell::RefCell;

use tiny_skia::{FillRule, Paint, PathBuilder, Pixmap, Point, Transform};

use crate::{
  drawable::{Bound, Drawable},
  primitive::Config,
};

pub struct Scatter {
  name: String,
  x: RefCell<Vec<f32>>,
  y: RefCell<Vec<f32>>,
  value: RefCell<Option<Vec<f32>>>,
  forth_dim: RefCell<Option<Vec<f32>>>,
  config: RefCell<Config>,
}

impl Scatter {
  pub fn new(name: String, config: Config) -> Self {
    Self {
      name,
      x: RefCell::new(Vec::new()),
      y: RefCell::new(Vec::new()),
      value: RefCell::new(None),
      forth_dim: RefCell::new(None),
      config: RefCell::new(config),
    }
  }
  pub fn set_x(&self, x: &[f32]) {
    self.x.replace(x.to_vec());
  }
  /// set y values
  /// make sure `x` has been set
  pub fn set_y(&self, y: &[f32]) {
    if self.x.borrow().is_empty() {
      return;
    }
    let n = self.x.borrow().len();
    self.y.replace(y.iter().take(n).cloned().collect());
  }
  /// set value dimension, show by size
  /// make sure `x` has been set
  pub fn set_value(&self, values: &[f32]) {
    if self.x.borrow().is_empty() {
      return;
    }
    let n = self.x.borrow().len();
    self
      .value
      .replace(Some(values.iter().take(n).cloned().collect()));
  }
  /// set forth dimension, show by colors
  /// make sure `x` has been set
  pub fn set_forth_dim(&self, values: &[f32]) {
    if self.x.borrow().is_empty() {
      return;
    }
    let n = self.x.borrow().len();
    self
      .forth_dim
      .replace(Some(values.iter().take(n).cloned().collect()));
  }

  //===== functions for cahnge data =======

  pub fn change_y(&self, y: &[f32]) {
    if self.x.borrow().is_empty() {
      return;
    }
    let n = self.x.borrow().len();
    self.y.replace(y.iter().take(n).cloned().collect());
  }
  pub fn change_values(&self, values: &[f32]) {
    if self.x.borrow().is_empty() {
      return;
    }
    let n = self.x.borrow().len();
    self
      .value
      .replace(Some(values.iter().take(n).cloned().collect()));
  }
  pub fn change_forth_dim(&self, values: &[f32]) {
    if self.x.borrow().is_empty() {
      return;
    }
    let n = self.x.borrow().len();
    self
      .forth_dim
      .replace(Some(values.iter().take(n).cloned().collect()));
  }

  pub fn set_data_prototype(&self, y: &[f32], x_start: f32, step: f32) {
    let n = y.len();
    if n == 0 {
      return;
    }

    if !self.x.borrow().is_empty() {
      self.set_y(y);
      return;
    }

    self
      .x
      .replace((0..=n).map(|v| v as f32 * step + x_start).collect());
    self.set_y(y);
  }
  pub fn set_date_with_step(&self, y: &[f32], step: f32) {
    self.set_data_prototype(y, 0., step);
  }
  pub fn set_data_norm(&self, y: &[f32]) {
    self.set_data_prototype(y, 0., 1.);
  }
}

impl Drawable for Scatter {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform) {
    let x_vec = self.x.borrow();

    let y_vec = self.y.borrow();

    if x_vec.len() != y_vec.len() || x_vec.is_empty() {
      return;
    }

    let config = self.config.borrow();
    const RADIUS: f32 = 5.;

    let mut paint = Paint::default();
    let [r, g, b, a] = config.color;
    paint.anti_alias = true;

    let values;
    if self.value.borrow().is_none() {
      values = vec![1.0; x_vec.len()];
    } else {
      values = self.value.borrow().as_ref().unwrap().clone();
    }

    let mean = values.iter().sum::<f32>() / values.len() as f32;

    for i in 0..x_vec.len() {
      let mut center = Point::from_xy(x_vec[i], y_vec[i]);
      // switch paint to for range to prepare for setting color based on forth_dim
      paint.set_color_rgba8(r, g, b, a);
      ts.map_point(&mut center);

      let radius = (values[i] / mean).max(6.).min(1.) * RADIUS;

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
    let x_vec = self.x.borrow();
    let y_vec = self.y.borrow();

    if x_vec.is_empty() || y_vec.is_empty() || x_vec.len() != y_vec.len() {
      return None;
    }
    let padding = 1.0;

    let mut x_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;
    let mut y_min = f32::INFINITY;
    let mut y_max = f32::NEG_INFINITY;

    for (&xv, &yv) in x_vec.iter().zip(y_vec.iter()) {
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
    self.config.borrow().color
  }

  fn set_color(&self, color: [u8; 4]) {
    self.config.borrow_mut().color = color;
  }
}
