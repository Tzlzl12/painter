use std::rc::Rc;

use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Rect, Stroke, Transform};

use crate::{color, drawable::Drawable};

pub struct Axis {
  pub x: f32,
  pub y: f32,
  pub veiwport: Rect,
  color_index: usize,
  drawables: Vec<Rc<dyn Drawable>>,

  config: Config,
}

impl Axis {
  pub fn new(x: f32, y: f32, size: (f32, f32)) -> Self {
    Self {
      x,
      y,
      veiwport: Rect::from_xywh(0., 0., size.0, size.1).unwrap(),
      drawables: Vec::new(),
      color_index: 0,
      config: Config::default(),
    }
  }
  pub fn set_x_limit(&mut self, limit: Option<(f32, f32)>) {
    self.config.x_limit = limit;
  }
  pub fn set_y_limit(&mut self, limit: Option<(f32, f32)>) {
    self.config.y_limit = limit;
  }
  pub fn set_strategy(&mut self, strategy: ScaleStrategy) {
    self.config.strategy = strategy;
  }
  pub fn change_veiwport(&mut self, axis: (f32, f32), size: (f32, f32)) {
    self.x = axis.0;
    self.y = axis.1;
    self.veiwport = Rect::from_xywh(0., 0., size.0, size.1).unwrap();
  }
  fn render_axis(&self, pixmap: &mut Pixmap, ts: &Transform) {
    let width = self.veiwport.width();
    let height = self.veiwport.height();
    let margin: f32 = (width * 0.1).min(50.);

    let (x_min, x_max) = self.config.x_limit.unwrap_or((0.0, 1.0));
    let (y_min, y_max) = self.config.y_limit.unwrap_or((0.0, 1.0));

    // --- 1. 坐标映射 ---
    let mut origin = Point::from_xy(0.0, 0.0);
    let mut x_start = Point::from_xy(x_min, 0.0);
    let mut x_end = Point::from_xy(x_max, 0.0);
    let mut y_start = Point::from_xy(0.0, y_min);
    let mut y_end = Point::from_xy(0.0, y_max);

    ts.map_point(&mut origin);
    ts.map_point(&mut x_start);
    ts.map_point(&mut x_end);
    ts.map_point(&mut y_start);
    ts.map_point(&mut y_end);

    let mut pb = PathBuilder::new();
    let arrow_len = 10.0;
    let tick_size = 5.0;

    // --- 2. 画 X 轴 ---
    pb.move_to(x_start.x, origin.y);
    pb.line_to(x_end.x, origin.y);
    // X 轴箭头
    pb.move_to(x_end.x, origin.y);
    pb.line_to(x_end.x - arrow_len, origin.y - arrow_len * 0.5);
    pb.move_to(x_end.x, origin.y);
    pb.line_to(x_end.x - arrow_len, origin.y + arrow_len * 0.5);

    // --- 3. 画 Y 轴 ---
    pb.move_to(origin.x, y_start.y);
    pb.line_to(origin.x, y_end.y);
    // Y 轴箭头
    pb.move_to(origin.x, y_end.y);
    pb.line_to(origin.x - arrow_len * 0.5, y_end.y + arrow_len);
    pb.move_to(origin.x, y_end.y);
    pb.line_to(origin.x + arrow_len * 0.5, y_end.y + arrow_len);

    // --- 4. 画 X 轴刻度 ---
    let num_ticks = 10;
    for i in 0..=num_ticks {
      let t = i as f32 / num_ticks as f32;
      let x_val = x_min + t * (x_max - x_min);
      let mut p_tick = Point::from_xy(x_val, 0.0);
      ts.map_point(&mut p_tick);

      pb.move_to(p_tick.x, origin.y);
      pb.line_to(p_tick.x, origin.y - tick_size);
    }

    // --- 5. 画 Y 轴刻度 ---
    for i in 0..=num_ticks {
      let t = i as f32 / num_ticks as f32;
      let y_val = y_min + t * (y_max - y_min);

      // 映射每一个 Y 轴刻度点的像素位置 (x 固定在 0.0)
      let mut p_tick = Point::from_xy(0.0, y_val);
      ts.map_point(&mut p_tick);

      // 刻度线从 Y 轴向右突出
      pb.move_to(origin.x, p_tick.y);
      pb.line_to(origin.x + tick_size, p_tick.y);
    }

    // --- 6. 渲染 ---
    let mut paint = Paint::default();
    paint.set_color_rgba8(200, 200, 200, 255);
    paint.anti_alias = true;

    let stroke = Stroke {
      width: 1.5,
      ..tiny_skia::Stroke::default()
    };

    let base_ts = Transform::from_translate(self.x, self.y);

    if let Some(path) = pb.finish() {
      pixmap.stroke_path(&path, &paint, &stroke, base_ts, None);
    }
  }
  fn auto_limit(&mut self) {
    if !self.config.x_limit.is_none() {
      return;
    }
    let mut total_bounds = None;

    for drawable in &self.drawables {
      if let Some(b) = drawable.bound() {
        match total_bounds {
          None => total_bounds = Some(b),
          Some(ref mut tb) => {
            tb.x_min = tb.x_min.min(b.x_min);
            tb.x_max = tb.x_max.max(b.x_max);
            tb.y_min = tb.y_min.min(b.y_min);
            tb.y_max = tb.y_max.max(b.y_max);
          }
        }
      }
    }

    if let Some(b) = total_bounds {
      // 给边界留一点点余量（Margin），防止曲线贴着坐标轴边缘
      let padding = 0.05;
      let dx = if b.x_max == b.x_min {
        1.0
      } else {
        (b.x_max - b.x_min) * padding
      };
      let dy = if b.y_max == b.y_min {
        1.0
      } else {
        (b.y_max - b.y_min) * padding
      };

      self.set_x_limit(Some((b.x_min - dx, b.x_max + dx)));
      self.set_y_limit(Some((b.y_min - dy, b.y_max + dy)));
    }
  }
  pub fn render(&mut self, pixmap: &mut Pixmap) {
    self.auto_limit();

    let width = self.veiwport.width();
    let height = self.veiwport.height();
    let margin: f32 = (width * 0.1).min(50.);

    let (x_min, x_max) = self.config.x_limit.unwrap_or((0.0, 1.0));
    let (y_min, y_max) = self.config.y_limit.unwrap_or((0.0, 1.0));

    let x_range = x_max - x_min;
    let y_range = y_max - y_min;

    if x_range == 0.0 || y_range == 0.0 {
      return;
    }

    let plot_w = width - 2. * margin;
    let plot_h = height - 2. * margin;

    // --- 根据策略计算缩放 ---
    let (scale_x, scale_y, offset_x, offset_y) = match self.config.strategy {
      ScaleStrategy::Stretch => {
        // 分别拉伸，偏移为 0
        (plot_w / x_range, plot_h / y_range, 0.0, 0.0)
      }
      ScaleStrategy::Fit => {
        // 保持比例，取最小值并计算居中偏移
        let s = (plot_w / x_range).min(plot_h / y_range);
        let ox = (plot_w - (x_range * s)) / 2.0;
        let oy = (plot_h - (y_range * s)) / 2.0;
        (s, s, ox, oy)
      }
    };

    // 应用变换
    let inner_ts = Transform::from_translate(margin + offset_x, height - (margin + offset_y))
      .pre_scale(scale_x, -scale_y)
      .pre_translate(-x_min, -y_min);

    let base_ts = Transform::from_translate(self.x, self.y);
    let ts = base_ts.pre_concat(inner_ts);

    self.render_axis(pixmap, &ts);
    // ... 剩下的遍历绘制逻辑 ...
    for drawable in &self.drawables {
      // 颜色分配逻辑保持不变
      if drawable.get_color() == [0, 0, 0, 0] {
        let color = color::get_color(self.color_index & 7);
        self.color_index += 1;
        drawable.set_color(color);
      }
      drawable.draw(pixmap, &ts);
    }
  }
  pub fn add(&mut self, drawable: Rc<dyn Drawable>) {
    self.drawables.push(drawable);
  }
}

#[derive(Default)]
pub struct Config {
  x_limit: Option<(f32, f32)>,
  y_limit: Option<(f32, f32)>,
  strategy: ScaleStrategy,
}
pub enum ScaleStrategy {
  Fit,
  Stretch,
}
impl Default for ScaleStrategy {
  fn default() -> Self {
    ScaleStrategy::Fit
  }
}
