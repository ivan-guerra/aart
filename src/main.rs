use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(
        value_enum,
        help = "path to image file"
    )]
    image_path: std::path::PathBuf,

    #[arg(
        short = 's',
        long,
        default_value_t = 1.0, 
        help = "image scaling factor"
    )]
    scale: f64,

    #[arg(
        short = 'x',
        long,
        default_value_t = 10, 
        value_parser = clap::value_parser!(u32).range(1..=256),
        help = "character height"
    )]
    char_width: u32,

    #[arg(
        short = 'y',
        long,
        default_value_t = 18, 
        value_parser = clap::value_parser!(u32).range(1..=256),
        help = "character_width"
    )]
    char_height: u32,
}

fn main() {
    let args = Args::parse();
    if args.scale < 0.01 || args.scale > 1.0 {
        eprintln!("error: scale must be between 0.01 and 1.0");
        std::process::exit(1);
    }
    let config = aart::Config::new(
        args.image_path,
        args.scale,
        args.char_width,
        args.char_height,
    );

    if let Err(e) = aart::run(&config) {
        eprintln!("error: {}", e);
        std::process::exit(1);
    }
}
