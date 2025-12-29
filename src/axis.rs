use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

use crate::drawable::Drawable;

pub struct Axis {
  pub x: f32,
  pub y: f32,
  pub veiwport: Rect,
  drawables: Vec<Box<dyn Drawable>>,

  config: Config,
}

impl Axis {
  pub fn new(x: f32, y: f32, size: (f32, f32)) -> Self {
    Self {
      x,
      y,
      veiwport: Rect::from_xywh(0., 0., size.0, size.1).unwrap(),
      drawables: Vec::new(),
      config: Config::default(),
    }
  }
  pub fn set_x_limit(&mut self, limit: Option<(f32, f32)>) {
    self.config.x_limit = limit;
  }
  pub fn set_y_limit(&mut self, limit: Option<(f32, f32)>) {
    self.config.y_limit = limit;
  }
  pub fn change_veiwport(&mut self, axis: (f32, f32), size: (f32, f32)) {
    self.x = axis.0;
    self.y = axis.1;
    self.veiwport = Rect::from_xywh(0., 0., size.0, size.1).unwrap();
  }
  fn render_axis(&self, pixmap: &mut Pixmap) {
    let width = self.veiwport.width();
    let height = self.veiwport.height();
    let margin: f32 = (width * 0.1).min(50.);
    let plot_w = width - 2.0 * margin;
    let plot_h = height - 2.0 * margin;

    let (x_min, x_max) = self.config.x_limit.unwrap_or((0.0, 1.0));
    let (y_min, y_max) = self.config.y_limit.unwrap_or((0.0, 1.0));

    // 计算数学上的 0 点对应的像素坐标
    // 如果 0 不在范围内，我们会把轴线固定在边缘（或者干脆不画）
    let x_zero_raw = (0.0 - x_min) / (x_max - x_min);
    let y_zero_raw = (0.0 - y_min) / (y_max - y_min);

    // 限制在 [0, 1] 范围内，防止轴线画到 Viewport 外面
    let x_axis_px = margin + x_zero_raw.clamp(0.0, 1.0) * plot_w;
    let y_axis_py = (height - margin) - y_zero_raw.clamp(0.0, 1.0) * plot_h;

    let mut pb = PathBuilder::new();
    let arrow_len = 10.0;
    let tick_size = margin * 0.1;

    // --- 1. 画 X 轴 (它穿过 Y=0 的地方) ---
    pb.move_to(margin, y_axis_py);
    pb.line_to(width - margin, y_axis_py);
    // X 轴箭头
    pb.move_to(width - margin, y_axis_py);
    pb.line_to(width - margin - arrow_len, y_axis_py - arrow_len * 0.5);
    pb.move_to(width - margin, y_axis_py);
    pb.line_to(width - margin - arrow_len, y_axis_py + arrow_len * 0.5);

    // --- 2. 画 Y 轴 (它穿过 X=0 的地方) ---
    pb.move_to(x_axis_px, height - margin);
    pb.line_to(x_axis_px, margin);
    // Y 轴箭头
    pb.move_to(x_axis_px, margin);
    pb.line_to(x_axis_px - arrow_len * 0.5, margin + arrow_len);
    pb.move_to(x_axis_px, margin);
    pb.line_to(x_axis_px + arrow_len * 0.5, margin + arrow_len);

    // --- 3. 画刻度 (基于数学步长) ---
    let num_ticks = 10;
    for i in 0..=num_ticks {
      let t = i as f32 / num_ticks as f32;

      // X 刻度
      let px = margin + t * plot_w;
      pb.move_to(px, y_axis_py); // 刻度长在轴线上
      pb.line_to(px, y_axis_py - tick_size);

      // Y 刻度
      let py = (height - margin) - t * plot_h;
      pb.move_to(x_axis_px, py);
      pb.line_to(x_axis_px + tick_size, py);
    }

    // 渲染
    let base_ts = Transform::from_translate(self.x, self.y);
    let mut paint = Paint::default();
    paint.set_color_rgba8(200, 200, 200, 255);
    paint.anti_alias = true;
    let stroke = Stroke {
      width: 1.5,
      ..Stroke::default()
    };

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
    self.render_axis(pixmap);

    let base_ts = Transform::from_translate(self.x, self.y);
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

    // 映射逻辑：
    // 第一步：将数学点 (x, y) 平移到 (x - x_min, y - y_min)，使其从 0 开始
    // 第二步：缩放，并将 Y 轴翻转（数学向上为正，屏幕向下为正）
    // 第三步：平移到屏幕上的绘图起始点 (margin, height - margin)
    let inner_ts = Transform::from_translate(margin, height - margin)
      .pre_scale(plot_w / x_range, -plot_h / y_range)
      .pre_translate(-x_min, -y_min);

    let ts = base_ts.pre_concat(inner_ts);

    for drawable in &self.drawables {
      drawable.draw(pixmap, &ts);
    }
  }
  pub fn add(&mut self, drawable: Box<dyn Drawable>) {
    self.drawables.push(drawable);
  }
}

#[derive(Default)]
pub struct Config {
  x_limit: Option<(f32, f32)>,
  y_limit: Option<(f32, f32)>,
}
