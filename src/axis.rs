use std::rc::Rc;

use tiny_skia::{Paint, PathBuilder, Pixmap, Point, Rect, Stroke, Transform};

use crate::{color, drawable::Drawable};

pub struct Axis {
  pub x: f32,
  pub y: f32,
  pub viewport: Rect,
  color_index: usize,
  drawables: Vec<Rc<dyn Drawable>>,

  config: Config,
}

impl Axis {
  pub fn new(x: f32, y: f32, size: (f32, f32)) -> Self {
    Self {
      x,
      y,
      viewport: Rect::from_xywh(0., 0., size.0, size.1).unwrap(),
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
    self.viewport = Rect::from_xywh(0., 0., size.0, size.1).unwrap();
  }
  fn render_axis(&self, pixmap: &mut Pixmap, ui_ts: &Transform) {
    // 1. 获取基础物理信息
    let width = self.viewport.width();
    let height = self.viewport.height();
    let margin: f32 = (width * 0.1).min(50.);

    // 2. 获取逻辑范围
    let (x_min, x_max) = self.config.x_limit.unwrap_or((0.0, 1.0));
    let (y_min, y_max) = self.config.y_limit.unwrap_or((0.0, 1.0));
    let x_range = (x_max - x_min).max(1e-6);
    let y_range = (y_max - y_min).max(1e-6);

    // 3. 重新计算 Scale（这部分逻辑必须与 render 函数中一致）
    let plot_w = width - 2. * margin;
    let plot_h = height - 2. * margin;

    let (scale_x, scale_y) = match self.config.strategy {
      ScaleStrategy::Stretch => (plot_w / x_range, plot_h / y_range),
      ScaleStrategy::Fit => {
        let s = (plot_w / x_range).min(plot_h / y_range);
        (s, s)
      }
    };

    // 4. 计算实际物理轴长
    let actual_w = x_range * scale_x;
    let actual_h = y_range * scale_y;

    // --- 开始绘制 ---
    let mut pb = PathBuilder::new();
    let arrow_len = 10.0;
    let tick_size = 5.0;

    // X 轴：从 0 到 actual_w
    pb.move_to(0.0, 0.0);
    pb.line_to(actual_w, 0.0);
    // X 箭头
    pb.move_to(actual_w, 0.0);
    pb.line_to(actual_w - arrow_len, -arrow_len * 0.5);
    pb.move_to(actual_w, 0.0);
    pb.line_to(actual_w - arrow_len, arrow_len * 0.5);

    // Y 轴：从 0 到 -actual_h
    pb.move_to(0.0, 0.0);
    pb.line_to(0.0, -actual_h);
    // Y 箭头
    pb.move_to(0.0, -actual_h);
    pb.line_to(-arrow_len * 0.5, -actual_h + arrow_len);
    pb.move_to(0.0, -actual_h);
    pb.line_to(arrow_len * 0.5, -actual_h + arrow_len);

    // --- 5. 画刻度 ---
    let (x_interval, _) = Self::calculate_tick_interval(x_range);
    let mut x_val = (x_min / x_interval).ceil() * x_interval;
    while x_val <= x_max + x_interval * 0.001 {
      let t = (x_val - x_min) / x_range;
      let px = t * actual_w; // 基于实际缩放后的长度
      pb.move_to(px, -tick_size);
      pb.line_to(px, tick_size);
      x_val += x_interval;
    }

    let (y_interval, _) = Self::calculate_tick_interval(y_range);
    let mut y_val = (y_min / y_interval).ceil() * y_interval;
    while y_val <= y_max + y_interval * 0.001 {
      let t = (y_val - y_min) / y_range;
      let py = -t * actual_h; // 基于实际缩放后的高度
      pb.move_to(-tick_size, py);
      pb.line_to(tick_size, py);
      y_val += y_interval;
    }

    // --- 6. 渲染 ---
    let mut paint = Paint::default();
    paint.set_color_rgba8(200, 200, 200, 255);
    paint.anti_alias = true;

    if let Some(path) = pb.finish() {
      // 使用传入的 ui_ts 确定原点位置，但内部 Path 是 1:1 的像素
      pixmap.stroke_path(
        &path,
        &paint,
        &Stroke {
          width: 1.5,
          ..Default::default()
        },
        *ui_ts,
        None,
      );
    }
  }

  // 辅助函数：计算合适的刻度间隔
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

      // --- 核心修改：最小值不减 dx/dy，直接用原始最小值 ---
      // 这样逻辑原点 (b.x_min, b.y_min) 就会精准对齐绘图区的边缘
      self.set_x_limit(Some((b.x_min, b.x_max + dx)));
      self.set_y_limit(Some((b.y_min, b.y_max + dy)));
    }
  }
  pub fn render(&mut self, pixmap: &mut Pixmap) {
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
    self.render_axis(pixmap, &ui_ts);

    // 绘制数据：使用数据变换
    for drawable in &self.drawables {
      if drawable.get_color() == [0, 0, 0, 0] {
        let color = color::get_color(self.color_index & 7);
        self.color_index += 1;
        drawable.set_color(color);
      }
      drawable.draw(pixmap, &data_ts);
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
