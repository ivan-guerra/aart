//! ASCII Art Generator Library
//!
//! This library provides functionality to convert images into ASCII art representations.
//! It supports various configuration options such as image scaling and character aspect
//! ratio adjustments to ensure the output appears proportional in terminal displays.
//!
//! # Features
//!
//! - Convert any image format supported by the `image` crate to ASCII art
//! - Configurable scaling to control output size
//! - Character aspect ratio correction for terminal display
//! - Brightness-based character mapping for optimal visual representation
//!
//! # Example
//!
//! ```rust
//! use aart::{Config, run};
//! use std::path::PathBuf;
//!
//! let config = Config {
//!     image_path: PathBuf::from("example.jpg"),
//!     scale: 1.0,
//!     char_width: 2,
//!     char_height: 1,
//! };
//!
//! run(&config).expect("Failed to convert image to ASCII art");
//! ```
use image::{GenericImageView, Pixel};

/// Configuration parameters for ASCII art generation.
pub struct Config {
    /// Path to the source image file that will be converted to ASCII art
    pub image_path: std::path::PathBuf,
    /// Scaling factor to resize the image before conversion (e.g., 0.5 for half size, 2.0 for double size)
    pub scale: f64,
    /// Width of a single character in the terminal, used for aspect ratio correction
    pub char_width: u32,
    /// Height of a single character in the terminal, used for aspect ratio correction
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

/// Scales an image according to the provided configuration.
///
/// This function resizes the input image based on the scale factor and character dimensions
/// specified in the config. It accounts for terminal character aspect ratio to ensure the
/// output image appears proportional when displayed as ASCII art.
///
/// # Arguments
///
/// * `image` - The source image to be scaled
/// * `config` - Configuration parameters containing scale factor and character dimensions
///
/// # Returns
///
/// Returns a new `DynamicImage` that has been scaled according to the configuration
///
/// # Example
///
/// ```
/// use aart::{Config, scale_image};
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgb8(100, 100);
/// let config = Config {
///     scale: 2.0,
///     char_width: 2,
///     char_height: 1,
///     ..Default::default()
/// };
///
/// let scaled = scale_image(img, &config);
/// ```
pub fn scale_image(image: image::DynamicImage, config: &Config) -> image::DynamicImage {
    let char_scale = config.char_width as f64 / config.char_height as f64;
    let (width, height) = image.dimensions();
    let new_width = (width as f64 * config.scale) as u32;
    let new_height = (height as f64 * config.scale * char_scale) as u32;

    image.resize(new_width, new_height, image::imageops::FilterType::Nearest)
}

/// Converts a pixel's color values to a corresponding ASCII character based on brightness.
///
/// This function takes an RGBA pixel and returns an ASCII character that represents
/// its brightness level. It calculates the average of RGB channels (ignoring alpha)
/// and maps it to a character from a predefined set of ASCII characters ranging from
/// darkest (' ') to brightest ('$').
///
/// # Arguments
///
/// * `pixel` - An RGBA pixel from the image
///
/// # Returns
///
/// Returns a character from the ASCII set that corresponds to the pixel's brightness
///
/// # Example
///
/// ```
/// use image::Rgba;
/// use aart::get_char;
///
/// let pixel = Rgba([128, 128, 128, 255]); // Medium grey pixel
/// let ascii_char = get_char(&pixel);
/// ```
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

    ASCII_CHARS
        .chars()
        .nth((avg_color as usize).rem_euclid(ASCII_CHARS.len()))
        .unwrap()
}

/// Converts an image into ASCII art representation.
///
/// This function takes a source image and configuration parameters to produce
/// an ASCII art string. The process involves scaling the image according to
/// the configuration and then converting each pixel to a corresponding ASCII
/// character based on its brightness.
///
/// # Arguments
///
/// * `image` - The source image to convert to ASCII art
/// * `config` - Configuration parameters for scaling and character dimensions
///
/// # Returns
///
/// Returns a String containing the ASCII art representation of the image,
/// with newline characters separating each row.
///
/// # Example
///
/// ```
/// use aart::{Config, convert_image_to_ascii};
/// use image::DynamicImage;
///
/// let img = DynamicImage::new_rgb8(100, 100);
/// let config = Config {
///     scale: 1.0,
///     char_width: 2,
///     char_height: 1,
///     ..Default::default()
/// };
///
/// let ascii_art = convert_image_to_ascii(img, &config);
/// println!("{}", ascii_art);
/// ```
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

/// Executes the main ASCII art conversion process.
///
/// This function orchestrates the complete process of converting an image to ASCII art:
/// 1. Opens the image file specified in the configuration
/// 2. Converts the image to ASCII art
/// 3. Prints the result to standard output
///
/// # Arguments
///
/// * `config` - Configuration parameters including the image path and conversion settings
///
/// # Returns
///
/// Returns `Ok(())` if successful, or an error if the image cannot be opened or processed
///
/// # Errors
///
/// This function will return an error if:
/// - The image file cannot be found or opened
/// - The image format is invalid or unsupported
///
/// # Example
///
/// ```
/// use aart::{Config, run};
/// use std::path::PathBuf;
///
/// let config = Config {
///     image_path: PathBuf::from("image.jpg"),
///     scale: 1.0,
///     char_width: 2,
///     char_height: 1,
/// };
///
/// match run(&config) {
///     Ok(_) => println!("Successfully converted image to ASCII art"),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
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
