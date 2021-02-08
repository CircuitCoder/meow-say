use image::Rgb;

pub mod draw;
pub mod img;

pub fn try_parse_color(s: &str) -> anyhow::Result<Rgb<u8>> {
  if s.len() != 7 || s.as_bytes()[0] != b'#' {
    return Err(anyhow::anyhow!("Invalid color format"));
  }

  let r = u8::from_str_radix(&s[1..3], 16)?;
  let g = u8::from_str_radix(&s[3..5], 16)?;
  let b = u8::from_str_radix(&s[5..7], 16)?;

  Ok(Rgb([r, g, b]))
}