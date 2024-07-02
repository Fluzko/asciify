use clap::Parser;
use image::{io::Reader as ImageReader, GenericImageView};

mod args;
use args::{Args, FilterExt};
mod extends;
mod strategies;
use strategies::get_image_strategy;

fn open_image(path: &str) -> image::DynamicImage {
    ImageReader::open(path)
        .unwrap_or_else(|_| panic!("Invalid path"))
        .decode()
        .unwrap_or_else(|_| panic!("Invalid image format"))
}

// Re calculates the image dimentions to fit the new width by recalculating the aspect-ratio.
// Since the ASCII characters are not square, they're almost w * 2 = h
fn rescale_dimentions(img: &image::DynamicImage, new_width: u32) -> (u32, u32) {
    let (width, height) = img.dimensions();
    let aspect_ratio = width as f32 / height as f32;
    let new_height = ((new_width as f32 / aspect_ratio) / 2.0) as u32;

    (new_width, new_height)
}

fn main() {
    let args = Args::parse();

    let mut img = open_image(&args.path);

    let (new_width, new_height) = rescale_dimentions(&img, args.width);
    img = img.resize_exact(new_width, new_height, args.filter.to_image_filter());

    let image_strategy = get_image_strategy(&img, &args);

    for y in 0..new_height {
        for x in 0..new_width {
            let pixel = image_strategy.get_char(x, y);
            print!("{}", pixel);
        }
        println!();
    }
}
