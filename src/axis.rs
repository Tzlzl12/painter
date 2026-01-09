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
  // fn render_axis(&self, pixmap: &mut Pixmap, ui_ts: &Transform) {
  //   let width = self.viewport.width();
  //   let height = self.viewport.height();
  //   let margin = (width * 0.1).min(50.0);

  //   // 读取已经设置好的范围（可能是手动设置，也可能是 auto_limit 自动算的）
  //   let (x_min, x_max) = self.config.x_limit.unwrap_or((-1.0, 1.0));
  //   let (y_min, y_max) = self.config.y_limit.unwrap_or((-1.0, 1.0));

  //   let x_range = (x_max - x_min).max(1e-6);
  //   let y_range = (y_max - y_min).max(1e-6);

  //   // 计算绘图区域实际像素尺寸
  //   let plot_w = width - 2.0 * margin;
  //   let plot_h = height - 2.0 * margin;

  //   let (scale_x, scale_y) = match self.config.strategy {
  //     ScaleStrategy::Stretch => (plot_w / x_range, plot_h / y_range),
  //     ScaleStrategy::Fit => {
  //       let s = (plot_w / x_range).min(plot_h / y_range);
  //       (s, s)
  //     }
  //   };

  //   let actual_w = x_range * scale_x;
  //   let actual_h = y_range * scale_y;

  //   // 计算数学上的 0 在当前坐标系中的归一化位置 (0.0 ~ 1.0)
  //   // 这决定了 0 点应该画在轴的哪个位置
  //   let zero_x_norm = if x_min >= 0.0 {
  //     0.0 // 全正 → 0 在最左
  //   } else if x_max <= 0.0 {
  //     1.0 // 全负 → 0 在最右
  //   } else {
  //     (0.0 - x_min) / x_range // 跨零 → 0 在中间合适位置
  //   };

  //   let zero_y_norm = if y_min >= 0.0 {
  //     0.0
  //   } else if y_max <= 0.0 {
  //     1.0
  //   } else {
  //     (0.0 - y_min) / y_range
  //   };

  //   // 转换为像素坐标（相对于绘图区域左下角）
  //   let origin_x = zero_x_norm * actual_w;
  //   let origin_y = -zero_y_norm * actual_h; // tiny-skia y 向下为正，所以负号

  //   // 开始构建轴路径
  //   let mut pb = PathBuilder::new();
  //   let arrow_len = 10.0;
  //   let tick_size = 5.0;

  //   // X 轴 ────────────────────────────────────────────────
  //   pb.move_to(0.0, origin_y);
  //   pb.line_to(actual_w, origin_y);

  //   // X 轴箭头（始终指向正方向）
  //   pb.move_to(actual_w, origin_y);
  //   pb.line_to(actual_w - arrow_len, origin_y - arrow_len * 0.5);
  //   pb.move_to(actual_w, origin_y);
  //   pb.line_to(actual_w - arrow_len, origin_y + arrow_len * 0.5);

  //   // Y 轴 ────────────────────────────────────────────────
  //   pb.move_to(origin_x, 0.0);
  //   pb.line_to(origin_x, -actual_h);

  //   // Y 轴箭头（始终指向上方 = 正方向）
  //   pb.move_to(origin_x, -actual_h);
  //   pb.line_to(origin_x - arrow_len * 0.5, -actual_h + arrow_len);
  //   pb.move_to(origin_x, -actual_h);
  //   pb.line_to(origin_x + arrow_len * 0.5, -actual_h + arrow_len);

  //   // X 刻度 ──────────────────────────────────────────────
  //   let (x_interval, _) = Self::calculate_tick_interval(x_range);
  //   let mut x_val = (x_min / x_interval).floor() * x_interval; // 从 floor 开始，确保负值也能画
  //   while x_val <= x_max + x_interval * 0.001 {
  //     let t = (x_val - x_min) / x_range;
  //     let px = t * actual_w;
  //     // 只在轴可见范围内画刻度
  //     if (0.0..=actual_w).contains(&px) {
  //       pb.move_to(px, origin_y - tick_size);
  //       pb.line_to(px, origin_y + tick_size);
  //     }
  //     x_val += x_interval;
  //   }

  //   // Y 刻度 ──────────────────────────────────────────────
  //   let (y_interval, _) = Self::calculate_tick_interval(y_range);
  //   let mut y_val = (y_min / y_interval).floor() * y_interval;
  //   while y_val <= y_max + y_interval * 0.001 {
  //     let t = (y_val - y_min) / y_range;
  //     let py = -t * actual_h;
  //     if (-actual_h..=0.0).contains(&py) {
  //       pb.move_to(origin_x - tick_size, py);
  //       pb.line_to(origin_x + tick_size, py);
  //     }
  //     y_val += y_interval;
  //   }

  //   // 渲染
  //   let mut paint = Paint::default();
  //   paint.set_color_rgba8(200, 200, 200, 255);
  //   paint.anti_alias = true;

  //   if let Some(path) = pb.finish() {
  //     pixmap.stroke_path(
  //       &path,
  //       &paint,
  //       &Stroke {
  //         width: 1.5,
  //         ..Default::default()
  //       },
  //       *ui_ts,
  //       None,
  //     );
  //   }
  // }
  // fn render_axis(&self, pixmap: &mut Pixmap, ui_ts: &Transform) {
  //   // --- 1. 基础布局与比例计算 ---
  //   let width = self.viewport.width();
  //   let height = self.viewport.height();
  //   let margin = (width * 0.1).min(50.0);

  //   let (x_min, x_max) = self.config.x_limit.unwrap_or((-1.0, 1.0));
  //   let (y_min, y_max) = self.config.y_limit.unwrap_or((-1.0, 1.0));

  //   let x_range = (x_max - x_min).max(1e-6);
  //   let y_range = (y_max - y_min).max(1e-6);

  //   let plot_w = width - 2.0 * margin;
  //   let plot_h = height - 2.0 * margin;

  //   let (scale_x, scale_y) = match self.config.strategy {
  //     ScaleStrategy::Stretch => (plot_w / x_range, plot_h / y_range),
  //     ScaleStrategy::Fit => {
  //       let s = (plot_w / x_range).min(plot_h / y_range);
  //       (s, s)
  //     }
  //   };

  //   let actual_w = x_range * scale_x;
  //   let actual_h = y_range * scale_y;

  //   // 计算 0 点在画布上的像素位置
  //   let origin_x = if x_min >= 0.0 {
  //     0.0
  //   } else if x_max <= 0.0 {
  //     actual_w
  //   } else {
  //     (-x_min / x_range) * actual_w
  //   };
  //   let origin_y = if y_min >= 0.0 {
  //     0.0
  //   } else if y_max <= 0.0 {
  //     -actual_h
  //   } else {
  //     (-y_min / y_range) * -actual_h
  //   };

  //   let (x_interval, _) = Self::calculate_tick_interval(x_range);
  //   let (y_interval, _) = Self::calculate_tick_interval(y_range);

  //   // --- 2. 分离绘制 ---

  //   // A. 绘制网格线 (Grid)
  //   self.draw_grid(
  //     pixmap, ui_ts, x_min, x_max, y_min, y_max, x_interval, y_interval, actual_w, actual_h,
  //   );

  //   if !self.config.x_limit.is_none() {
  //     println!(
  //       "{} {}",
  //       self.config.x_limit.unwrap().0,
  //       self.config.x_limit.unwrap().1
  //     );
  //     if self.config.x_limit.unwrap().0 <= 0.0 && self.config.x_limit.unwrap().1 >= 0.0 {
  //       self.draw_axis_x(pixmap, ui_ts, actual_w, origin_y);
  //     }
  //   }
  //   if !self.config.y_limit.is_none() {
  //     println!(
  //       "{} {}",
  //       self.config.y_limit.unwrap().0,
  //       self.config.y_limit.unwrap().1
  //     );
  //     if self.config.y_limit.unwrap().0 <= 0.0 && self.config.y_limit.unwrap().1 >= 0.0 {
  //       self.draw_axis_y(pixmap, ui_ts, actual_h, origin_x);
  //     }
  //   }
  // }

  /// 专门负责绘制网格线
  fn render_axis(&self, pixmap: &mut Pixmap, ui_ts: &Transform) {
    // ... 前面计算 scale_x, scale_y, actual_w, actual_h, origin_x, origin_y 的逻辑保持不变 ...

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

    let (x_interval, _) = Self::calculate_tick_interval(x_range);
    let (y_interval, _) = Self::calculate_tick_interval(y_range);

    // 1. 绘制网格：只传必要的终点和间隔，其他能算出来的都在里面算
    self.draw_grid(pixmap, ui_ts, x_interval, y_interval, actual_w, actual_h);

    self.draw_axis_x(pixmap, ui_ts, actual_w, origin_y);
    self.draw_axis_y(pixmap, ui_ts, actual_h, origin_x);
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

    self.stroke_path(pixmap, pb, ts, 1.5, [200, 200, 200, 255]);
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

    self.stroke_path(pixmap, pb, ts, 1.5, [200, 200, 200, 255]);
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
    if self.config.x_limit.is_some() || self.config.y_limit.is_some() {
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
  pub fn render(&mut self, pixmap: &mut Pixmap) {
    // println!(
    //   "render start | x_limit: {:?} | y_limit: {:?}",
    //   self.config.x_limit, self.config.y_limit
    // );
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
