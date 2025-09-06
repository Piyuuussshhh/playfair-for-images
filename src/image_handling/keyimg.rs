use image::{DynamicImage, GenericImageView, ImageBuffer, Rgba};
use rand::Rng;

// PlayfairMatrix will hold the 16x16 matrix and a lookup table for the position of each pixel value in it for quick access.
#[derive(Debug)]
pub struct PlayfairMatrix {
    pub matrix: [[u8; 16]; 16],
    pub lookup: [(usize, usize); 256],
}

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

pub fn make_playfair_matrix(key_img: &DynamicImage) -> PlayfairMatrix {
    let mut matrix = [[-1i32; 16]; 16];
    let mut lookup = [(0usize, 0usize); 256];
    let mut seen = [false; 256];
    let mut idx = 0;
    key_img.pixels().for_each(|(_, _, pixel)| {
        let val = pixel[0].wrapping_add(pixel[1]).wrapping_add(pixel[2]) as i32;
        if !seen[val as usize] {
            matrix[idx / 16][idx % 16] = val;
            lookup[val as usize] = (idx / 16, idx % 16);
            seen[val as usize] = true;
            idx += 1;
        }
    });

    // Fill in remaining values
    (0..=255u8).for_each(|val| {
        if !seen[val as usize] {
            matrix[idx / 16][idx % 16] = val as i32;
            lookup[val as usize] = (idx / 16, idx % 16);
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

    PlayfairMatrix { matrix: res, lookup }
}
