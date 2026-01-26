use tiny_skia::{Color, Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

use crate::{color, drawable::Drawable, text_render::TextRender};

pub struct Axis {
  pub x: f32,
  pub y: f32,
  pub viewport: Rect,
  color_index: usize,
  drawables: Vec<Box<dyn Drawable>>,

  config: Config,
}

impl Axis {
  pub(crate) fn new(x: f32, y: f32, size: (f32, f32)) -> Self {
    Self {
      x,
      y,
      viewport: Rect::from_xywh(0., 0., size.0, size.1).unwrap(),
      drawables: Vec::new(),
      color_index: 0,
      config: Config::default(),
    }
  }
  pub(crate) fn change_veiwport(&mut self, axis: (f32, f32), size: (f32, f32)) {
    self.x = axis.0;
    self.y = axis.1;
    self.viewport = Rect::from_xywh(0., 0., size.0, size.1).unwrap();
  }

  /// design to render axis with grid lines
  fn render_axis(&self, pixmap: &mut Pixmap, ui_ts: &Transform, tr: &TextRender) {
    let width = self.viewport.width();
    let height = self.viewport.height();
    let margin = (width * 0.1).min(50.0);

    let (x_min, x_max) = self.config.x_limit.unwrap_or((-1.0, 1.0));
    let (y_min, y_max) = self.config.y_limit.unwrap_or((-1.0, 1.0));

    let x_range = (x_max - x_min).max(1e-6);
    let y_range = (y_max - y_min).max(1e-6);

    let plot_w = width - 2.0 * margin;
    let plot_h = height - 2.0 * margin;

    let (scale_x, scale_y) = match self.config.strategy {
      ScaleStrategy::Stretch => (plot_w / x_range, plot_h / y_range),
      ScaleStrategy::Fit => {
        let s = (plot_w / x_range).min(plot_h / y_range);
        (s, s)
      }
    };

    let actual_w = x_range * scale_x;
    let actual_h = y_range * scale_y;

    // 计算 0 点在画布上的像素位置
    let origin_x = if x_min >= 0.0 {
      0.0
    } else if x_max <= 0.0 {
      actual_w
    } else {
      (-x_min / x_range) * actual_w
    };
    let origin_y = if y_min >= 0.0 {
      0.0
    } else if y_max <= 0.0 {
      -actual_h
    } else {
      (-y_min / y_range) * -actual_h
    };

    let (x_interval, xn) = Self::calculate_tick_interval(x_range);
    let (y_interval, yn) = Self::calculate_tick_interval(y_range);

    self.draw_grid(pixmap, ui_ts, x_interval, y_interval, actual_w, actual_h);

    self.draw_ticks(
      pixmap,
      ui_ts,
      tr,
      (x_interval, xn),
      (y_interval, yn),
      actual_w,
      actual_h,
    );

    if x_min <= 0.0 && x_max >= 0.0 {
      self.draw_axis_y(pixmap, ui_ts, actual_h, origin_x);
    }
    if y_min <= 0.0 && y_max >= 0.0 {
      self.draw_axis_x(pixmap, ui_ts, actual_w, origin_y);
    }
    println!("{} {}", x_interval, y_interval);
  }
  fn draw_grid(&self, pixmap: &mut Pixmap, ts: &Transform, x_int: f32, y_int: f32, w: f32, h: f32) {
    let mut pb = PathBuilder::new();
    let (x_min, x_max) = self.config.x_limit.unwrap_or((0.0, 10.0));
    let (y_min, y_max) = self.config.y_limit.unwrap_or((0.0, 10.0));

    let x_range = (x_max - x_min).max(1e-6);
    let y_range = (y_max - y_min).max(1e-6);

    // 1. 垂直线 (X 轴网格)
    // 逻辑：计算从哪个整数倍 interval 开始画，直到 x_max 前停下
    let mut x_val = (x_min / x_int).floor() * x_int;
    while x_val < x_max - (x_int * 0.01) {
      // 减去微小偏移量，防止在 x_max 处多画一根封口线
      if x_val >= x_min {
        // 关键：px 必须基于 x_range 比例计算，确保与点的位置逻辑对齐
        let px = (x_val - x_min) / x_range * w;
        pb.move_to(px, 0.0);
        pb.line_to(px, -h);
      }
      x_val += x_int;
    }

    // 2. 水平线 (Y 轴网格)
    let mut y_val = (y_min / y_int).floor() * y_int;
    while y_val < y_max - (y_int * 0.01) {
      if y_val >= y_min {
        let py = -((y_val - y_min) / y_range * h);
        pb.move_to(0.0, py);
        pb.line_to(w, py);
      }
      y_val += y_int;
    }

    self.stroke_path(pixmap, pb, ts, 1., color::get_gray());
  }
  fn draw_ticks(
    &self, pixmap: &mut Pixmap, ui_ts: &Transform, tr: &TextRender, x_info: (f32, usize),
    y_info: (f32, usize), w: f32, h: f32,
  ) {
    let (x_min, x_max) = self.config.x_limit.unwrap_or((0.0, 1.0));
    let (y_min, y_max) = self.config.y_limit.unwrap_or((0.0, 1.0));
    let x_range = (x_max - x_min).max(1e-6);
    let y_range = (y_max - y_min).max(1e-6);

    let (x_int, x_count) = x_info;
    let (y_int, y_count) = y_info;

    let font_size = 12.0; // 这个现在正确传给 size 参数
    let [r, g, b, a] = color::get_fg();
    let text_color = Color::from_rgba8(r, g, b, a);

    // 1. 绘制 X 轴刻度 (标签在轴下方)
    let x_start_val = (x_min / x_int).floor() * x_int;
    for i in 0..x_count {
      let x_val = x_start_val + (i as f32 * x_int);
      if x_val >= x_min && x_val <= x_max + 1e-6 {
        let px = (x_val - x_min) / x_range * w;
        let label = format!("{:.1}", x_val);

        // 修正居中：根据字符数量估算宽度，font_size * 0.5 是平均字符宽度
        let text_w = label.len() as f32 * (font_size * 0.5);

        tr.draw(
          pixmap,
          &label,
          ui_ts.tx + px - (text_w / 2.0), // X: 轴原点 + 偏移 - 半宽
          ui_ts.ty + 10.0,                // Y: 轴原点下方 10 像素
          font_size,                      // Size
          text_color,
        );
      }
    }

    // 2. 绘制 Y 轴刻度 (标签在轴左侧)
    let y_start_val = (y_min / y_int).floor() * y_int;
    for i in 0..y_count {
      let y_val = y_start_val + (i as f32 * y_int);
      if y_val >= y_min && y_val <= y_max + 1e-6 {
        let py = -((y_val - y_min) / y_range * h); // 笛卡尔转屏幕坐标
        let label = format!("{:.1}", y_val);

        let text_w = label.len() as f32 * (font_size * 0.5);

        tr.draw(
          pixmap,
          &label,
          ui_ts.tx - text_w - 8.0,           // X: 轴原点左边，留 8px 间距
          ui_ts.ty + py - (font_size / 2.0), // Y: 居中对齐刻度线
          font_size,                         // Size
          text_color,
        );
      }
    }
  }
  /// 绘制 X 轴
  fn draw_axis_x(&self, pixmap: &mut Pixmap, ts: &Transform, w: f32, origin_y: f32) {
    let mut pb = PathBuilder::new();
    let arrow_len = 10.0;

    pb.move_to(0.0, origin_y);
    pb.line_to(w, origin_y);

    // 箭头
    pb.move_to(w, origin_y);
    pb.line_to(w - arrow_len, origin_y - arrow_len * 0.5);
    pb.move_to(w, origin_y);
    pb.line_to(w - arrow_len, origin_y + arrow_len * 0.5);

    self.stroke_path(pixmap, pb, ts, 1.5, color::get_fg());
  }

  /// 绘制 Y 轴
  fn draw_axis_y(&self, pixmap: &mut Pixmap, ts: &Transform, h: f32, origin_x: f32) {
    let mut pb = PathBuilder::new();
    let arrow_len = 10.0;

    pb.move_to(origin_x, 0.0);
    pb.line_to(origin_x, -h);

    // 箭头
    pb.move_to(origin_x, -h);
    pb.line_to(origin_x - arrow_len * 0.5, -h + arrow_len);
    pb.move_to(origin_x, -h);
    pb.line_to(origin_x + arrow_len * 0.5, -h + arrow_len);

    self.stroke_path(pixmap, pb, ts, 1.5, color::get_fg());
  }

  /// 公用渲染辅助
  fn stroke_path(
    &self, pixmap: &mut Pixmap, pb: PathBuilder, ts: &Transform, width: f32, color: [u8; 4],
  ) {
    if let Some(path) = pb.finish() {
      let mut paint = Paint::default();
      paint.set_color_rgba8(color[0], color[1], color[2], color[3]);
      paint.anti_alias = true;
      pixmap.stroke_path(
        &path,
        &paint,
        &Stroke {
          width,
          ..Default::default()
        },
        *ts,
        None,
      );
    }
  }
  /// calculate a "nice" tick interval and number of ticks for a given range
  /// ## parameter
  /// * range: f32 - the data range (max - min)
  fn calculate_tick_interval(range: f32) -> (f32, usize) {
    if range <= 0.0 {
      return (1.0, 1);
    }

    // 目标: 大约 5-10 个刻度
    let raw_interval = range / 8.0;

    // 计算数量级
    let magnitude = 10_f32.powf(raw_interval.log10().floor());

    // 标准化到 1, 2, 5 系列
    let normalized = raw_interval / magnitude;
    let nice_interval = if normalized < 1.5 {
      1.0 * magnitude
    } else if normalized < 3.0 {
      2.0 * magnitude
    } else if normalized < 7.0 {
      5.0 * magnitude
    } else {
      10.0 * magnitude
    };

    let num_ticks = (range / nice_interval).ceil() as usize + 1;
    (nice_interval, num_ticks)
  }
  fn auto_limit(&mut self) {
    if self.config.x_limit.is_some() && self.config.y_limit.is_some() {
      return;
    }
    println!("into autolimit");
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
      let padding = 0.1; // 只给最大值留 10% 的呼吸空间

      let dx = if b.x_max <= b.x_min {
        1.0
      } else {
        (b.x_max - b.x_min) * padding
      };
      let dy = if b.y_max <= b.y_min {
        1.0
      } else {
        (b.y_max - b.y_min) * padding
      };

      if self.config.x_limit.is_none() {
        self.set_x_limit(Some((b.x_min, b.x_max + dx)));
      }
      if self.config.y_limit.is_none() {
        self.set_y_limit(Some((b.y_min, b.y_max + dy)));
      }
    }
  }
  pub(crate) fn render(&mut self, pixmap: &mut Pixmap, tr: &TextRender) {
    self.auto_limit();

    let width = self.viewport.width();
    let height = self.viewport.height();
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
        // 分别拉伸，填满整个绘图区域
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

    let base_ts = Transform::from_translate(self.x, self.y);

    // UI 变换（用于坐标轴）：仅平移到绘图区域左下角
    let ui_ts = base_ts.pre_translate(margin + offset_x, height - (margin + offset_y));

    // 数据变换（用于数据绘制）：平移+缩放+翻转
    let data_ts = ui_ts
      .pre_scale(scale_x, -scale_y)
      .pre_translate(-x_min, -y_min);

    // 绘制坐标轴：使用 UI 变换
    self.render_axis(pixmap, &ui_ts, &tr);

    // 绘制数据：使用数据变换
    for drawable in &mut self.drawables {
      if drawable.get_color() == [0, 0, 0, 0] {
        let color = color::get_color(self.color_index & 7);
        self.color_index += 1;
        drawable.set_color(color);
      }
      drawable.draw(pixmap, &data_ts);
    }
  }
  pub fn add(&mut self, drawable: Box<dyn Drawable>) {
    self.drawables.push(drawable);
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
