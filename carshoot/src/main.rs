#![allow(dead_code, unused_variables)]
use rusty_engine::prelude::*;

const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;

#[derive(Resource)]
struct GameState {
    health: f32,
    timer: Timer
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health: 100.0,
            timer: Timer::from_seconds(1.0, TimerMode::Repeating)
        }
    }
}


fn main() {
    let game_states = GameState::default();
    let mut shoot_game = Game::new();

    // set window settings
    shoot_game.window_settings(Window {
        title: "Car Shoot!".to_string(),
        ..Default::default()
    });

    // set ambient audio
    shoot_game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.12);

    // add player sprite
    let player = shoot_game.add_sprite("player", SpritePreset::RacingCarBlue);
    player.translation = Vec2::new(0.0, 0.0);
    player.rotation = SOUTH_WEST;


    // game run
    shoot_game.add_logic(game_logic);
    shoot_game.run(game_states);

}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

}