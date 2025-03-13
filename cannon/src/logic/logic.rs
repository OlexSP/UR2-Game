use crate::game_state::GameState;
use crate::logic::collisions::handle_collisions;
use crate::logic::input::handle_input;
use crate::logic::projectile::update_projectile;
use rusty_engine::prelude::*;

pub fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    update_text_messages(engine, game_state);
    handle_input(engine, game_state);
    update_cannon(engine, game_state);
    update_projectile(engine, game_state);
    handle_collisions(engine, game_state);
    //game_over(engine, game_state);
}

fn update_text_messages(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.magnitude.1 {
        let magnitude_message = engine.texts.get_mut("magnitude").unwrap();
        magnitude_message.value = format!("Magnitude: {:.1}", game_state.magnitude.0);
        game_state.magnitude.1 = false;

        let slider = engine.sprites.get_mut("magnitude_slider").unwrap();
        slider.translation.x = -605.0 + game_state.magnitude.0 * 6.0;
    }

    let attempt_message = engine.texts.get_mut("attempts").unwrap();
    attempt_message.value = format!("Attempts: {}", game_state.attempts)
}

fn update_cannon(engine: &mut Engine, game_state: &mut GameState) {
    // cannon rotation logic
    let cannon = engine.sprites.get_mut("cannon").unwrap();
    cannon.rotation = game_state.rotation;
}

// fn game_over(engine: &mut Engine, game_state: &mut GameState) {
//     if (game_state.lives == 0 || game_state.attempts == 0) &&
//
// }


