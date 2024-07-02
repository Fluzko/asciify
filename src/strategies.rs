use image::{DynamicImage, ImageBuffer, Rgb};

use crate::args::Args;
use crate::extends::{PixelExtended, StdColor};

static CHARSET: [char; 5] = [' ', '░', '▒', '▓', '█'];
static WHITE: Rgb<u8> = Rgb([255, 255, 255]);

pub trait ImageStrategy {
    fn get_char(&self, x: u32, y: u32) -> String;
}

struct BaseStrategy {
    matrix: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}
struct PngStrategy {
    matrix: ImageBuffer<image::Rgba<u8>, Vec<u8>>,
    transparency_threshold: u8,
}
struct BnWStrategy {
    matrix: ImageBuffer<image::Luma<u8>, Vec<u8>>,
}

impl BaseStrategy {
    fn new(img: &DynamicImage) -> Self {
        let matrix = img.to_rgb8();
        BaseStrategy { matrix }
    }
}
impl PngStrategy {
    fn new(img: &DynamicImage, transparency_threshold: u8) -> Self {
        let matrix = img.to_rgba8();
        PngStrategy {
            matrix,
            transparency_threshold,
        }
    }
}
impl BnWStrategy {
    fn new(img: &DynamicImage) -> Self {
        let matrix = img.to_luma8();

        BnWStrategy { matrix }
    }
}

impl ImageStrategy for BaseStrategy {
    fn get_char(&self, x: u32, y: u32) -> String {
        let pixel = self.matrix[(x, y)];

        let char = CHARSET.last().unwrap();

        char.rgb(pixel)
    }
}

impl ImageStrategy for PngStrategy {
    fn get_char(&self, x: u32, y: u32) -> String {
        let pixel = self.matrix[(x, y)];
        let char = CHARSET.last().unwrap();

        if pixel.is_black() && pixel[3] < self.transparency_threshold {
            char.rgb(WHITE)
        } else {
            char.rgba(pixel)
        }
    }
}

impl ImageStrategy for BnWStrategy {
    fn get_char(&self, x: u32, y: u32) -> String {
        let pixel = self.matrix[(x, y)];

        let color_strength = pixel.strength();
        let step_size = 255 / CHARSET.len();
        let max_index = CHARSET.len() - 1;
        let char_index = (color_strength / step_size).min(max_index);

        CHARSET[char_index].rgb(WHITE)
    }
}

pub fn get_image_strategy(img: &DynamicImage, args: &Args) -> Box<dyn ImageStrategy> {
    if args.black_and_white {
        Box::new(BnWStrategy::new(img))
    } else if args.path.ends_with(".png") {
        Box::new(PngStrategy::new(img, args.transparency_threshold))
    } else {
        Box::new(BaseStrategy::new(img))
    }
}
