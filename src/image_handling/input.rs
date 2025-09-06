use image::GenericImageView;
use std::thread;

use super::{encrypt, keyimg, pairing};

pub fn encrypt_and_save_output(path: &str, keyimg_path: Option<String>, output_path: &str) {
    // Get plaintext image.
    let pt_img = image::open(path).expect("Failed to open image");
    let (width, height) = pt_img.dimensions();
    println!("Opened image with dimensions {width} x {height}");

    // Get key image and generate playfair matrix.
    let key_image = if let Some(key_path) = keyimg_path {
        image::open(key_path).expect("Failed to open key image")
    } else {
        keyimg::generate_random_key_image()
    };
    let playfair_matrix = keyimg::make_playfair_matrix(&key_image);
    println!("Generated playfair matrix: {:?}", playfair_matrix.matrix);
    println!("Lookup table: {:?}", playfair_matrix.lookup);

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
    if (width * height) % 2 != 0 {
        // Add a black pixel value (0) for padding to make the length even.
        pt_img_rc.push(0);
        pt_img_gc.push(0);
        pt_img_bc.push(0);
    }
    // Prepare cipherimage RGB channels.
    let len = pt_img_rc.len();
    let mut ct_img_rc = vec![0u8; len];
    let mut ct_img_gc = vec![0u8; len];
    let mut ct_img_bc = vec![0u8; len];

    // Apply encryption to each channel separately, concurrently.
    thread::scope(|s| {
        s.spawn(|| {
            process_channel(
                &pt_img_rc,
                &mut ct_img_rc,
                &playfair_matrix,
                pairing::opposite_pairing,
            );
        });

        s.spawn(|| {
            process_channel(
                &pt_img_bc,
                &mut ct_img_bc,
                &playfair_matrix,
                pairing::sequential_pairing,
            );
        });

        s.spawn(|| {
            process_channel(
                &pt_img_gc,
                &mut ct_img_gc,
                &playfair_matrix,
                pairing::halfway_pairing,
            );
        });
    });

    // Assemble and save the cipherimage.
    let ct_img = assemble_cipherimg(width, height, &ct_img_rc, &ct_img_gc, &ct_img_bc);
    ct_img.save(output_path).expect("Failed to save image");
}

fn process_channel<F>(
    pt_channel: &[u8],
    ct_channel: &mut [u8],
    playfair_matrix: &keyimg::PlayfairMatrix,
    pairing_fn: F,
) where
    F: Fn(usize, &[u8], bool) -> ((u8, u8), (usize, usize)),
{
    encrypt::encrypt_channel(pt_channel, ct_channel, &playfair_matrix, pairing_fn);
    // Reverse the channel and add it to itself. This step makes the encryption non-decryptable, forever.
    let len = ct_channel.len();
    let half_len = len / 2;
    for i in 0..half_len {
        let p1 = ct_channel[i];
        let p2 = ct_channel[len - 1 - i];

        ct_channel[i] = p1.wrapping_add(p2);
        ct_channel[len - 1 - i] = p2.wrapping_add(p1);
    }
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
