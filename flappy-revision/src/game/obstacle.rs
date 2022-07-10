use bracket_lib::prelude::RandomNumberGenerator;

pub(super) struct Obstacle {
    pub(super) x: i32,
    pub(super) gap_y: i32,
    pub(super) size: i32,
}

impl Obstacle {
    pub(super) fn at(x: i32, score: i32) -> Self {
        let mut random = RandomNumberGenerator::new();
        Self {
            x,
            gap_y: random.range(10, 40),
            size: i32::max(2, 20 - score),
        }
    }
}
