use std::fmt::Display;
use std::path::Path;
use clap::{CommandFactory, Parser, ValueEnum};
use clap::error::ErrorKind;
use image::io::Reader as ImageReader;
use image::{GenericImageView, load_from_memory};
use image::imageops::FilterType;

pub const DEFAULT_GRADIENT: &'static str = " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";

#[derive(ValueEnum, Clone)]
enum ColorMode {
    /// Red channel (alpha is preserved)
    R,
    /// Green channel (alpha is preserved)
    G,
    /// Blue channel (alpha is preserved)
    B,
    /// Alpha channel
    A,
    /// Intensity
    Intensity,
    /// Value
    Value,
    /// Saturation
    Saturation,
    /// Hue
    Hue,
    /// Luminance
    Luminance
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path or link to the image file
    #[arg()]
    file: String,
    /// Color mode
    #[arg(long, short, value_enum, default_value_t = ColorMode::Intensity)]
    mode: ColorMode,
    /// Character gradient (must be at least 2 characters)
    #[arg(long, short, default_value_t = DEFAULT_GRADIENT.to_string())]
    gradient: String,
    /// Gaussian blur sigma
    #[arg(long, short, default_value_t = 0.2)]
    blur: f32,
    /// Inverts image's colors
    #[arg(long, short, default_value_t = false)]
    invert: bool,
    /// Image's size multiplier
    #[arg(long, short, default_value_t = 1.0)]
    aspect_ratio: f32
}

fn error(err: ErrorKind, msg: impl Display) -> ! {
    Args::command().error(err, msg).exit();
}

fn u8_to_f32(rgba: [u8; 4]) -> [f32; 4] {
    rgba.map(|x| (x as f32) / (u8::MAX as f32))
}

fn to_ascii(gradient: &str, value: f32) -> char {
    gradient.chars().nth((value * ((gradient.chars().count() - 1) as f32)).round() as usize).unwrap()
}

fn max(rgba: [f32; 4]) -> f32 {
    rgba[0..3].iter().copied().reduce(f32::max).unwrap()
}

fn min(rgba: [f32; 4]) -> f32 {
    rgba[0..3].iter().copied().reduce(f32::min).unwrap()
}

fn intensity(rgba: [f32; 4]) -> f32 {
    (rgba[0] + rgba[1] + rgba[2]) / 3.0
}

fn value(rgba: [f32; 4]) -> f32 {
    max(rgba)
}

fn saturation(rgba: [f32; 4]) -> f32 {
    let max = max(rgba);

    if max == 0.0 {
        return 0.0;
    }

    (max - min(rgba)) / max
}

fn hue(rgba: [f32; 4]) -> f32 {
    let max = max(rgba);
    let min = min(rgba);

    let hue;

    if (max - min) == 0.0 {
        hue = 0.0;
    }
    else if max == rgba[0] {
        hue = 60.0 * ((rgba[1] - rgba[2]) / (max - min).rem_euclid(6.0));
    }
    else if max == rgba[1] {
        hue = 60.0 * ((rgba[2] - rgba[0]) / (max - min) + 2.0);
    }
    else {
        hue = 60.0 * ((rgba[0] - rgba[1]) / (max - min) + 4.0);
    }

    hue / 360.0
}

fn luminance(rgba: [f32; 4]) -> f32 {
    (max(rgba) + min(rgba)) / 2.0
}

fn main() {
    let args = Args::parse();

    if args.gradient.len() < 2 {
        error(ErrorKind::InvalidValue, "gradient length must be at least 2");
    }

    let path = Path::new(&args.file);
    let mut image;

    if path.exists() {
        let reader = match ImageReader::open(path) {
            Ok(reader) => reader,
            Err(_) => error(ErrorKind::Io, format!("couldn't open file at '{}'", &args.file))
        };

        image = match reader.decode() {
            Ok(image) => image,
            Err(_) => error(ErrorKind::InvalidValue, "unsupported format or corrupted image file")
        };
    }
    else {
        let response = match reqwest::blocking::get(&args.file) {
            Ok(response) => response,
            Err(_) => error(ErrorKind::Io, format!("failed downloading file from '{}'", &args.file))
        };

        let bytes = match response.bytes() {
            Ok(bytes) => bytes,
            Err(_) => error(ErrorKind::InvalidValue, "unsupported format or corrupted image file")
        };

        image = match load_from_memory(&bytes) {
            Ok(image) => image,
            Err(_) => error(ErrorKind::InvalidValue, "unsupported format or corrupted image file")
        };
    }

    image = image.resize((image.width() as f32 * args.aspect_ratio).floor() as u32,
                         (image.height() as f32 * args.aspect_ratio).floor() as u32,
                         FilterType::Lanczos3);

    if args.invert {
        image.invert();
    }

    image = image.blur(args.blur);

    let mut ascii = vec![String::new(); image.height() as usize];

    for x in 0..image.width() {
        for y in 0..image.height() {
            let rgba = u8_to_f32(image.get_pixel(x, y).0);

            ascii
                .iter_mut()
                .nth(y as usize)
                .unwrap()
                .push(match args.mode {
                    ColorMode::R => to_ascii(&args.gradient, rgba[0] * rgba[3]),
                    ColorMode::G => to_ascii(&args.gradient, rgba[1] * rgba[3]),
                    ColorMode::B => to_ascii(&args.gradient, rgba[2] * rgba[3]),
                    ColorMode::A => to_ascii(&args.gradient, rgba[3]),
                    ColorMode::Intensity => to_ascii(&args.gradient, intensity(rgba) * rgba[3]),
                    ColorMode::Value => to_ascii(&args.gradient, value(rgba) * rgba[3]),
                    ColorMode::Saturation => to_ascii(&args.gradient, saturation(rgba) * rgba[3]),
                    ColorMode::Hue => to_ascii(&args.gradient, hue(rgba) * rgba[3]),
                    ColorMode::Luminance => to_ascii(&args.gradient, luminance(rgba) * rgba[3]),
                });
        }
    }

    println!("{}", ascii.join("\n"));
}