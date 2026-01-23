use std::{
  cell::{Cell, RefCell},
  f32, u8,
};

use tiny_skia::{Paint, PathBuilder, Point, Stroke, Transform};

use crate::{
  color,
  drawable::{Bound, Drawable},
  primitive::Config,
};

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
}

pub struct ErrorBar {
  name: String,
  bars: RefCell<Vec<Bar>>,

  color_index: Cell<usize>,
}

impl ErrorBar {
  const OFFSET: usize = 1;

  pub fn new(name: String) -> Self {
    Self {
      name,
      bars: RefCell::new(Vec::new()),
      color_index: Cell::new(0),
    }
  }
  fn get_avarage(vals: &[f32]) -> f32 {
    if vals.is_empty() {
      return 0.0;
    }
    vals.iter().sum::<f32>() / vals.len() as f32
  }

  pub fn set_data(&self, vals: &[f32]) {
    let index = self.color_index.get();
    let color = color::get_color(index);
    self.color_index.set((index + 1) & 7);

    let config = Config {
      color,
      ..Config::default()
    }; // config set finished

    let mut min = f32::INFINITY;
    let mut max = f32::NEG_INFINITY;

    let mean = Self::get_avarage(vals);

    for &val in vals {
      min = min.min(val);
      max = max.max(val);
    }

    let bar = Bar::new(mean, min, max, config);

    self.bars.borrow_mut().push(bar);
  }
  pub fn set_data_prototype(&self, mean: f32, min: f32, max: f32) {
    let index = self.color_index.get();
    let color = color::get_color(index);
    self.color_index.set((index + 1) & 7);

    let config = Config {
      color,
      ..Config::default()
    };

    let bar = Bar::new(mean, min, max, config);
    self.bars.borrow_mut().push(bar);
  }
}

impl Drawable for ErrorBar {
  fn draw(&self, pixmap: &mut tiny_skia::Pixmap, ts: &tiny_skia::Transform) {
    let bars = self.bars.borrow();

    if bars.is_empty() {
      return;
    }

    let mut paint = Paint::default();
    for (y_index, bar) in bars.iter().enumerate() {
      if bar.config.is_hidden {
        continue;
      }
      let mut pb = PathBuilder::new();
      let mut x_start = Point::from_xy(bar.min, (y_index + Self::OFFSET) as f32);
      let mut x_end = Point::from_xy(bar.max, (y_index + Self::OFFSET) as f32);
      let mut mean = Point::from_xy(bar.mean, (y_index + Self::OFFSET) as f32);

      // finish point map
      ts.map_point(&mut x_start);
      ts.map_point(&mut x_end);
      ts.map_point(&mut mean);

      let [r, g, b, a] = bar.config.color;
      paint.set_color_rgba8(r, g, b, a);
      paint.anti_alias = true;
      if let Some(circle) = PathBuilder::from_circle(mean.x, mean.y, 6.) {
        pixmap.fill_path(
          &circle,
          &paint,
          tiny_skia::FillRule::Winding,
          Transform::identity(),
          None,
        );
      }
      // draw mean circle finished

      // start draw line
      pb.move_to(x_start.x, x_start.y);
      pb.line_to(x_end.x, x_end.y);

      // Draw vertical line at min
      let offset = 4.;
      pb.move_to(x_start.x, x_start.y - offset);
      pb.line_to(x_start.x, x_start.y + offset);

      // Draw vertical line at max
      pb.move_to(x_end.x, x_end.y - offset);
      pb.line_to(x_end.x, x_end.y + offset);
      let stroke = Stroke {
        width: 2.,
        line_cap: tiny_skia::LineCap::Round,
        line_join: tiny_skia::LineJoin::Round,
        ..Stroke::default()
      };

      if let Some(path) = pb.finish() {
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
      }
    }
  }
  fn bound(&self) -> Option<crate::drawable::Bound> {
    let bars = self.bars.borrow();
    if bars.is_empty() {
      return None;
    }

    let y_min: f32 = Self::OFFSET as f32;
    let y_max = (bars.len() + Self::OFFSET) as f32;

    let mut x_min = f32::INFINITY;
    let mut x_max = f32::NEG_INFINITY;

    for bar in bars.iter() {
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
  fn name(&self) -> String {
    self.name.clone()
  }
  // not used, manage color in self
  fn get_color(&self) -> [u8; 4] {
    [255, 255, 255, 255]
  }
  fn set_color(&self, _color: [u8; 4]) {}
}
