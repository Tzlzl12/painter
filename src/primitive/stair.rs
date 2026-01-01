use crate::{
  drawable::{Bound, Drawable},
  primitive::config::Config,
};
use std::cell::RefCell;

use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Stroke, Transform};
use winit::event::ElementState;
pub enum StairStyle {
  trace_x,
  trace_y,
  histogram,
}
pub struct Stair {
  name: String,
  stair_style: RefCell<StairStyle>,
  x: RefCell<Vec<f32>>,
  y: RefCell<Vec<f32>>,
  config: RefCell<Config>,
}

impl Stair {
  pub fn new(name: String, config: Config) -> Self {
    Self {
      name,
      x: RefCell::new(Vec::new()),
      y: RefCell::new(Vec::new()),
      config: RefCell::new(config),
      stair_style: RefCell::new(StairStyle::trace_x),
    }
  }

  fn _add_data(&self, x: &[f32], y: &[f32]) {
    self.x.borrow_mut().extend_from_slice(x);
    self.y.borrow_mut().extend_from_slice(y);
  }
  pub fn set_style(&self, style: StairStyle) {
    *self.stair_style.borrow_mut() = style;
  }
  pub fn set_data(&self, x: &[f32], y: &[f32]) {
    self.x.borrow_mut().clear();
    self.y.borrow_mut().clear();
    self._add_data(x, y);
  }
}

impl Drawable for Stair {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform) {
    let x_vec = self.x.borrow();
    let y_vec = self.y.borrow();
    if x_vec.len() < 2 {
      return;
    }

    let mut pb = PathBuilder::new();

    // 1. 处理第一个起点
    let mut p0 = Point::from_xy(x_vec[0], y_vec[0]);
    ts.map_point(&mut p0);
    pb.move_to(p0.x, p0.y);

    // 2. 遍历后续点，手动映射并构建阶梯
    for i in 0..x_vec.len() - 1 {
      match *self.stair_style.borrow() {
        StairStyle::trace_x => {
          // (x_i, y_i) -> (x_i+1, y_i) -> (x_i+1, y_i+1)
          let mut p_corner = Point::from_xy(x_vec[i + 1], y_vec[i]);
          ts.map_point(&mut p_corner);
          pb.line_to(p_corner.x, p_corner.y);

          let mut p_next = Point::from_xy(x_vec[i + 1], y_vec[i + 1]);
          ts.map_point(&mut p_next);
          pb.line_to(p_next.x, p_next.y);
        }
        StairStyle::trace_y => {
          // (x_i, y_i) -> (x_i, y_i+1) -> (x_i+1, y_i+1)
          let mut p_corner = Point::from_xy(x_vec[i], y_vec[i + 1]);
          ts.map_point(&mut p_corner);
          pb.line_to(p_corner.x, p_corner.y);

          let mut p_next = Point::from_xy(x_vec[i + 1], y_vec[i + 1]);
          ts.map_point(&mut p_next);
          pb.line_to(p_next.x, p_next.y);
        }
        StairStyle::histogram => {
          let mid_x = (x_vec[i] + x_vec[i + 1]) / 2.0;

          // (x_i, y_i) -> (mid_x, y_i) -> (mid_x, y_i+1) -> (x_i+1, y_i+1)
          let mut p1 = Point::from_xy(mid_x, y_vec[i]);
          ts.map_point(&mut p1);
          pb.line_to(p1.x, p1.y);

          let mut p2 = Point::from_xy(mid_x, y_vec[i + 1]);
          ts.map_point(&mut p2);
          pb.line_to(p2.x, p2.y);

          let mut p_next = Point::from_xy(x_vec[i + 1], y_vec[i + 1]);
          ts.map_point(&mut p_next);
          pb.line_to(p_next.x, p_next.y);
        }
      }
    }

    if let Some(path) = pb.finish() {
      let mut paint = Paint::default();
      let config = self.config.borrow();
      let [r, g, b, a] = config.color;
      paint.set_color_rgba8(r, g, b, a);
      paint.anti_alias = true;

      let stroke = Stroke {
        width: config.stroke_width, // 这里直接用像素线宽，不用除以任何东西
        ..Default::default()
      };

      // 既然点已经 map 过了，这里必须传 identity()
      pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
  }

  fn bound(&self) -> Option<crate::drawable::Bound> {
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
