pub fn encrypt_channel<F>(
    pt_channel: &[u8],
    ct_channel: &mut [u8],
    key_matrix: &[[u8; 16]; 16],
    paring_fn: F,
    is_first_half: bool,
) where
    F : Fn(usize, &[u8]) -> ((u8, u8), (usize, usize))
{
    let len = pt_channel.len();
    assert_eq!(len % 2, 0, "Channel length must be even");

    let range = if is_first_half { 0..(len/2) } else { (len/2)..len };
    for i in range {
        // Get the pair of pixels to encrypt.
        let ((p1, p2), (idx1, idx2)) = paring_fn(i, &pt_channel);
        let (row1, col1) = find_position(p1, key_matrix);
        let (row2, col2) = find_position(p2, key_matrix);

        // Apply Playfair cipher rules.
        if p1 == p2 {
            // Same pixel: let first pixel be unchanged, invert second pixel
            // let p2 = if p2 == 0 { 255 } else { 0 };
            let p2 = p2.wrapping_add(128);
            let (new_b_row, new_b_col) = find_position(p2, key_matrix);
            if row1 == new_b_row {
                // Same row: shift right
                ct_channel[idx1] = key_matrix[row1][(col1 + 1) % 16];
                ct_channel[idx2] = key_matrix[new_b_row][(new_b_col + 1) % 16];
            } else if col1 == new_b_col {
                // Same column: shift down
                ct_channel[idx1] = key_matrix[(row1 + 1) % 16][col1];
                ct_channel[idx2] = key_matrix[(new_b_row + 1) % 16][new_b_col];
            } else {
                // Rectangle swap
                ct_channel[idx1] = key_matrix[row1][new_b_col];
                ct_channel[idx2] = key_matrix[new_b_row][col1];
            }
        } else if row1 == row2 {
            // Same row: shift right
            ct_channel[idx1] = key_matrix[row1][(col1 + 1) % 16];
            ct_channel[idx2] = key_matrix[row2][(col2 + 1) % 16];
        } else if col1 == col2 {
            // Same column: shift down
            ct_channel[idx1] = key_matrix[(row1 + 1) % 16][col1];
            ct_channel[idx2] = key_matrix[(row2 + 1) % 16][col2];
        } else {
            // Rectangle swap
            ct_channel[idx1] = key_matrix[row1][col2];
            ct_channel[idx2] = key_matrix[row2][col1];
        }
    }
}

fn find_position(value: u8, matrix: &[[u8; 16]; 16]) -> (usize, usize) {
    for (i, row) in matrix.iter().enumerate() {
        for (j, &val) in row.iter().enumerate() {
            if val == value {
                return (i, j);
            }
        }
    }
    panic!("Value not found in matrix");
}