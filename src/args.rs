use clap::{value_parser, Parser, ValueEnum};

#[derive(ValueEnum, Debug, Clone)]
pub enum Filter {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

pub trait FilterExt {
    fn to_image_filter(&self) -> image::imageops::FilterType;
}
impl FilterExt for Filter {
    fn to_image_filter(&self) -> image::imageops::FilterType {
        match self {
            Filter::Nearest => image::imageops::FilterType::Nearest,
            Filter::Triangle => image::imageops::FilterType::Triangle,
            Filter::CatmullRom => image::imageops::FilterType::CatmullRom,
            Filter::Gaussian => image::imageops::FilterType::Gaussian,
            Filter::Lanczos3 => image::imageops::FilterType::Lanczos3,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub path: String,

    #[arg(short, long, default_value_t = 100)]
    pub width: u32,

    #[arg(short, long, value_enum, default_value_t = Filter::CatmullRom)]
    pub filter: Filter,

    #[arg(short, long, default_value_t = 25,  value_parser = value_parser!(u8).range(0..=255))]
    pub transparency_threshold: u8,

    #[arg(short, long, default_value_t = false)]
    pub black_and_white: bool,
}
