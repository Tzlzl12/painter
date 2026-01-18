use crate::color;
use std::cell::{Cell, RefCell};
use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Rect, Stroke, Transform};

use crate::{
  drawable::{Bound, Drawable},
  primitive::Config,
};

struct Bars {
  y: Vec<f32>,

  config: Config,
}
impl Bars {
  pub fn new(y: Vec<f32>, config: Config) -> Self {
    Self { y, config }
  }
}

pub struct Histrogram {
  name: String,
  x: RefCell<Vec<f32>>,
  bars: RefCell<Vec<Bars>>,

  color_index: Cell<usize>,
}

impl Histrogram {
  pub fn new(name: String) -> Self {
    Self {
      name,
      x: RefCell::new(Vec::new()),
      bars: RefCell::new(Vec::new()),
      color_index: Cell::new(0),
    }
  }
  fn max_len(&self) -> usize {
    self.x.borrow().len().saturating_sub(1)
  }
  fn resize(&self, final_limit: usize) {
    for bar in self.bars.borrow_mut().iter_mut() {
      if bar.y.len() != final_limit {
        bar.y.resize(final_limit, 0.0);
      }
    }
  }
  pub fn set_x(&self, x: &[f32]) {
    let mut x_vec = self.x.borrow_mut();
    let limit = x.len();
    x_vec.clear();
    x_vec.extend_from_slice(x);

    if self.bars.borrow().is_empty() {
      self.resize(limit);
    }
  }
  pub fn set_data(&self, y: &[f32]) {
    let mut bars_vec = self.bars.borrow_mut();

    // auto choose color
    let color = color::get_color(self.color_index.get());
    let index = self.color_index.get();
    self.color_index.set((index + 1) & 7);

    let config = Config {
      color,
      ..Config::default()
    };
    // get the value based on the max_len(x)
    let valid_value: Vec<f32> = y.iter().take(self.max_len()).cloned().collect();
    let bar = Bars::new(valid_value, config);
    bars_vec.push(bar);
  }
  // althougn the function is supported, but not recommended to use it
  pub fn add_data(&self, index: usize, y: &[f32]) {
    let mut bars_vec = self.bars.borrow_mut();
    if let Some(bar) = bars_vec.get_mut(index) {
      let limit = self.max_len();
      let current_len = bar.y.len();

      if current_len < limit {
        let should_add_len = limit - current_len;
        bar.y.extend(y.iter().take(should_add_len).cloned());
      }
    }
  }
  pub fn change_data(&mut self, index: usize, y: &[f32]) {
    let mut bars_vec = self.bars.borrow_mut();
    if let Some(bar) = bars_vec.get_mut(index) {
      let limit = self.max_len();
      let new_y: Vec<f32> = y.iter().take(limit).cloned().collect();
      bar.y.clear();
      bar.y.extend(new_y);
    }
  }
  pub fn set_data_prototype(&self, y: &[f32], x_start: f32, step: f32) {
    let n = y.len();
    if n == 0 {
      return;
    }

    // if the x is not set, set it first
    let mut x_vec = self.x.borrow_mut();
    if x_vec.is_empty() {
      let mut new_x = Vec::with_capacity(n + 1);
      for i in 0..=n {
        new_x.push(x_start + i as f32 * step);
      }
      *x_vec = new_x;

      // get the max bins
      let limit = x_vec.len().saturating_sub(1);

      // auto set color
      let idx = self.color_index.get();
      let color = color::get_color(idx);
      self.color_index.set((idx + 1) & 7);

      let config = Config {
        color,
        ..Config::default()
      };

      // to the same length as x bins
      let final_y = y.iter().take(limit).cloned().collect::<Vec<f32>>();
      self.bars.borrow_mut().push(Bars::new(final_y, config));
    } else {
      // drop x reference, to avoid set_data() use it
      drop(x_vec);
      // if have x, just set data
      self.set_data(y);
    }
  }
  pub fn set_data_with_step(&self, y: &[f32], step: f32) {
    self.set_data_prototype(y, 0., step);
  }
  pub fn set_data_norm(&self, y: &[f32]) {
    self.set_data_prototype(y, 0., 1.);
  }
}

impl Drawable for Histrogram {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform) {
    let bars_vec = self.bars.borrow();
    let x_ref = self.x.borrow();

    // 只有当至少有两个刻度（一个槽位）且有数据组时才绘制
    if x_ref.len() < 2 || bars_vec.is_empty() {
      return;
    }

    let num_groups = bars_vec.len() as f32;
    let group_width_ratio = 0.8;

    // 预计算一些不变的常量
    let half_num_groups = (num_groups - 1.0) / 2.0;

    // 遍历槽位：i 是槽位索引，x_start/x_end 是左右边界
    for i in 0..(x_ref.len() - 1) {
      let x_start = x_ref[i];
      let x_end = x_ref[i + 1];
      let total_step_w = x_end - x_start;
      let x_center = x_start + total_step_w * 0.5;
      let single_bar_w = (total_step_w * group_width_ratio) / num_groups;

      for (g_idx, bar) in bars_vec.iter().enumerate() {
        if bar.config.is_hidden {
          continue;
        }

        let y_val = bar.y[i];
        if y_val == 0.0 {
          continue;
        } // 零值不画，节省性能

        let offset = (g_idx as f32 - half_num_groups) * single_bar_w;
        let x_l = x_center + offset - single_bar_w * 0.5;
        let x_r = x_center + offset + single_bar_w * 0.5;

        // 映射并构造矩形
        let mut p1 = Point::from_xy(x_l, y_val);
        let mut p2 = Point::from_xy(x_r, 0.0);
        ts.map_point(&mut p1);
        ts.map_point(&mut p2);

        if let Some(r_rect) = Rect::from_ltrb(
          p1.x.min(p2.x),
          p1.y.min(p2.y),
          p1.x.max(p2.x),
          p1.y.max(p2.y),
        ) {
          let [r, g, b, a] = bar.config.color;
          let mut paint = Paint::default();
          paint.set_color_rgba8(r, g, b, a);
          paint.anti_alias = true;

          pixmap.fill_rect(r_rect, &paint, Transform::identity(), None);

          // 描边
          let path = PathBuilder::from_rect(r_rect);
          let mut stroke_paint = Paint::default();
          stroke_paint.set_color_rgba8(
            r.saturating_sub(40),
            g.saturating_sub(40),
            b.saturating_sub(40),
            255,
          );
          let stroke = Stroke {
            width: bar.config.stroke_width,
            ..Default::default()
          };
          pixmap.stroke_path(&path, &stroke_paint, &stroke, Transform::identity(), None);
        }
      }
    }
  }

  fn bound(&self) -> Option<Bound> {
    if self.x.borrow().is_empty() {
      return None;
    }

    let bars_vec = self.bars.borrow();
    let mut y_max = 0.0f32;
    let mut y_min = 0.0f32;
    let mut has_data = false;

    for bar in bars_vec.iter() {
      if bar.config.is_hidden {
        continue;
      }
      for &y in &bar.y {
        y_max = y_max.max(y);
        y_min = y_min.min(y);
        has_data = true;
      }
    }

    if !has_data {
      return None;
    }

    Some(Bound {
      x_min: *self.x.borrow().first().unwrap(),
      x_max: *self.x.borrow().last().unwrap(),
      y_min,
      y_max,
    })
  }
  fn name(&self) -> String {
    self.name.clone()
  }
  /// not used in this primitive
  fn get_color(&self) -> [u8; 4] {
    [255, 255, 255, 255]
  }
  /// not used in this primitive
  fn set_color(&self, _color: [u8; 4]) {}
}
