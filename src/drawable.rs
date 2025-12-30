use tiny_skia::{Pixmap, Transform};

pub struct Bound {
  pub x_min: f32,
  pub x_max: f32,
  pub y_min: f32,
  pub y_max: f32,
}
pub trait Drawable {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform);
  fn bound(&self) -> Option<Bound>;
  fn get_color(&self) -> [u8; 4];
  fn set_color(&self, color: [u8; 4]);
}
