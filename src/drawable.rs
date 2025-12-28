use tiny_skia::{Pixmap, Transform};

pub trait Drawable {
  fn draw(&self, pixmap: &mut Pixmap, ts: &Transform);
}
