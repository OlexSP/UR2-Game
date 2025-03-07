#![allow(dead_code, unused_variables)]

use rusty_engine::prelude::*;

const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 10.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;

#[derive(Resource)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: u32,
    score: u32,
    spawn_timer: Timer
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
            cars_left: 25,
            score: 0,
            spawn_timer: Timer::from_seconds(0.0, TimerMode::Once)
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

    // set text
    let score = shoot_game.
        add_text("score", format!("Score: {}", game_states.score));
    score.translation = Vec2::new(520.0, 320.0);
    let cars_left = shoot_game.
        add_text("cars_left", format!("Cars left: {}", game_states.cars_left));
    cars_left.translation = Vec2::new(-520.0, 320.0);

    // set player sprite
    let player = shoot_game.add_sprite("player", SpritePreset::RacingBarrierRed);
    player.translation = Vec2::new(0.0, -355.0);
    player.rotation = NORTH;
    player.scale = 0.5;
    player.layer = CHARACTER_LAYER;


    // game run
    shoot_game.add_logic(game_logic);
    shoot_game.run(game_states);

}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

}