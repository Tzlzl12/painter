use std::cell::RefCell;

use tiny_skia::{FillRule, Paint, PathBuilder, Point, Stroke, Transform};

use crate::{
  drawable::{Bound, Drawable},
  primitive::Config,
};

#[derive(Debug)]
pub enum AreaType {
  Line,
  Step,
}
pub struct Area {
  name: String,
  x_edge: RefCell<Vec<f32>>,
  y_value: RefCell<Vec<f32>>,

  area_type: AreaType,

  config: RefCell<Config>,
}

impl Area {
  /// Creates a new `Area` with the given name and configuration.
  ///
  /// # Arguments
  ///
  /// * `name` - The name identifier for the area.
  /// * `config` - The configuration settings for the area.
  pub fn new(name: String, config: Config) -> Self {
    Area {
      name,
      x_edge: RefCell::new(Vec::new()),
      y_value: RefCell::new(Vec::new()),
      config: RefCell::new(config),
      area_type: AreaType::Step,
    }
  }
  /// Adds data to the existing x and y vectors.
  ///
  /// # Arguments
  ///
  /// * `x` - Slice of x coordinates to add.
  /// * `y` - Slice of y coordinates to add.
  ///
  /// # Note
  ///
  /// This function does nothing if the lengths of `x` and `y` do not match.
  fn _add_data(&self, x: &[f32], y: &[f32]) {
    if x.len() != y.len() {
      return;
    }
    self.x_edge.borrow_mut().extend_from_slice(x);
    self.y_value.borrow_mut().extend_from_slice(y);
  }
  /// Sets the data for the area by replacing existing values.
  ///
  /// # Arguments
  ///
  /// * `x` - Slice of x coordinates (edges).
  /// * `y` - Slice of y coordinates (values).
  ///
  /// # Note
  ///
  /// Prints a warning if `x.len() != y.len() - 1`.
  pub fn set_data(&self, x: &[f32], y: &[f32]) {
    if x.len() != y.len() - 1 {
      println!("make sure x length is less 1 then y length");
    }
    self.x_edge.replace(x.to_vec());
    self.y_value.replace(y.to_vec());
  }
  /// Sets data for the area using a prototype range definition.
  ///
  /// Generates x coordinates starting from `x_start` with a given `step`.
  ///
  /// # Arguments
  ///
  /// * `y` - Slice of y coordinates.
  /// * `x_start` - The starting value for the x coordinates.
  /// * `step` - The step size between consecutive x coordinates.
  /// # Note
  /// this `x` auto generated is not precise(need to optimize)
  pub fn set_data_prototype(&self, y: &[f32], x_start: f32, step: f32) {
    let n = y.len();
    if n == 0 {
      return;
    }
    self
      .x_edge
      .replace((0..=n).map(|i| x_start + i as f32 * step).collect());
    self.y_value.replace(y.to_vec());
  }
  /// Sets data for the area using a linear step for x coordinates.
  ///
  /// This is a convenience wrapper around `set_data_prototype`.
  ///
  /// # Arguments
  ///
  /// * `y` - Slice of y coordinates.
  /// * `x_start` - The starting value for the x coordinates.
  /// * `step` - The step size between consecutive x coordinates.
  pub fn set_data_with_step(&self, y: &[f32], x_start: f32, step: f32) {
    self.set_data_prototype(y, x_start, step);
  }
  /// Sets data for the area with normalized x coordinates (0, 1, 2, ...).
  ///
  /// This is a convenience wrapper around `set_data_prototype` with `x_start = 0.0` and `step = 1.0`.
  ///
  /// # Arguments
  ///
  /// * `y` - Slice of y coordinates.
  pub fn set_data_norm(&self, y: &[f32]) {
    self.set_data_prototype(y, 0.0, 1.0);
  }
  /// Changes the area type.
  ///
  /// # Arguments
  ///
  /// * `area_type` - The new area type.
  ///   * (`AreaType::Line` or `AreaType::Step`)
  pub fn change_area_type(&mut self, area_type: AreaType) {
    self.area_type = area_type;
  }
}

impl Drawable for Area {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &tiny_skia::Transform) {
    let x_vec = self.x_edge.borrow();
    let y_vec = self.y_value.borrow();

    if x_vec.is_empty() {
      return;
    }

    let mut pb = PathBuilder::new();
    let baseline = 0.0;

    match self.area_type {
      // --- 模式 1: Step (阶梯状/柱状面积) ---
      AreaType::Step => {
        // 如果只有一位 x，自动补齐为从 0.0 到 x[0]
        let edges: Vec<f32> = if x_vec.len() == 1 {
          vec![0.0, x_vec[0]]
        } else {
          x_vec.clone()
        };

        for i in 0..edges.len() - 1 {
          let x_l = edges[i];
          let x_r = edges[i + 1];
          let y_val = y_vec.get(i).cloned().unwrap_or(0.0);

          // 构造矩形的四个点并映射
          let mut p1 = Point::from_xy(x_l, baseline);
          let mut p2 = Point::from_xy(x_l, y_val);
          let mut p3 = Point::from_xy(x_r, y_val);
          let mut p4 = Point::from_xy(x_r, baseline);

          ts.map_point(&mut p1);
          ts.map_point(&mut p2);
          ts.map_point(&mut p3);
          ts.map_point(&mut p4);

          pb.move_to(p1.x, p1.y);
          pb.line_to(p2.x, p2.y);
          pb.line_to(p3.x, p3.y);
          pb.line_to(p4.x, p4.y);
          pb.close();
        }
      }

      // --- 模式 2: Curve (折线面积填充) ---
      AreaType::Line => {
        if !x_vec.is_empty() {
          // 1. 起点：(x_0, baseline)
          let mut p_start = Point::from_xy(x_vec[0], baseline);
          ts.map_point(&mut p_start);
          pb.move_to(p_start.x, p_start.y);

          // 2. 连接所有数据点 (x_i, y_i)
          for i in 0..x_vec.len() {
            let y_val = y_vec.get(i).cloned().unwrap_or(0.0);
            let mut p = Point::from_xy(x_vec[i], y_val);
            ts.map_point(&mut p);
            pb.line_to(p.x, p.y);
          }

          // 3. 终点：(x_last, baseline)
          let mut p_end = Point::from_xy(*x_vec.last().unwrap(), baseline);
          ts.map_point(&mut p_end);
          pb.line_to(p_end.x, p_end.y);
          pb.close();
        }
      }
    }

    // --- 统一渲染逻辑 ---
    if let Some(path) = pb.finish() {
      let config = self.config.borrow();
      let mut paint = Paint::default();
      let [r, g, b, a] = config.color;

      // 1. 填充路径 (使用半透明色)
      paint.set_color_rgba8(r, g, b, a / 2);
      paint.anti_alias = true;
      pixmap.fill_path(
        &path,
        &paint,
        FillRule::Winding,
        Transform::identity(),
        None,
      );

      // 2. 描边 (使用相同的颜色，或者你可以根据需求调深一点)
      let stroke = Stroke {
        width: config.stroke_width,
        ..Default::default()
      };
      pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
  }
  fn bound(&self) -> Option<Bound> {
    if self.x_edge.borrow().is_empty() || self.y_value.borrow().is_empty() {
      return None;
    }
    let x_min = self
      .x_edge
      .borrow()
      .iter()
      .fold(f32::INFINITY, |a, &b| a.min(b));
    let x_max = self
      .x_edge
      .borrow()
      .iter()
      .fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let y_min = self
      .y_value
      .borrow()
      .iter()
      .fold(f32::INFINITY, |a, &b| a.min(b));
    let y_max = self
      .y_value
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
