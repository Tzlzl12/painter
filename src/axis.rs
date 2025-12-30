use std::rc::Rc;

use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

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
    let fg = color::get_fg();
    paint.set_color_rgba8(fg[0], fg[1], fg[2], fg[3]);
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

    // 1. 计算比例
    let px_per_unit_x = plot_w / x_range;
    let px_per_unit_y = plot_h / y_range;
    let scale = px_per_unit_x.min(px_per_unit_y);

    // 2. 【核心修改】计算偏移量以实现居中
    // 剩余的宽度 / 2 = 左侧额外需要的位移
    let x_offset = (plot_w - (x_range * scale)) / 2.0;
    // 剩余的高度 / 2 = 底部额外需要的位移（向上提）
    let y_offset = (plot_h - (y_range * scale)) / 2.0;

    // 3. 应用变换
    // margin + x_offset 让图形横向居中
    // height - (margin + y_offset) 让图形纵向居中
    let inner_ts = Transform::from_translate(margin + x_offset, height - (margin + y_offset))
      .pre_scale(scale, -scale)
      .pre_translate(-x_min, -y_min);

    let ts = base_ts.pre_concat(inner_ts);

    // ... 剩下的遍历绘制逻辑 ...
    for drawable in &self.drawables {
      // 颜色分配逻辑保持不变
      if drawable.get_color() == [255, 255, 255, 255] {
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
}
