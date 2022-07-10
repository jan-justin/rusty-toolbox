mod game;

use bracket_lib::prelude::{main_loop, BError, BTermBuilder};
use game::Game;

fn main() -> BError {
    let world = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        .build()?;
    main_loop(world, Game::new())
}
