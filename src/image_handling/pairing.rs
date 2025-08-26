pub fn opposite_pairing(idx: usize, channel: &[u8]) -> ((u8, u8), (usize, usize)) {
    let len = channel.len();

    ((channel[idx], channel[len - 1 - idx]), (idx, len - 1 - idx))
}

pub fn halfway_pairing(idx: usize, channel: &[u8]) -> ((u8, u8), (usize, usize)) {
    let len = channel.len();
    (
        (channel[idx], channel[(idx + len / 2) % len]),
        (idx, (idx + len / 2) % len),
    )
}

pub fn sequential_pairing(idx: usize, channel: &[u8]) -> ((u8, u8), (usize, usize)) {
    let len = channel.len();
    if idx < len / 2 {
        (
            (channel[2 * idx], channel[2 * idx + 1]),
            (2 * idx, 2 * idx + 1),
        )
    } else {
        let offset = idx - len / 2;
        (
            (channel[idx + 2 * offset], channel[idx + 2 * offset + 1]),
            (idx + 2 * offset, idx + 2 * offset + 1),
        )
    }
}

pub fn random_pairing(idx: usize, channel: &[u8]) -> ((u8, u8), (usize, usize)) {
    use rand::Rng;
    let mut rng = rand::rng();
    let len = channel.len();
    let first = channel[2 * idx];
    let second_idx = rng.random_range(0..len);
    let second = channel[second_idx];
    ((first, second), (idx, len - 1 - idx))
}
