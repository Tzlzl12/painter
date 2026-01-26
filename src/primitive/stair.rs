use crate::{
  drawable::{Bound, Drawable},
  primitive::config::Config,
};

use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Stroke, Transform};
pub enum StairStyle {
  TraceX,
  TraceY,
  Histogram,
}
pub struct Stair {
  name: String,
  stair_style: StairStyle,
  x: Vec<f32>,
  y: Vec<f32>,
  config: Config,
}

impl Stair {
  pub fn new(name: String, config: Config) -> Self {
    Self {
      name,
      x: Vec::new(),
      y: Vec::new(),
      config,
      stair_style: StairStyle::TraceX,
    }
  }

  fn _add_data(&mut self, x: &[f32], y: &[f32]) {
    self.x.extend_from_slice(x);
    self.y.extend_from_slice(y);
  }
  /// Sets the style for the stair visualization.
  ///
  /// # Arguments
  ///
  /// * `style` - The `StairStyle` to apply.
  pub fn set_style(&mut self, style: StairStyle) {
    self.stair_style = style;
  }
  /// Sets the data for the stair visualization.
  ///
  /// This method clears any existing data before adding the new data.
  ///
  /// # Arguments
  ///
  /// * `x` - A slice of f32 values representing the x-coordinates.
  /// * `y` - A slice of f32 values representing the y-coordinates.
  pub fn set_data(&mut self, x: &[f32], y: &[f32]) {
    self.x.clear();
    self.y.clear();
    self._add_data(x, y);
  }
}

impl Drawable for Stair {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform) {
    if self.x.len() < 2 {
      return;
    }

    let mut pb = PathBuilder::new();

    // 1. 处理第一个起点
    let mut p0 = Point::from_xy(self.x[0], self.y[0]);
    ts.map_point(&mut p0);
    pb.move_to(p0.x, p0.y);

    // 2. 遍历后续点，手动映射并构建阶梯
    for i in 0..self.x.len() - 1 {
      match self.stair_style {
        StairStyle::TraceX => {
          // (x_i, y_i) -> (x_i+1, y_i) -> (x_i+1, y_i+1)
          let mut p_corner = Point::from_xy(self.x[i + 1], self.y[i]);
          ts.map_point(&mut p_corner);
          pb.line_to(p_corner.x, p_corner.y);

          let mut p_next = Point::from_xy(self.x[i + 1], self.y[i + 1]);
          ts.map_point(&mut p_next);
          pb.line_to(p_next.x, p_next.y);
        }
        StairStyle::TraceY => {
          // (x_i, y_i) -> (x_i, y_i+1) -> (x_i+1, y_i+1)
          let mut p_corner = Point::from_xy(self.x[i], self.y[i + 1]);
          ts.map_point(&mut p_corner);
          pb.line_to(p_corner.x, p_corner.y);

          let mut p_next = Point::from_xy(self.x[i + 1], self.y[i + 1]);
          ts.map_point(&mut p_next);
          pb.line_to(p_next.x, p_next.y);
        }
        StairStyle::Histogram => {
          let mid_x = (self.x[i] + self.x[i + 1]) / 2.0;

          // (x_i, y_i) -> (mid_x, y_i) -> (mid_x, y_i+1) -> (x_i+1, y_i+1)
          let mut p1 = Point::from_xy(mid_x, self.y[i]);
          ts.map_point(&mut p1);
          pb.line_to(p1.x, p1.y);

          let mut p2 = Point::from_xy(mid_x, self.y[i + 1]);
          ts.map_point(&mut p2);
          pb.line_to(p2.x, p2.y);

          let mut p_next = Point::from_xy(self.x[i + 1], self.y[i + 1]);
          ts.map_point(&mut p_next);
          pb.line_to(p_next.x, p_next.y);
        }
      }
    }

    if let Some(path) = pb.finish() {
      let mut paint = Paint::default();
      let [r, g, b, a] = self.config.color;
      paint.set_color_rgba8(r, g, b, a);
      paint.anti_alias = true;

      let stroke = Stroke {
        width: self.config.stroke_width, // 这里直接用像素线宽，不用除以任何东西
        ..Default::default()
      };

      // 既然点已经 map 过了，这里必须传 identity()
      pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
  }

  fn bound(&self) -> Option<crate::drawable::Bound> {
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
