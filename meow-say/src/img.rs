use image;
use image::{ImageBuffer, Rgba};
use lazy_static::lazy_static;
use std::io::BufReader;

pub static CAT_1: &'static [u8] = include_bytes!("./res/img/1.png");
pub static CAT_2: &'static [u8] = include_bytes!("./res/img/2.png");
pub static CAT_3: &'static [u8] = include_bytes!("./res/img/3.png");

pub const CAT_CNT: usize = 3;

pub static CATS: [&'static [u8]; CAT_CNT] = [CAT_1, CAT_2, CAT_3];

pub type Img = ImageBuffer<Rgba<u8>, Vec<u8>>;

lazy_static! {
    pub static ref PARSED_CATS: anyhow::Result<Vec<Img>> = statics();
}

pub fn statics() -> anyhow::Result<Vec<Img>> {
    CATS.iter()
        .map(|raw| -> anyhow::Result<Img> {
            let ret = image::load_from_memory(*raw)?;
            Ok(ret.into_rgba8())
        })
        .collect()
}

pub fn dynamic(id: &str) -> anyhow::Result<Img> {
    let path = format!("src/res/img/{}.png", id);
    let file = std::fs::File::open(&path)?;
    let reader = BufReader::new(file);
    let img = image::load(reader, image::ImageFormat::Png)?;
    Ok(img.into_rgba8())
}
