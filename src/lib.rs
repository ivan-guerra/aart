use image::GenericImageView;

pub struct Config {
    pub image_path: std::path::PathBuf,
    pub scale: f64,
    pub char_width: u32,
    pub char_height: u32,
}

impl Config {
    pub fn new(
        image_path: std::path::PathBuf,
        scale: f64,
        char_width: u32,
        char_height: u32,
    ) -> Config {
        Config {
            image_path,
            scale,
            char_width,
            char_height,
        }
    }
}

fn scale_image(image: image::DynamicImage, config: &Config) -> image::DynamicImage {
    let char_scale = config.char_width as f64 / config.char_height as f64;
    let (width, height) = image.dimensions();
    let new_width = (width as f64 * config.scale) as u32;
    let new_height = (height as f64 * config.scale * char_scale) as u32;

    image.resize(new_width, new_height, image::imageops::FilterType::Nearest)
}

fn get_char(pixel: &image::Rgba<u8>) -> char {
    static ASCII_CHARS: &str =
        " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let avg_color = pixel.0.iter().map(|c| *c as f64).sum::<f64>() / 3.0;

    ASCII_CHARS
        .chars()
        .nth((avg_color as usize).rem_euclid(ASCII_CHARS.len()))
        .unwrap()
}

fn convert_image_to_ascii(image: image::DynamicImage, config: &Config) -> String {
    let image = scale_image(image, config);
    let (width, height) = image.dimensions();
    let mut ascii_image = String::new();

    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            ascii_image.push(get_char(&pixel));
        }
        ascii_image.push('\n');
    }

    ascii_image
}

pub fn run(config: &Config) -> Result<(), Box<dyn std::error::Error>> {
    let image = image::open(&config.image_path)?;
    let ascii_image = convert_image_to_ascii(image, config);

    println!("{}", ascii_image);

    Ok(())
}
