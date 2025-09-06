pub fn opposite_pairing(idx: usize, channel: &[u8], switch: bool) -> ((u8, u8), (usize, usize)) {
    let len = channel.len();

    if switch {
        ((channel[idx], channel[len - 1 - idx]), (idx, len - 1 - idx))
    } else {
        ((channel[idx], channel[len - 1 - idx]), (len - 1 - idx, idx))
    }
}

pub fn halfway_pairing(idx: usize, channel: &[u8], switch: bool) -> ((u8, u8), (usize, usize)) {
    let len = channel.len();
    if switch {
        (
            (channel[idx], channel[(idx + len / 2) % len]),
            ((idx, (idx + len / 2) % len)),
        )
    } else {
        (
            (channel[idx], channel[(idx + len / 2) % len]),
            ((idx + len / 2) % len, idx),
        )
    }
}

pub fn sequential_pairing(idx: usize, channel: &[u8], switch: bool) -> ((u8, u8), (usize, usize)) {
    if switch {
        (
            (channel[2 * idx], channel[2 * idx + 1]),
            (2 * idx, 2 * idx + 1),
        )
    } else {
        (
            (channel[2 * idx], channel[2 * idx + 1]),
            (2 * idx + 1, 2 * idx),
        )
    }
}

pub fn random_pairing(idx: usize, channel: &[u8], switch: bool) -> ((u8, u8), (usize, usize)) {
    use rand::Rng;
    let mut rng = rand::rng();
    let len = channel.len();
    let first = channel[idx];
    let second_idx = rng.random_range(0..len);
    let second = channel[second_idx];
    if switch {
        ((first, second), (idx, second_idx))
    } else {
        ((first, second), (second_idx, idx))
    }
}
