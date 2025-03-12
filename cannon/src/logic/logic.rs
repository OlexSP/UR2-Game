use crate::constants::*;
use crate::game_state::GameState;

use crate::logic::collisions::handle_collisions;
use crate::logic::projectile::update_projectile;
use rusty_engine::prelude::*;

pub fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    update_text_messages(engine, game_state);
    handle_input(engine, game_state);
    update_cannon(engine, game_state);
    update_projectile(engine, game_state);
    handle_collisions(engine, game_state);
}

fn update_text_messages(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.magnitude.1 {
        let magnitude_message = engine.texts.get_mut("magnitude").unwrap();
        magnitude_message.value = format!("Magnitude: {:.1}", game_state.magnitude.0);
        game_state.magnitude.1 = false;
    }
}
fn handle_input(engine: &mut Engine, game_state: &mut GameState) {
    // keyboard input logic
    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        game_state.rotation += ROTATION_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        game_state.rotation -= ROTATION_SPEED * engine.delta_f32;
    }
    game_state.rotation = game_state.rotation.clamp(RIGHT, UP);

    if engine.keyboard_state.pressed_any(&[KeyCode::Left, KeyCode::A]) {
        game_state.magnitude.0 -= MAGNITUDE_CHANGING_SPEED * engine.delta_f32;
        game_state.magnitude.1 = true;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Right, KeyCode::D]) {
        game_state.magnitude.0 += MAGNITUDE_CHANGING_SPEED * engine.delta_f32;
        game_state.magnitude.1 = true;
    }
    game_state.magnitude.0 = game_state.magnitude.0.clamp(0.0, 25.0);
}

fn update_cannon(engine: &mut Engine, game_state: &mut GameState) {
    // cannon rotation logic
    let cannon = engine.sprites.get_mut("cannon").unwrap();
    cannon.rotation = game_state.rotation;
}



