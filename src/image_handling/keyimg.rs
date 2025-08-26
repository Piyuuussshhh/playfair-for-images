use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rand::Rng;

pub fn generate_random_key_image() -> DynamicImage {
    let mut rng = rand::rng();

    let img = ImageBuffer::from_fn(16, 16, |_, _| {
        Rgba([
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            rng.random_range(0..=255),
            255,
        ])
    });
    DynamicImage::ImageRgba8(img)
}

pub fn make_playfair_matrix(key_img: &DynamicImage) -> [[u8; 16]; 16] {
    let mut matrix = [[-1i32; 16]; 16];
    let mut seen = [false; 256];
    let mut idx = 0;
    key_img.pixels().for_each(|(_, _, pixel)| {
        let val = pixel[0].wrapping_add(pixel[1]).wrapping_add(pixel[2]) as i32;
        println!(
            "Pixel at ({}, {}) has value {}. It has {} seen.",
            idx / 16,
            idx % 16,
            val,
            if seen[val as usize] {
                "BEEN"
            } else {
                "NOT BEEN"
            }
        );
        if !seen[val as usize] {
            matrix[idx / 16][idx % 16] = val;
            seen[val as usize] = true;
            idx += 1;
        }
    });

    // Fill in remaining values
    (0..=255u8).for_each(|val| {
        if !seen[val as usize] {
            matrix[idx / 16][idx % 16] = val as i32;
            seen[val as usize] = true;
            idx += 1;
        }
    });

    let mut res = [[0u8; 16]; 16];

    for i in 0..16 {
        for j in 0..16 {
            res[i][j] = matrix[i][j] as u8;
        }
    }

    res
}
