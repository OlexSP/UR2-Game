use rusty_engine::prelude::*;

mod constants;
mod game_state;
mod sprites;
mod logic;

use crate::sprites::setup_sprites;
use game_state::GameState;
use logic::logic::game_logic;

fn main() {
    let mut game = Game::new();
    let game_state = GameState::default();

    setup_sprites(&mut game, &game_state);

    game.audio_manager
        .play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    game.add_logic(game_logic);
    game.run(game_state);
}

