use super::keyimg;

pub fn encrypt_channel<F>(
    pt_channel: &[u8],
    ct_channel: &mut [u8],
    playfair_grid: &keyimg::PlayfairMatrix,
    pairing_fn: F,
) where
    F: Fn(usize, &[u8], bool) -> ((u8, u8), (usize, usize)),
{
    let len = pt_channel.len();
    assert_eq!(len % 2, 0, "Channel length must be even");

    let mut switch = false;
    let mut counter = 0;

    for i in 0..(len / 2) {
        // Get the pair of pixels to encrypt.
        let ((p1, p2), (idx1, idx2)) = pairing_fn(i, &pt_channel, switch);
        counter += 1;
        if counter == 10 {
            switch = !switch;
            counter = 0;
        }
        let (row1, col1) = playfair_grid.lookup[p1 as usize];
        let (row2, col2) = playfair_grid.lookup[p2 as usize];

        // Apply Playfair cipher rules.
        if p1 == p2 {
            let p2 = p2.wrapping_add(128);
            let (new_b_row, new_b_col) = playfair_grid.lookup[p2 as usize];
            if row1 == new_b_row {
                // Same row: shift right
                ct_channel[idx1] = playfair_grid.matrix[row1][(col1 + 1) % 16];
                ct_channel[idx2] = playfair_grid.matrix[new_b_row][(new_b_col + 1) % 16];
            } else if col1 == new_b_col {
                // Same column: shift down
                ct_channel[idx1] = playfair_grid.matrix[(row1 + 1) % 16][col1];
                ct_channel[idx2] = playfair_grid.matrix[(new_b_row + 1) % 16][new_b_col];
            } else {
                // Rectangle swap
                ct_channel[idx1] = playfair_grid.matrix[row1][new_b_col];
                ct_channel[idx2] = playfair_grid.matrix[new_b_row][col1];
            }
        } else if row1 == row2 {
            // Same row: shift right
            ct_channel[idx1] = playfair_grid.matrix[row1][(col1 + 1) % 16];
            ct_channel[idx2] = playfair_grid.matrix[row2][(col2 + 1) % 16];
        } else if col1 == col2 {
            // Same column: shift down
            ct_channel[idx1] = playfair_grid.matrix[(row1 + 1) % 16][col1];
            ct_channel[idx2] = playfair_grid.matrix[(row2 + 1) % 16][col2];
        } else {
            // Rectangle swap
            ct_channel[idx1] = playfair_grid.matrix[row1][col2];
            ct_channel[idx2] = playfair_grid.matrix[row2][col1];
        }
    }
}