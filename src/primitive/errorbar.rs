use core::f32;

use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Stroke, Transform};

use crate::primitive::{Config, Histrogram};
use crate::{
  color,
  drawable::{Bound, Drawable},
};

pub enum ErrorBarType {
  BaseOnX,
  BaseOnY,
}

#[derive(Debug)]
struct Bar {
  mean: f32,
  min: f32,
  max: f32,

  config: Config,
}
impl Bar {
  pub fn new(mean: f32, min: f32, max: f32, config: Config) -> Self {
    Self {
      mean,
      min,
      max,
      config,
    }
  }
  fn or(mut self, rhs: f32) -> Self {
    let mean = (self.mean + rhs) / 2.;
    let min = self.min.min(rhs);
    let max = self.max.max(rhs);
    self.mean = mean;
    self.min = min;
    self.max = max;
    self
  }
}

pub struct ErrorBar {
  name: String,
  bars: Vec<Bar>,

  err_type: ErrorBarType,
  need_shift: bool,

  color_index: usize,
}

impl ErrorBar {
  const OFFSET: usize = 1;

  pub fn new(name: String) -> Self {
    Self {
      name,
      bars: Vec::new(),
      err_type: ErrorBarType::BaseOnX,
      color_index: 0,
      need_shift: true,
    }
  }
  fn get_avarage(vals: &[f32]) -> f32 {
    if vals.is_empty() {
      return 0.0;
    }
    vals.iter().sum::<f32>() / vals.len() as f32
  }
  fn gen_config(&mut self) -> Config {
    let index = self.color_index;
    let color = color::get_color(index);
    self.color_index = (index + 1) & 7;

    Config {
      color,
      ..Config::default()
    } // config set finished
  }

  /// Sets data by calculating statistics from the provided values and creating a new bar.
  ///
  /// # Arguments
  ///
  /// * `vals` - A slice of f32 values to calculate statistics from.
  ///
  /// This method calculates the mean, minimum, and maximum values from `vals`,
  /// creates a new `Bar` with these statistics and a color configuration,
  /// and adds it to the collection of bars.
  pub fn set_data(&mut self, vals: &[f32]) {
    let config = self.gen_config();
    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let mean = Self::get_avarage(vals);

    for &val in vals {
      min = min.min(val);
      max = max.max(val);
    }

    let bar = Bar::new(mean, min, max, config);

    self.bars.push(bar);
  }
  /// Creates a new bar with the provided statistics and adds it to the collection.
  ///
  /// # Arguments
  ///
  /// * `mean` - The mean (average) value for the bar.
  /// * `min` - The minimum value for the bar.
  /// * `max` - The maximum value for the bar.
  ///
  /// This is a prototype method that allows directly specifying statistics
  /// instead of calculating them from raw values.
  pub fn set_data_prototype(&mut self, mean: f32, min: f32, max: f32) {
    let config = self.gen_config();

    let bar = Bar::new(mean, min, max, config);
    self.bars.push(bar);
  }

  pub fn from_histogram(&mut self, his: &Histrogram) {
    let bars = his.get_bars(); // the bar of bars
    if bars.is_empty() {
      return;
    }

    let num_points = bars[0].get_values().len();
    if num_points == 0 {
      return;
    }
    self.need_shift = false;
    self.set_type(ErrorBarType::BaseOnY);
    let mut sums = vec![0.0f32; num_points];
    let mut mins = vec![f32::INFINITY; num_points];
    let mut maxs = vec![f32::NEG_INFINITY; num_points];

    for bar_group in bars {
      let values = bar_group.get_values();
      for (i, &val) in values.iter().enumerate().take(num_points) {
        sums[i] += val;
        if val < mins[i] {
          mins[i] = val;
        }
        if val > maxs[i] {
          maxs[i] = val;
        }
      }
    }

    let count = bars.len() as f32;
    let color_index = his.get_color_index();
    self.color_index = color_index + 1;
    let config = self.gen_config();
    let bar_vec: Vec<Bar> = (0..num_points)
      .map(|i| {
        Bar::new(
          sums[i] / count,
          mins[i],
          maxs[i], // 计算准确均值
          config,
        )
      })
      .collect();

    self.bars = bar_vec;
  }

  pub fn set_type(&mut self, tp: ErrorBarType) {
    self.err_type = tp;
  }
}

// implement for draw
impl ErrorBar {
  fn draw_internal(&self, pixmap: &mut Pixmap, ts: &Transform, mode: &ErrorBarType) {
    if self.bars.is_empty() {
      return;
    }

    let mut paint = Paint::default();
    let stroke = Stroke {
      width: 2.,
      ..Stroke::default()
    };

    for (index, bar) in self.bars.iter().enumerate() {
      if bar.config.is_hidden {
        continue;
      }

      let index_pos = index as f32
        + if self.need_shift {
          1.5 // prefer to OFFSET +0.5
        } else {
          Self::OFFSET as f32
        }; // add 0.5 to shift to the center

      let (mut start, mut end, mut mean) = match mode {
        ErrorBarType::BaseOnY => (
          Point::from_xy(index_pos, bar.min),
          Point::from_xy(index_pos, bar.max),
          Point::from_xy(index_pos, bar.mean),
        ),
        ErrorBarType::BaseOnX => (
          Point::from_xy(bar.min, index_pos),
          Point::from_xy(bar.max, index_pos),
          Point::from_xy(bar.mean, index_pos),
        ),
      };

      ts.map_point(&mut start);
      ts.map_point(&mut end);
      ts.map_point(&mut mean);

      let [r, g, b, a] = bar.config.color;
      paint.set_color_rgba8(r, g, b, a);
      paint.anti_alias = true;

      // 绘制均值圆
      if let Some(circle) = PathBuilder::from_circle(mean.x, mean.y, 6.) {
        pixmap.fill_path(
          &circle,
          &paint,
          tiny_skia::FillRule::Winding,
          Transform::identity(),
          None,
        );
      }

      // draw line
      let mut pb = PathBuilder::new();
      pb.move_to(start.x, start.y);
      pb.line_to(end.x, end.y);

      // draw capibility
      let offset = 4.;
      match mode {
        ErrorBarType::BaseOnY => {
          pb.move_to(start.x - offset, start.y);
          pb.line_to(start.x + offset, start.y);
          pb.move_to(end.x - offset, end.y);
          pb.line_to(end.x + offset, end.y);
        }
        ErrorBarType::BaseOnX => {
          pb.move_to(start.x, start.y - offset);
          pb.line_to(start.x, start.y + offset);
          pb.move_to(end.x, end.y - offset);
          pb.line_to(end.x, end.y + offset);
        }
      }

      if let Some(path) = pb.finish() {
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
      }
    }
  }
  fn bound_x(&self) -> Option<Bound> {
    if self.bars.is_empty() {
      return None;
    }

    let y_min: f32 = Self::OFFSET as f32;
    let y_max = (self.bars.len() + Self::OFFSET) as f32;

    let mut x_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;

    for bar in self.bars.iter() {
      x_min = x_min.min(bar.min);
      x_max = x_max.max(bar.max);
    }

    Some(Bound {
      x_min,
      x_max,
      y_min,
      y_max,
    })
  }
  fn bound_y(&self) -> Option<Bound> {
    if self.bars.is_empty() {
      return None;
    }

    let x_min: f32 = if self.need_shift { Self::OFFSET } else { 0 } as f32;
    let x_max = (self.bars.len() + if self.need_shift { Self::OFFSET } else { 0 }) as f32;

    let mut y_min = f32::INFINITY;
    let mut y_max = f32::NEG_INFINITY;

    for bar in self.bars.iter() {
      y_min = y_min.min(bar.min);
      y_max = y_max.max(bar.max);
    }

    Some(Bound {
      x_min,
      x_max,
      y_min,
      y_max,
    })
  }
}

impl Drawable for ErrorBar {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &tiny_skia::Transform) {
    self.draw_internal(pixmap, ts, &self.err_type);
  }
  fn bound(&self) -> Option<crate::drawable::Bound> {
    match self.err_type {
      ErrorBarType::BaseOnX => self.bound_x(),
      ErrorBarType::BaseOnY => self.bound_y(),
    }
  }
  fn name(&self) -> String {
    self.name.clone()
  }
  // not used, manage color in self
  fn get_color(&self) -> [u8; 4] {
    [255, 255, 255, 255]
  }
  fn set_color(&mut self, _color: [u8; 4]) {}
}
