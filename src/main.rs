use clap::{arg, Parser};

mod image_handling;
use image_handling::input;

#[derive(Parser, Debug)]
struct Args {
    /// Path to the image that the user wants to encrypt
    #[arg(short, long)]
    image_path: String,
    /// Whether to randomize the encryption key
    #[arg(short, long, default_value = None)]
    keyimg_path: Option<String>,
}

fn main() {
    // Take path of the image that the user wants to encrypt as input
    let args = Args::parse();
    input::encrypt_and_save_output(
        &args.image_path,
        args.keyimg_path,
        "output/cipherimage.png"
    );
}
