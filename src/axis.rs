use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

pub struct Axis {
  pub x: f32,
  pub y: f32,
  pub veiwport: Rect,

  _config: Config,
}

impl Axis {
  pub fn new(x: f32, y: f32, size: (f32, f32)) -> Self {
    Self {
      x,
      y,
      veiwport: Rect::from_xywh(0., 0., size.0, size.1).unwrap(),
      _config: Config {},
    }
  }
  pub fn change_veiwport(&mut self, axis: (f32, f32), size: (f32, f32)) {
    self.x = axis.0;
    self.y = axis.1;
    self.veiwport = Rect::from_xywh(0., 0., size.0, size.1).unwrap();
  }
  pub fn render(&self, pixmap: &mut Pixmap) {
    let width = self.veiwport.width();
    let height = self.veiwport.height();
    // println!(
    //   "[Axis Render] Pos: ({}, {}), Viewport: {}x{}",
    //   self.x, self.y, width, height
    // );
    // 根据 x 坐标换个颜色，方便区分
    let margin: f32 = (width * 0.1).min(50.);
    let tick_size: f32 = margin * 0.1;
    let gap: f32 = margin;
    let mut paint = Paint::default();
    paint.set_color_rgba8(200, 200, 200, 255);

    let stroke = Stroke {
      width: 2.0,
      ..Stroke::default()
    };

    let origin = (margin, height - margin);

    let x_end = (width - margin, height - margin);
    let y_end = (margin, margin);
    // println!(
    //   "  [Path Info] Local Origin: {:?}, Local X-End: {:?}",
    //   origin, x_end
    // );

    let mut pb = PathBuilder::new();

    let arrow_len = 10.0;
    pb.move_to(origin.0, origin.1);
    pb.line_to(x_end.0, x_end.1);

    // --- 2. 绘制 X 轴箭头 (在 x_end 处) ---
    pb.move_to(x_end.0, x_end.1);
    pb.line_to(x_end.0 - arrow_len, x_end.1 - arrow_len * 0.5);
    pb.move_to(x_end.0, x_end.1);
    pb.line_to(x_end.0 - arrow_len, x_end.1 + arrow_len * 0.5);

    // --- 3. 绘制 Y 轴主线 (这里必须重新 move_to 到 origin) ---
    pb.move_to(origin.0, origin.1);
    pb.line_to(y_end.0, y_end.1);

    // --- 4. 绘制 Y 轴箭头 (在 y_end 处) ---
    pb.move_to(y_end.0, y_end.1);
    pb.line_to(y_end.0 - arrow_len * 0.5, y_end.1 + arrow_len);
    pb.move_to(y_end.0, y_end.1);
    pb.line_to(y_end.0 + arrow_len * 0.5, y_end.1 + arrow_len);
    // draw x ticks
    let mut x = origin.0 + gap;
    while x < x_end.0 {
      pb.move_to(x, origin.1);
      pb.line_to(x, origin.1 + tick_size);
      x += gap;
    }

    // draw y ticks
    let mut y = origin.1 - gap;
    while y > y_end.1 {
      pb.move_to(origin.0, y);
      pb.line_to(origin.0 - tick_size, y);
      y -= gap;
    }

    if let Some(path) = pb.finish() {
      pixmap.stroke_path(
        &path,
        &paint,
        &stroke,
        Transform::from_translate(self.x, self.y),
        None,
      );
    }
  }
}

pub struct Config {}
