use image::{GenericImageView, Pixel};

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

impl Default for Config {
    fn default() -> Self {
        Config {
            image_path: std::path::PathBuf::new(),
            scale: 1.0,
            char_width: 1,
            char_height: 1,
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

pub fn get_char(pixel: &image::Rgba<u8>) -> char {
    static ASCII_CHARS: &str =
        " .'`^\",:;Il!i><~+_-?][}{1)(|\\/tfjrxnuvczXYUJCLQ0OZmwqpdbkhao*#MW&8%B@$";
    let avg_color = pixel
        .channels()
        .iter()
        .take(3)
        .map(|&c| c as f64)
        .sum::<f64>()
        / 3.0;
    dbg!(avg_color);
    dbg!(avg_color as usize);
    dbg!((avg_color as usize).rem_euclid(ASCII_CHARS.len()));

    ASCII_CHARS
        .chars()
        .nth((avg_color as usize).rem_euclid(ASCII_CHARS.len()))
        .unwrap()
}

pub fn convert_image_to_ascii(image: image::DynamicImage, config: &Config) -> String {
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

#[cfg(test)]
mod tests {
    use super::*;
    use image::Rgba;

    #[test]
    fn get_char_returns_correct_char_on_black_pixel() {
        let black_pixel = Rgba([0, 0, 0, 255]);
        assert_eq!(get_char(&black_pixel), ' ');
    }

    #[test]
    fn get_char_returns_correct_char_on_white_pixel() {
        let white_pixel = Rgba([255, 255, 255, 255]);
        assert_eq!(get_char(&white_pixel), 'L');
    }

    #[test]
    fn get_char_returns_correct_char_on_grey_pixel() {
        let grey_pixel = Rgba([128, 128, 128, 255]);
        let result = get_char(&grey_pixel);
        assert_eq!(result, 'a');
    }

    #[test]
    fn get_char_returns_correct_char_on_equivalent_avg() {
        let mixed_pixel = Rgba([50, 100, 150, 255]);
        // Average is 100, should return consistent character
        let result = get_char(&mixed_pixel);
        let same_avg_pixel = Rgba([100, 100, 100, 255]);
        assert_eq!(result, get_char(&same_avg_pixel));
    }

    #[test]
    fn get_char_ignores_alpha_channel() {
        let pixel1 = Rgba([100, 100, 100, 255]);
        let pixel2 = Rgba([100, 100, 100, 0]);
        assert_eq!(get_char(&pixel1), get_char(&pixel2));
    }

    use image::{DynamicImage, GenericImageView, ImageBuffer, Rgb};

    fn create_test_image(width: u32, height: u32) -> DynamicImage {
        let img = ImageBuffer::from_fn(width, height, |_, _| Rgb([100, 100, 100]));
        DynamicImage::ImageRgb8(img)
    }

    #[test]
    fn scale_image_does_no_scaling_when_input_and_output_dimensions_are_equal() {
        let input = create_test_image(100, 100);
        let config = Config {
            scale: 1.0,
            char_width: 1,
            char_height: 1,
            ..Default::default()
        };

        let scaled = scale_image(input, &config);
        assert_eq!(scaled.dimensions(), (100, 100));
    }

    #[test]
    fn scale_image_doubles_output_image_scale() {
        let input = create_test_image(100, 100);
        let config = Config {
            scale: 2.0,
            char_width: 1,
            char_height: 1,
            ..Default::default()
        };

        let scaled = scale_image(input, &config);
        assert_eq!(scaled.dimensions(), (200, 200));
    }
}
