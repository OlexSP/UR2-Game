use rusty_engine::prelude::*;

mod constants;
mod game_state;
mod sprites;
mod logic;

use game_state::GameState;
use logic::logic::game_logic;
use sprites::{setup_cannon, setup_goal, setup_obstacles, setup_text};

fn main() {
    let mut game = Game::new();
    let game_state = GameState::default();

    setup_cannon(&mut game, game_state.rotation);
    setup_goal(&mut game);
    setup_obstacles(&mut game);
    setup_text(&mut game, game_state.magnitude.0, game_state.score);

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(game_state);
}

