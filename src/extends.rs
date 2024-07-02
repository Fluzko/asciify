use image::{Rgb, Rgba};

pub trait StdColor {
    fn rgb(&self, pixel: Rgb<u8>) -> String;
    fn rgba(&self, pixel: Rgba<u8>) -> String;
}

impl StdColor for char {
    fn rgba(&self, pixel: Rgba<u8>) -> String {
        let r = pixel[0];
        let g = pixel[1];
        let b = pixel[2];

        self.rgb(Rgb([r, g, b]))
    }

    fn rgb(&self, pixel: Rgb<u8>) -> String {
        let r = pixel[0] as usize;
        let g = pixel[1] as usize;
        let b = pixel[2] as usize;

        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }
}

pub trait PixelExtended {
    fn strength(&self) -> usize;
    fn is_black(&self) -> bool;
}

impl PixelExtended for Rgb<u8> {
    fn strength(&self) -> usize {
        let r = self[0] as usize;
        let g = self[1] as usize;
        let b = self[2] as usize;

        (r + g + b) / 3
    }

    fn is_black(&self) -> bool {
        self[0] == 0 && self[1] == 0 && self[2] == 0
    }
}

impl PixelExtended for Rgba<u8> {
    fn strength(&self) -> usize {
        let r = self[0] as usize;
        let g = self[1] as usize;
        let b = self[2] as usize;
        let a = self[3] as usize;

        (r + g + b + a) / 4
    }

    fn is_black(&self) -> bool {
        self[0] == 0 && self[1] == 0 && self[2] == 0
    }
}

impl PixelExtended for image::Luma<u8> {
    fn strength(&self) -> usize {
        self[0] as usize
    }

    fn is_black(&self) -> bool {
        self[0] == 0
    }
}
