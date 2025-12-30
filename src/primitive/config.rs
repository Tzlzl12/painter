pub struct Config {
  pub is_hidden: bool,
  pub color: [u8; 4],
  pub stroke_width: f32,
}
impl Default for Config {
  fn default() -> Self {
    Self {
      is_hidden: false,
      color: [255, 255, 255, 255],
      stroke_width: 2.0,
    }
  }
}
