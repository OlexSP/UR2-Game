use crate::game_state::GameState;
use crate::logic::collisions::{handle_collisions, reposition_sprites};
use crate::logic::input::handle_input;
use crate::logic::projectile::update_projectile;
use crate::sprites::setup_lives;
use rusty_engine::prelude::*;

pub fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if engine.sprites.get("ball").is_none() && (game_state.lives == 0 || game_state.attempts == 0) {
        if !game_state.game_over {
            game_over(engine);
            game_state.game_over = true;
        }
        if engine.keyboard_state.just_pressed(KeyCode::Y) {
            reset_game(engine, game_state);
            update_text_messages(engine,game_state);

        } else if engine.keyboard_state.just_pressed(KeyCode::N){
            engine.should_exit = true;
        }
        return;
    }

    handle_input(engine, game_state);
    update_cannon(engine, game_state);
    update_projectile(engine, game_state);
    handle_collisions(engine, game_state);
    update_text_messages(engine, game_state);
}

fn reset_game(engine: &mut Engine, game_state: &mut GameState){
    reset_game_state(game_state);
    engine.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);
    reposition_sprites(&mut engine.sprites);
    engine.texts.remove("game_over");
    reset_game_state(game_state);
    setup_lives(engine, &game_state.lives)

}
fn reset_game_state(game_state: &mut GameState) {
    *game_state = GameState::default()
}
fn update_text_messages(engine: &mut Engine, game_state: &mut GameState) {

    let magnitude_message = engine.texts.get_mut("magnitude").unwrap();
    magnitude_message.value = format!("Magnitude: {:.1}", game_state.magnitude);

    let slider = engine.sprites.get_mut("magnitude_slider").unwrap();
    slider.translation.x = -605.0 + game_state.magnitude * 6.0;

    let attempt_message = engine.texts.get_mut("attempts").unwrap();
    attempt_message.value = format!("Attempts: {}", game_state.attempts);

    let score_message = engine.texts.get_mut("score").unwrap();
    score_message.value = format!("Score: {}", game_state.score);
}

fn update_cannon(engine: &mut Engine, game_state: &mut GameState) {
    // cannon rotation logic
    let cannon = engine.sprites.get_mut("cannon").unwrap();
    cannon.rotation = game_state.rotation;
}

fn game_over(engine: &mut Engine) {
    let game_over = engine
        .add_text("game_over", "Game Over!\n Restart? (y/n)");
    game_over.font_size = 120.0;
    game_over.translation.y = 100.0;
    engine.audio_manager.stop_music();
    engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
}
