use super::{mode::GameMode, obstacle::Obstacle, state::State, world::*};
use bracket_lib::prelude::*;

pub struct Game {
    state: State,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: State::new(),
        }
    }

    fn with_world<'a>(&'a mut self, world: &'a mut BTerm) -> GameWithWorld {
        GameWithWorld {
            world,
            state: &mut self.state,
        }
    }
}

impl GameState for Game {
    fn tick(&mut self, world: &mut BTerm) {
        self.with_world(world).advance_one_tick();
    }
}

struct GameWithWorld<'a> {
    state: &'a mut State,
    world: &'a mut BTerm,
}

impl<'a> GameWithWorld<'a> {
    fn advance_one_tick(&mut self) {
        match self.state.mode {
            GameMode::End => self.game_over(),
            GameMode::Menu => self.main_menu(),
            GameMode::Playing => self.play(),
        };
    }

    fn main_menu(&mut self) {
        self.show_welcome_banner();
        self.handle_user_menu_input();
    }

    fn show_welcome_banner(&mut self) {
        let GameWithWorld { world, .. } = self;
        world.cls();
        world.print_centered(5, "Welcome to Flappy Dragon");
        world.print_centered(8, "(P) Play Game");
        world.print_centered(9, "(Q) Quit Game");
    }

    fn handle_user_menu_input(&mut self) {
        let GameWithWorld { state, world } = self;
        if let Some(key) = world.key {
            match key {
                VirtualKeyCode::P => state.reset(),
                VirtualKeyCode::Q => world.quitting = true,
                _ => {}
            }
        }
    }

    fn game_over(&mut self) {
        self.show_death_banner();
        self.handle_user_menu_input();
    }

    fn show_death_banner(&mut self) {
        let GameWithWorld { state, world } = self;
        world.cls();
        world.print_centered(5, "You are dead");
        world.print_centered(6, format!("Your score: {}", state.score));
        world.print_centered(8, "(P) Play Game");
        world.print_centered(9, "(Q) Quit Game");
    }

    fn play(&mut self) {
        self.render_fixed_elements();
        self.render_moving_elements();
        self.update_frame_time();
        self.handle_player_player_input_if_any();
        self.handle_player_obstacle_collision_if_any();
        self.render_new_obstacle_if_player_passed();
    }

    fn render_fixed_elements(&mut self) {
        let GameWithWorld { state, world } = self;
        world.cls_bg(NAVY);
        world.print(0, 0, "Press SPACE to flap.");
        world.print(0, 1, format!("Score: {}", state.score));
    }

    fn render_moving_elements(&mut self) {
        self.render_player();
        self.render_obstacle();
    }

    fn render_player(&mut self) {
        let GameWithWorld { state, world } = self;
        world.set(0, state.player.y, YELLOW, BLACK, to_cp437('@'));
    }

    fn render_obstacle(&mut self) {
        let GameWithWorld { state, world } = self;
        let State {
            player, obstacle, ..
        } = state;
        let screen_x = obstacle.x - player.x;
        let gap_outer_limit = obstacle.size / 2;
        for y in 0..obstacle.gap_y - gap_outer_limit {
            world.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
        for y in obstacle.gap_y + gap_outer_limit..SCREEN_HEIGHT {
            world.set(screen_x, y, RED, BLACK, to_cp437('|'));
        }
    }

    fn update_frame_time(&mut self) {
        let GameWithWorld { state, world } = self;
        state.frame_time += world.frame_time_ms;
        if state.frame_time > FRAME_DURATION {
            state.frame_time = 0.;
            state.player.move_accounting_for_gravity();
        }
    }

    fn handle_player_player_input_if_any(&mut self) {
        let GameWithWorld { state, world } = self;
        if let Some(VirtualKeyCode::Space) = world.key {
            state.player.flap();
        }
    }

    fn handle_player_obstacle_collision_if_any(&mut self) {
        let GameWithWorld { state, .. } = self;
        let State {
            player, obstacle, ..
        } = state;
        let player_hit_the_ground = player.y > SCREEN_HEIGHT;
        let player_hit_obstacle = player.hit_obstacle(obstacle);
        if player_hit_the_ground || player_hit_obstacle {
            state.mode = GameMode::End;
        }
    }

    fn render_new_obstacle_if_player_passed(&mut self) {
        let GameWithWorld { state, .. } = self;
        let State {
            player, obstacle, ..
        } = state;
        if player.x > obstacle.x {
            state.score += 1;
            state.obstacle = Obstacle::at(player.x + SCREEN_WIDTH, state.score);
        }
    }
}
