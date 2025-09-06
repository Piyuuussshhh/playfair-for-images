use clap::{arg, Parser};

mod image_handling;
use image_handling::input;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the image that the user wants to encrypt.
    #[arg(short, long)]
    image_path: String,
    /// The encryption key image path. If not provided, a random key image will be generated.
    #[arg(short, long, default_value = None)]
    keyimg_path: Option<String>,
}

fn main() {
    let args = Args::parse();
    input::encrypt_and_save_output(
        &args.image_path,
        args.keyimg_path,
        "output/cipherimage.png"
    );
}
