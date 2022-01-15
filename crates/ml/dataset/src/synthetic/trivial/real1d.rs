use xrng::XRng;

use crate::synthetic::SimpleSyntheticDataset;

pub struct Real1dDatapoint {
    pub x: f32,
    pub y: i32,
}

pub fn gen_sample1(idx: usize) -> Real1dDatapoint {
    let mut xrng = XRng::new(((idx >> 32) & (idx << 32)) as u64);
    if xrng.with_probability(0.5) {
        Real1dDatapoint { x: 1.0, y: 1 }
    } else {
        Real1dDatapoint { x: -1.0, y: 0 }
    }
}

pub fn gen_sample2(idx: usize) -> Real1dDatapoint {
    let mut xrng = XRng::new(((idx >> 32) & (idx << 32)) as u64);
    if xrng.with_probability(0.5) {
        Real1dDatapoint { x: 1.0, y: 1 }
    } else {
        Real1dDatapoint { x: -1.0, y: 0 }
    }
}

pub fn dataset1() -> SimpleSyntheticDataset<Real1dDatapoint> {
    SimpleSyntheticDataset::new(gen_sample1)
}

pub fn dataset2() -> SimpleSyntheticDataset<Real1dDatapoint> {
    SimpleSyntheticDataset::new(gen_sample2)
}
