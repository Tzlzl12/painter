use tiny_skia::{Paint, PathBuilder, Pixmap, Rect, Stroke, Transform};

use crate::drawable::Drawable;

pub struct Axis {
  pub x: f32,
  pub y: f32,
  pub veiwport: Rect,
  drawables: Vec<Box<dyn Drawable>>,

  _config: Config,
}

impl Axis {
  pub fn new(x: f32, y: f32, size: (f32, f32)) -> Self {
    Self {
      x,
      y,
      veiwport: Rect::from_xywh(0., 0., size.0, size.1).unwrap(),
      drawables: Vec::new(),
      _config: Config::default(),
    }
  }
  pub fn set_x_limit(&mut self, limit: (f32, f32)) {
    self._config.x_limit = limit;
  }
  pub fn set_y_limit(&mut self, limit: (f32, f32)) {
    self._config.y_limit = limit;
  }
  pub fn change_veiwport(&mut self, axis: (f32, f32), size: (f32, f32)) {
    self.x = axis.0;
    self.y = axis.1;
    self.veiwport = Rect::from_xywh(0., 0., size.0, size.1).unwrap();
  }
  pub fn render(&self, pixmap: &mut Pixmap) {
    let width = self.veiwport.width();
    let height = self.veiwport.height();

    // dynamic set the option
    let margin: f32 = (width * 0.1).min(50.);
    let tick_size: f32 = margin * 0.1;
    let gap: f32 = margin;

    let mut paint = Paint::default();
    paint.set_color_rgba8(200, 200, 200, 255);

    let stroke = Stroke {
      width: 3.0,
      ..Stroke::default()
    };

    let origin = (margin, height - margin);

    let x_end = (width - margin, height - margin);
    let y_end = (margin, margin);

    let plot_w = width - 2. * margin;
    let plot_h = height - 2. * margin;

    let mut pb = PathBuilder::new();

    let arrow_len = 10.0;
    // draw x axis
    pb.move_to(origin.0, origin.1);
    pb.line_to(x_end.0, x_end.1);

    // draw x arrow
    pb.move_to(x_end.0, x_end.1);
    pb.line_to(x_end.0 - arrow_len, x_end.1 - arrow_len * 0.5);
    pb.move_to(x_end.0, x_end.1);
    pb.line_to(x_end.0 - arrow_len, x_end.1 + arrow_len * 0.5);

    // draw y axis
    pb.move_to(origin.0, origin.1);
    pb.line_to(y_end.0, y_end.1);

    // draw y arrow
    pb.move_to(y_end.0, y_end.1);
    pb.line_to(y_end.0 - arrow_len * 0.5, y_end.1 + arrow_len);
    pb.move_to(y_end.0, y_end.1);
    pb.line_to(y_end.0 + arrow_len * 0.5, y_end.1 + arrow_len);
    // draw x ticks
    let mut x = origin.0 + gap;
    while x < x_end.0 {
      pb.move_to(x, origin.1);
      pb.line_to(x, origin.1 - tick_size);
      x += gap;
    }

    // draw y ticks
    let mut y = origin.1 - gap;
    while y > y_end.1 {
      pb.move_to(origin.0, y);
      pb.line_to(origin.0 + tick_size, y);
      y -= gap;
    }

    let base_ts = Transform::from_translate(self.x, self.y);
    if let Some(path) = pb.finish() {
      pixmap.stroke_path(&path, &paint, &stroke, base_ts, None);
    }

    let x_range = self._config.x_limit.1 - self._config.x_limit.0;
    let y_range = self._config.y_limit.1 - self._config.y_limit.0;

    // 如果范围是 0，防止除以 0 导致崩溃
    if x_range == 0.0 || y_range == 0.0 {
      return;
    }
    let inner_ts = Transform::from_translate(margin, self.veiwport.height() - margin)
      .pre_scale(plot_w / x_range, -plot_h / y_range)
      .pre_translate(-self._config.x_limit.0, -self._config.y_limit.0);
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
  x_limit: (f32, f32),
  y_limit: (f32, f32),
}
