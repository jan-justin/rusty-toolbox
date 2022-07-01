use super::{mode::GameMode, obstacle::Obstacle, player::Player, world::SCREEN_WIDTH};

pub(super) struct State {
    pub(super) mode: GameMode,
    pub(super) player: Player,
    pub(super) frame_time: f32,
    pub(super) obstacle: Obstacle,
    pub(super) score: i32,
}

impl State {
    pub(super) fn new() -> Self {
        Self {
            score: 0,
            frame_time: 0.0,
            mode: GameMode::Menu,
            player: Player::at(5, 25),
            obstacle: Obstacle::at(SCREEN_WIDTH, 0),
        }
    }

    pub(super) fn reset(&mut self) {
        self.score = 0;
        self.frame_time = 0.0;
        self.mode = GameMode::Playing;
        self.player = Player::at(5, 25);
        self.obstacle = Obstacle::at(SCREEN_WIDTH, 0);
    }
}
