use crate::constants::*;
use crate::game_state::GameState;

use crate::logic::collisions::handle_collisions;
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

fn update_projectile(engine: &mut Engine, game_state: &mut GameState){
    //projectile movement logic

    let c_translation = engine.sprites.get_mut("cannon").unwrap().translation;
    let c_rotation = engine.sprites.get_mut("cannon").unwrap().rotation;

    // cannon bal movement
    if let Some(canon_ball) = engine.sprites.get_mut("ball") {
        canon_ball.translation.x += game_state.ball_velocity.x * engine.delta_f32;
        canon_ball.translation.y += game_state.ball_velocity.y * engine.delta_f32;

        game_state.ball_velocity.y -= GRAVITY_ACCELERATION * engine.delta_f32;
        game_state.ball_velocity.y = match game_state.ball_velocity.y {
            y if y > 0.0 => game_state.ball_velocity.y - AIR_RESISTANCE * engine.delta_f32,
            y if y < 0.0 => game_state.ball_velocity.y + AIR_RESISTANCE * engine.delta_f32,
            _ => game_state.ball_velocity.y,
        };

        game_state.ball_velocity.x -= AIR_RESISTANCE * engine.delta_f32;
        game_state.ball_velocity.x = game_state.ball_velocity.x.clamp(0.0, 1000.0);

        if canon_ball.translation.x > 750.0
            || canon_ball.translation.y > 1000.0
            || canon_ball.translation.y < -400.0
        {
            engine.sprites.remove("ball");
        }
    } else if engine.keyboard_state.just_pressed(KeyCode::Space)
        || engine.mouse_state.just_pressed(MouseButton::Left)
    {
        let cannon_ball = engine.add_sprite("ball", SpritePreset::RollingBallRed);
        cannon_ball.translation = c_translation;
        cannon_ball.rotation = c_rotation;
        cannon_ball.layer = BALL_LAYER;
        cannon_ball.collision = true;

        game_state.ball_velocity = Vec2::new(
            game_state.magnitude.0 * MAGNITUDE_MULTIPLIER * c_rotation.cos(),
            game_state.magnitude.0 * MAGNITUDE_MULTIPLIER * c_rotation.sin(),
        );

        engine.audio_manager.play_sfx(SfxPreset::Click, 0.2);
    }
}

