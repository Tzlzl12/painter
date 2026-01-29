use crate::color;
use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Rect, Stroke, Transform};

use crate::{
  drawable::{Bound, Drawable},
  primitive::Config,
};

pub(crate) struct Bars {
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
  x: Vec<f32>,
  bars: Vec<Bars>,

  color_index: usize,
}

impl Histrogram {
  pub fn new(name: String) -> Self {
    Self {
      name,
      x: Vec::new(),
      bars: Vec::new(),
      color_index: 0,
    }
  }
  fn max_len(&self) -> usize {
    self.x.len().saturating_sub(1)
  }
  fn resize(&mut self, final_limit: usize) {
    for bar in self.bars.iter_mut() {
      if bar.y.len() != final_limit {
        bar.y.resize(final_limit, 0.0);
      }
    }
  }
  /// Sets the x-axis values for the chart.
  ///
  /// # Arguments
  ///
  /// * `x` - A slice of f32 values representing the x-axis coordinates.
  pub fn set_x(&mut self, x: &[f32]) {
    let limit = x.len();
    self.x.clear();
    self.x.extend_from_slice(x);

    if self.bars.is_empty() {
      self.resize(limit);
    }
  }
  pub(crate) fn gen_config(&mut self) -> Config {
    // auto choose color
    let color = color::get_color(self.color_index);
    let index = self.color_index;
    self.color_index = (index + 1) & 7;

    Config {
      color,
      ..Config::default()
    }
  }
  pub(crate) fn get_color_index(&self) -> usize {
    self.color_index
  }
  /// Adds a new data series (bars) to the chart.
  ///
  /// Automatically assigns a color to the new series using an internal color index.
  ///
  /// # Arguments
  ///
  /// * `y` - A slice of f32 values representing the y-axis data.
  pub fn set_data(&mut self, y: &[f32]) {
    let config = self.gen_config();
    // get the value based on the max_len(x)
    let valid_value: Vec<f32> = y.iter().take(self.max_len()).cloned().collect();
    let bar = Bars::new(valid_value, config);
    self.bars.push(bar);
  }
  /// Adds data to an existing data series at the specified index.
  ///
  /// This function is supported but not recommended for general use.
  ///
  /// # Arguments
  ///
  /// * `index` - The index of the data series to modify.
  /// * `y` - A slice of f32 values to append to the series.
  /// # Note
  /// this method is not recommended
  pub fn add_data(&mut self, index: usize, y: &[f32]) {
    let limit = self.max_len();
    if let Some(bar) = self.bars.get_mut(index) {
      let current_len = bar.y.len();

      if current_len < limit {
        let should_add_len = limit - current_len;
        bar.y.extend(y.iter().take(should_add_len).cloned());
      }
    }
  }
  /// Changes the data of an existing data series at the specified index.
  ///
  /// Replaces the existing data with the new values provided.
  ///
  /// # Arguments
  ///
  /// * `index` - The index of the data series to modify.
  /// * `y` - A slice of f32 values to set as the new data.
  pub fn change_data(&mut self, index: usize, y: &[f32]) {
    let limit = self.max_len();
    if let Some(bar) = self.bars.get_mut(index) {
      let new_y: Vec<f32> = y.iter().take(limit).cloned().collect();
      bar.y.clear();
      bar.y.extend(new_y);
    }
  }
  /// Sets data for the chart, automatically generating x-axis values if not set.
  ///
  /// If the x-axis is empty, it will be generated starting from `x_start` with the given `step`.
  /// Otherwise, only the y-axis data is set.
  ///
  /// # Arguments
  ///
  /// * `y` - A slice of f32 values representing the y-axis data.
  /// * `x_start` - The starting value for the x-axis if it needs to be generated.
  /// * `step` - The step/increment value for the x-axis if it needs to be generated.
  pub fn set_data_prototype(&mut self, y: &[f32], x_start: f32, step: f32) {
    let n = y.len();
    if n == 0 {
      return;
    }

    // if the x is not set, set it first
    if self.x.is_empty() {
      let mut new_x = Vec::with_capacity(n + 1);
      for i in 0..=n {
        new_x.push(x_start + i as f32 * step);
      }
      self.x = new_x;

      // get the max bins
      let limit = self.x.len().saturating_sub(1);

      let config = self.gen_config();
      // get the value based on the max_len(x)
      // to the same length as x bins
      let final_y = y.iter().take(limit).cloned().collect::<Vec<f32>>();
      self.bars.push(Bars::new(final_y, config));
    } else {
      // if have x, just set data
      self.set_data(y);
    }
  }
  /// Sets data for the chart with x-axis starting at 0 and the specified step.
  ///
  /// # Arguments
  ///
  /// * `y` - A slice of f32 values representing the y-axis data.
  /// * `step` - The step/increment value for the x-axis.
  pub fn set_data_with_step(&mut self, y: &[f32], step: f32) {
    self.set_data_prototype(y, 0., step);
  }
  /// Sets normalized data for the chart with x-axis starting at 0 and a step of 1.
  ///
  /// # Arguments
  ///
  /// * `y` - A slice of f32 values representing the y-axis data.
  pub fn set_data_norm(&mut self, y: &[f32]) {
    self.set_data_prototype(y, 0., 1.);
  }
}

// with errorbar
impl Histrogram {
  pub(crate) fn get_bars(&self) -> &[Bars] {
    &self.bars
  }
}
impl Bars {
  pub(crate) fn get_values(&self) -> &[f32] {
    self.y.as_ref()
  }
}

impl Drawable for Histrogram {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform) {
    // 只有当至少有两个刻度（一个槽位）且有数据组时才绘制
    if self.x.len() < 2 || self.bars.is_empty() {
      return;
    }

    let num_groups = self.bars.len() as f32;
    let group_width_ratio = 0.8;

    // constants
    let half_num_groups = (num_groups - 1.0) / 2.0;

    // 遍历槽位：i 是槽位索引，x_start/x_end is the start/end of the slot
    for i in 0..(self.x.len() - 1) {
      let x_start = self.x[i];
      let x_end = self.x[i + 1];
      let total_step_w = x_end - x_start;
      // shift half of total_step_w
      let x_start = x_start + total_step_w * 0.5;
      let x_center = x_start + total_step_w * 0.5;
      let single_bar_w = (total_step_w * group_width_ratio) / num_groups;

      for (g_idx, bar) in self.bars.iter().enumerate() {
        if bar.config.is_hidden {
          continue;
        }

        let y_val = bar.y[i];
        if y_val == 0.0 {
          continue;
        } // 0. not draw

        let offset = (g_idx as f32 - half_num_groups) * single_bar_w;
        let x_l = x_center + offset - single_bar_w * 0.5;
        let x_r = x_center + offset + single_bar_w * 0.5;

        // draw rect
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

          // border
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
    if self.x.is_empty() {
      return None;
    }

    let mut y_max = 0.0f32;
    let mut y_min = 0.0f32;
    let mut has_data = false;

    for bar in self.bars.iter() {
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
      x_min: *self.x.first().unwrap(),
      x_max: *self.x.last().unwrap(),
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
  fn set_color(&mut self, _color: [u8; 4]) {}
}
