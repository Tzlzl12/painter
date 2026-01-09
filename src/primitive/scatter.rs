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
  value: Option<RefCell<Vec<f32>>>,
  forth_dim: Option<RefCell<Vec<f32>>>,
  config: RefCell<Config>,
}

impl Scatter {
  pub fn new(name: String, config: Config) -> Self {
    Self {
      name,
      x: RefCell::new(Vec::new()),
      y: RefCell::new(Vec::new()),
      value: None,
      forth_dim: None,
      config: RefCell::new(config),
    }
  }
  pub fn set_data(&self, x: &[f32], y: &[f32]) {
    self.x.replace(x.to_vec());
    self.y.replace(y.to_vec());
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
    let radius = 5.; // 至少1像素，避免看不见

    let mut paint = Paint::default();
    let [r, g, b, a] = config.color;
    paint.set_color_rgba8(r, g, b, a);
    paint.anti_alias = true;

    for i in 0..x_vec.len() {
      let mut center = Point::from_xy(x_vec[i], y_vec[i]);
      ts.map_point(&mut center);

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

    // 重点 1：如果这个 radius 是为了防止圆点被切边，
    // 它应该是一个非常小的逻辑数值，或者干脆设为 0 调试。
    // 之前你设为 10.0，而 x 范围才 [0, 22]，这 10.0 太大了！
    let padding = 0.0;

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
