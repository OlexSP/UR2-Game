#![allow(dead_code, unused_variables)]
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health_amount: 5,
            lost: false,
        }
    }
}

const PLAYER_SPEED: f32 = 250.0;

fn main() {
    let mut game = Game::new();
    let  game_state = GameState::default();

    // music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // add sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation = Vec2::new(-500.0, 0.0);
    player1.layer = 10.0;
    player1.collision = true;

    // text plates
    let health = game.add_text("health", format!("Health: {}", game_state.health_amount));
    health.translation = Vec2::new(520.0, 320.0);
    health.layer = 20.0;


    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut direction: f32 = 0.0;
    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        direction -= 1.0;
    }

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    if player1.translation.y > 360.0 || player1.translation.y < -360.0 {
        game_state.health_amount = 0;
    }

    // health text plate
    let health = engine.texts.get_mut("health").unwrap();
    health.value = format!("Health: {}", game_state.health_amount);
    health.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    health.translation.y = engine.window_dimensions.y / 2.0 - 30.0

}