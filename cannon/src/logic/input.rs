use rusty_engine::game::Engine;
use rusty_engine::keyboard::KeyCode;
use rusty_engine::{RIGHT, UP};
use rusty_engine::mouse::MouseButton;
use crate::constants::{MAGNITUDE_CHANGING_SPEED, ROTATION_SPEED};
use crate::game_state::GameState;

pub fn handle_input(engine: &mut Engine, game_state: &mut GameState) {
    // mouse drag control
    if engine.mouse_state.pressed(MouseButton::Left){
        game_state.magnitude.0 += engine.mouse_state.motion().x
            * MAGNITUDE_CHANGING_SPEED * engine.delta_f32;
        game_state.magnitude.1 = true;
    }

    if engine.mouse_state.pressed(MouseButton::Right){
        game_state.rotation += engine.mouse_state.motion().y
            * ROTATION_SPEED * engine.delta_f32;
    }


    // cannon rotation
    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        game_state.rotation += ROTATION_SPEED * engine.delta_f32;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        game_state.rotation -= ROTATION_SPEED * engine.delta_f32;
    }
    game_state.rotation = game_state.rotation.clamp(RIGHT, UP);

    // keyboard control
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
