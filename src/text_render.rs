use ab_glyph::{Font, FontVec, Point, ScaleFont};
use fontdb::{Database, Family, Query};
use tiny_skia::{Color, IntSize, Paint, Pixmap, Rect, Transform};

pub struct TextRender {
  font: FontVec,
  db: Database,
}

impl TextRender {
  pub fn new() -> Self {
    let mut db = Database::new();
    db.load_system_fonts();
    let source = include_bytes!("./maple.ttf");
    let font = FontVec::try_from_vec(source.to_vec()).unwrap();
    Self { font, db }
  }

  pub fn has_family(&self, family: &str) -> bool {
    let query = Query {
      families: &[Family::Name(family), Family::SansSerif],
      ..Default::default()
    };
    self.db.query(&query).is_some()
  }

  pub fn load_font(&mut self, famile_name: &str) {
    let query = Query {
      families: &[Family::Name(famile_name), Family::SansSerif],
      ..Default::default()
    };

    if let Some(id) = self.db.query(&query) {
      self.db.with_face_data(id, |source, index| {
        self.font = FontVec::try_from_vec_and_index(source.to_vec(), index).unwrap();
      });
    } else {
      // load font in the lib
      self.load_default_font();
    }
  }

  pub fn draw(
    &self, pixmap: &mut tiny_skia::Pixmap, text: &str, x: f32, y: f32, size: f32,
    color: tiny_skia::Color,
  ) {
    let font = &self.font;
    let scale = ab_glyph::PxScale::from(size);
    let scaled_font = font.as_scaled(scale);

    let adjusted_y = y + scaled_font.ascent();
    let mut x_cursor = x;

    // 1. 先把基本信息提取出来，避免在闭包里借用 pixmap
    let img_w = pixmap.width();
    let img_h = pixmap.height();
    let pixels = pixmap.pixels_mut(); // 在这里获取整个像素数组的可变引用

    // 2. 预准备颜色分量
    let r_f = color.red();
    let g_f = color.green();
    let b_f = color.blue();
    let a_f = color.alpha();

    for c in text.chars() {
      let glyph_id = font.glyph_id(c);
      let glyph = glyph_id.with_scale_and_position(
        size,
        ab_glyph::Point {
          x: x_cursor,
          y: adjusted_y,
        },
      );

      if let Some(outlined) = font.outline_glyph(glyph) {
        let bounds = outlined.px_bounds();

        // 3. 在这里直接操作像素
        outlined.draw(|ox, oy, coverage| {
          if coverage > 0.0 {
            let px = (bounds.min.x as i32) + ox as i32;
            let py = (bounds.min.y as i32) + oy as i32;

            // 边界检查
            if px >= 0 && py >= 0 && (px as u32) < img_w && (py as u32) < img_h {
              let idx = (py as u32 * img_w + px as u32) as usize;

              // 获取当前像素
              let old_px = pixels[idx];

              // 计算 Alpha 混合
              // 文字颜色使用 Premultiplied 格式进行混合
              let alpha = coverage * a_f;
              let inv_alpha = 1.0 - alpha;

              let out_r = (r_f * alpha + (old_px.red() as f32 / 255.0) * inv_alpha).clamp(0.0, 1.0);
              let out_g =
                (g_f * alpha + (old_px.green() as f32 / 255.0) * inv_alpha).clamp(0.0, 1.0);
              let out_b =
                (b_f * alpha + (old_px.blue() as f32 / 255.0) * inv_alpha).clamp(0.0, 1.0);
              let out_a = (alpha + (old_px.alpha() as f32 / 255.0) * inv_alpha).clamp(0.0, 1.0);

              // 直接写回像素内存
              if let Some(new_color) = tiny_skia::Color::from_rgba(out_r, out_g, out_b, out_a) {
                pixels[idx] = new_color.premultiply().to_color_u8();
              }
            }
          }
        });
      }
      x_cursor += scaled_font.h_advance(glyph_id);
    }
  }
}

impl TextRender {
  fn load_default_font(&mut self) {
    let source = include_bytes!("./maple.ttf");
    match FontVec::try_from_vec(source.to_vec()) {
      Ok(font) => self.font = font,
      Err(e) => println!("Error loading font: {}", e),
    }
  }
}
#[test]
fn test_has_font() {
  let tr = TextRender::new();
  assert!(tr.has_family("Noto Serif Test"));

  assert!(!tr.has_family("FiraCode"));
}
