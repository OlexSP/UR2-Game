use rand::prelude::*;
use rusty_engine::prelude::*;
use std::f64::consts::PI;
use rand::rng;
use crate::constants::*;
use crate::game_state::GameState;

pub fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    // text messages
    let magnitude_message = engine.texts.get_mut("magnitude").unwrap();

    // keyboard input
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        game_state.rotation += ROTATION_SPEED * engine.delta_f32;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        game_state.rotation -= ROTATION_SPEED * engine.delta_f32;
    }
    game_state.rotation = game_state.rotation.clamp(RIGHT, UP);

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Left, KeyCode::A])
    {
        game_state.magnitude -= MAGNITUDE_CHANGING_SPEED * engine.delta_f32;
        magnitude_message.value = format!("Magnitude: {:.1}", game_state.magnitude);
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Right, KeyCode::D])
    {
        game_state.magnitude += MAGNITUDE_CHANGING_SPEED * engine.delta_f32;
        magnitude_message.value = format!("Magnitude: {:.1}", game_state.magnitude);
    }
    game_state.magnitude = game_state.magnitude.clamp(0.0, 25.0);

    // cannon rotation movement
    let cannon = engine.sprites.get_mut("cannon").unwrap();
    cannon.rotation = game_state.rotation;
    let c_translation = cannon.translation;
    let c_rotation = cannon.rotation;

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
            game_state.magnitude * MAGNITUDE_MULTIPLIER * c_rotation.cos(),
            game_state.magnitude * MAGNITUDE_MULTIPLIER * c_rotation.sin(),
        );

        engine.audio_manager.play_sfx(SfxPreset::Click, 0.2);
    }

    // handle collisions
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("ball")
            || event.pair.either_contains("cannon")
            || event.state.is_end()
        {
            continue;
        }
        for label in [event.pair.0, event.pair.1] {
            // remove the ball if it hits an obstacle
            if label.starts_with("obstacle") {
                engine.sprites.remove("ball");
                engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.5);
            }
            if label.starts_with("goal") {
                game_state.score += 1;
                let score_text = engine.texts.get_mut("score").unwrap();
                score_text.value = format!("Score {}", game_state.score);
                engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.5);

                // new goal and obstacles translations
                for sprite in engine.sprites.values_mut() {
                    if sprite.label.starts_with("obstacle") {
                        sprite.translation = Vec2::new(
                            rng().random_range(-250.0..250.0),
                            rng().random_range(-320.0..320.0),
                        );
                        sprite.rotation = rng().random_range(0.0..2.0 * PI as f32);
                    } else if sprite.label.starts_with("goal") {
                        sprite.translation = Vec2::new(
                            rng().random_range(250.0..600.0),
                            rng().random_range(-330.0..300.0),
                        );
                    }
                }
            }
        }
    }
}