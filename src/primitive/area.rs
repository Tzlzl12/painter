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

  pub area_type: AreaType,

  config: RefCell<Config>,
}

impl Area {
  pub fn new(name: String, config: Config) -> Self {
    Area {
      name,
      x_edge: RefCell::new(Vec::new()),
      y_value: RefCell::new(Vec::new()),
      config: RefCell::new(config),
      area_type: AreaType::Step,
    }
  }
  fn _add_data(&self, x: &[f32], y: &[f32]) {
    self.x_edge.borrow_mut().extend_from_slice(x);
    self.y_value.borrow_mut().extend_from_slice(y);
  }
  pub fn set_data(&self, x: &[f32], y: &[f32]) {
    self.x_edge.borrow_mut().clear();
    self.y_value.borrow_mut().clear();
    self._add_data(x, y);
  }
}

impl Drawable for Area {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &tiny_skia::Transform) {
    let x_vec = self.x_edge.borrow();
    let y_vec = self.y_value.borrow();

    // 基础防御：如果没有数据点，直接返回
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
  fn get_color(&self) -> [u8; 4] {
    self.config.borrow().color
  }
  fn set_color(&self, color: [u8; 4]) {
    self.config.borrow_mut().color = color;
  }
}
