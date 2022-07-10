use super::obstacle::Obstacle;

pub(super) struct Player {
    pub(super) x: i32,
    pub(super) y: i32,
    pub(super) velocity: f32,
}

impl Player {
    pub(super) fn at(x: i32, y: i32) -> Self {
        Self { x, y, velocity: 0. }
    }

    pub(super) fn flap(&mut self) {
        self.velocity = -2.0;
    }

    pub(super) fn hit_obstacle(&self, obstacle: &Obstacle) -> bool {
        let is_on_same_x_plane = self.x == obstacle.x;
        let half_of_gap_size = obstacle.size / 2;
        let player_is_above_gap = self.y < obstacle.gap_y - half_of_gap_size;
        let player_is_below_gap = self.y > obstacle.gap_y + half_of_gap_size;
        is_on_same_x_plane && (player_is_above_gap || player_is_below_gap)
    }

    pub(super) fn move_accounting_for_gravity(&mut self) {
        if self.velocity < 2. {
            self.velocity += 0.2
        }
        self.x += 1;
        self.y += self.velocity as i32;
        if self.y < 0 {
            self.y = 0
        }
    }
}
