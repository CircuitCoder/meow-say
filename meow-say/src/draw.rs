use image::{GrayImage, ImageBuffer, Luma, Rgb, Rgba, imageops::overlay};
use imageproc::{distance_transform::euclidean_squared_distance_transform, drawing::{draw_cubic_bezier_curve_mut, draw_text_mut}, filter::gaussian_blur_f32, map::map_pixels};
use rusttype::{Font, Scale};
use lazy_static::lazy_static;

static FONT_RAW: &'static [u8] = include_bytes!("./res/font.ttf");

lazy_static! {
  static ref FONT: Option<Font<'static>> = Font::try_from_bytes(FONT_RAW);
}

fn expand(image: &GrayImage, size: f64) -> GrayImage {
  let dist = euclidean_squared_distance_transform(image);
  let filtered = map_pixels(&dist, |_x, _y, pix| {
    if pix.0[0] > size {
      return Luma([0u8]);
    } else {
      return Luma([255u8]);
    }
  });
  let blurred = gaussian_blur_f32(&filtered, 0.7);
  blurred
}

fn color(image: GrayImage, primary: Rgb<u8>) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
  map_pixels(&image, |_x, _y, pix| {
    return Rgba([primary.0[0], primary.0[1], primary.0[2], pix.0[0]]);
  })
}

pub fn draw(content: &str, img: &crate::img::Img, fg: Rgb<u8>) -> anyhow::Result<ImageBuffer<Rgba<u8>, Vec<u8>>> {
  let (img_width, img_height) = img.dimensions();
  let width = img_width + 300;

  // TODO: dynamic height: line wrap
  let height = img_height + 225;

  let mut line_inner = ImageBuffer::new(width, height);

  let scale = Scale {
    x: 80.0,
    y: 80.0
  };
  draw_cubic_bezier_curve_mut(&mut line_inner, (250.0, 275.0), (150.0, 175.0), (200.0, 275.0), (175.0, 200.0), Luma([255u8]));
  let font = FONT.as_ref().ok_or(anyhow::anyhow!("Invalid font"))?;
  let mut foreground = expand(&line_inner, 20.0);
  draw_text_mut(&mut foreground, Luma([255u8]), 50, 50, scale, font, content);

  let background = expand(&foreground, 20.0);
  let mut buffer = color(background, Rgb([255, 255, 255]));

  let foreground = color(foreground, fg);

  overlay(&mut buffer, &foreground, 0, 0);
  overlay(&mut buffer, img, width - img_width - 50, height - img_height - 50);

  // Padding = 50, position

  Ok(buffer)
}