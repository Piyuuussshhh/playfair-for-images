use std::thread;
use image::GenericImageView;

use super::{encrypt, keyimg, pairing};

pub fn encrypt_and_save_output(path: &str, keyimg_path: Option<String>, output_path: &str) {
    // Get plaintext image.
    let pt_img = image::open(path).expect("Failed to open image");
    let (width, height) = pt_img.dimensions();
    println!("Opened image with dimensions {width} x {height}");
    if width * height % 2 != 0 {
        panic!("Image dimensions must be even");
    }

    // Get key image and generate playfair matrix.
    let key_image = if let Some(_key_path) = keyimg_path {
        todo!()
    } else {
        keyimg::generate_random_key_image()
    };
    let key_matrix = keyimg::make_playfair_matrix(&key_image);
    println!("Generated key matrix: {:?}", key_matrix);

    // Don't need the alpha channel.
    let pt_img = pt_img.to_rgb8();
    // Separate the image into its RGB channels.
    let mut pt_img_rc = Vec::with_capacity(pt_img.len() / 3);
    let mut pt_img_gc = Vec::with_capacity(pt_img.len() / 3);
    let mut pt_img_bc = Vec::with_capacity(pt_img.len() / 3);
    pt_img.pixels().for_each(|pixel| {
        pt_img_rc.push(pixel[0]);
        pt_img_gc.push(pixel[1]);
        pt_img_bc.push(pixel[2]);
    });
    // Prepare cipherimage RGB channels.
    let mut ct_img_rc = vec![0u8; pt_img_rc.len()];
    let mut ct_img_gc = vec![0u8; pt_img_gc.len()];
    let mut ct_img_bc = vec![0u8; pt_img_bc.len()];

    // Apply encryption to each channel separately, concurrently.
    thread::scope(|s| {
        s.spawn(|| {
            // encrypt::encrypt_channel(&pt_img_rc, &mut ct_img_rc, &key_matrix, pairing::halfway_pairing);
            encrypt::encrypt_channel(&pt_img_rc, &mut ct_img_rc, &key_matrix, pairing::random_pairing);
        });

        s.spawn(|| {
            // encrypt::encrypt_channel(&pt_img_gc, &mut ct_img_gc, &key_matrix, pairing::sequential_pairing);
            encrypt::encrypt_channel(&pt_img_gc, &mut ct_img_gc, &key_matrix, pairing::random_pairing);
        });

        s.spawn(|| {
            // encrypt::encrypt_channel(&pt_img_bc, &mut ct_img_bc, &key_matrix, pairing::opposite_pairing);
            encrypt::encrypt_channel(&pt_img_bc, &mut ct_img_bc, &key_matrix, pairing::random_pairing);
        });
    });

    // Assemble and save the cipherimage.
    let ct_img = assemble_cipherimg(width, height, &ct_img_rc, &ct_img_gc, &ct_img_bc);
    ct_img.save(output_path).expect("Failed to save image");
}

fn assemble_cipherimg(
    width: u32,
    height: u32,
    ct_img_rc: &[u8],
    ct_img_gc: &[u8],
    ct_img_bc: &[u8],
) -> image::RgbImage {
    let mut img_buf = image::RgbImage::new(width, height);
    img_buf.pixels_mut().enumerate().for_each(|(i, pixel)| {
        pixel[0] = ct_img_rc[i];
        pixel[1] = ct_img_gc[i];
        pixel[2] = ct_img_bc[i];
    });
    img_buf
}