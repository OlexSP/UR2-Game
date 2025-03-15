use rusty_engine::prelude::*;
use std::env;

mod constants;
mod game_state;
mod sprites;
mod logic;

use game_state::GameState;
use logic::logic::game_logic;
use sprites::setup_sprites;

fn main() {
    let exe_path = env::current_exe().expect("Failed to get executable path");
    let exe_dir = exe_path.parent().expect("Failed to get executable directory");

    env::set_current_dir(&exe_dir).expect("Failed to set working directory");


    let mut game = Game::new();
    let game_state = GameState::default();

    setup_sprites(&mut game, &game_state);

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(game_state);
}

