use crate::constants::*;
use crate::game_state::GameState;
use rand::prelude::*;
use rand::rng;
use rusty_engine::prelude::*;
use rusty_engine::prelude::bevy::utils::HashMap;

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

// handle_collisions() variant w/o memory allocation
fn handle_collisions(engine: &mut Engine, game_state: &mut GameState) {
    // Iterate directly over the drain iterator
    for event in engine.collision_events.drain(..) {
        if !is_relevant_collision(&event) {
            continue;
        }
        for label in [event.pair.0, event.pair.1] {
            if label.starts_with("obstacle") {
                handle_obstacle_collision(&mut engine.sprites, &mut engine.audio_manager);
            }
            if label.starts_with("goal") {
                handle_goal_collision(
                    &mut engine.sprites,
                    &mut engine.audio_manager,
                    &mut engine.texts,
                    game_state,
                );
            }
        }
    }
}

fn is_relevant_collision(event: &CollisionEvent) -> bool {
    event.pair.either_contains("ball") &&
        !event.pair.either_contains("cannon") &&
        !event.state.is_end()
}

fn handle_obstacle_collision(sprites: &mut  HashMap<String, Sprite>, audio_manager: &mut AudioManager) {
    sprites.remove("ball");
    audio_manager.play_sfx(SfxPreset::Impact2, 0.5);
}

fn handle_goal_collision(
    sprites: &mut HashMap<String, Sprite>,
    audio_manager: &mut AudioManager,
    texts: &mut HashMap<String, Text>,
    game_state: &mut GameState,
) {
    game_state.score += 1;
    let score_text = texts.get_mut("score").unwrap();
    score_text.value = format!("Score {}", game_state.score);
    audio_manager.play_sfx(SfxPreset::Impact2, 0.5);
    reposition_sprites(sprites);
}

fn reposition_sprites(sprites: &mut HashMap<String, Sprite>) {
    for sprite in sprites.values_mut() {
        if sprite.label.starts_with("obstacle") {
            sprite.translation = Vec2::new(
                rng().random_range(-250.0..250.0),
                rng().random_range(-320.0..320.0),
            );
            sprite.rotation = rng().random_range(RIGHT..2.0 * LEFT);
        } else if sprite.label.starts_with("goal") {
            sprite.translation = Vec2::new(
                rng().random_range(250.0..600.0),
                rng().random_range(-330.0..300.0),
            );
        }
    }
}

// fn handle_collisions(engine: &mut Engine, game_state: &mut GameState) {
//     // Collect collision events into a Vec to release the mutable borrow early
//     let collision_events: Vec<CollisionEvent> = engine.collision_events.drain(..).collect();
//
//     for event in collision_events {
//         if !is_relevant_collision(&event) {
//             continue;
//         }
//         for label in [event.pair.0, event.pair.1] {
//             if label.starts_with("obstacle") {
//                 handle_obstacle_collision(engine);
//             }
//             if label.starts_with("goal") {
//                 handle_goal_collision(engine, game_state);
//             }
//         }
//     }
// }
//
//
// fn is_relevant_collision(event: &CollisionEvent) -> bool {
//     event.pair.either_contains("ball") &&
//         !event.pair.either_contains("cannon") &&
//         !event.state.is_end()
// }
//
// fn handle_obstacle_collision(engine: &mut Engine) {
//     engine.sprites.remove("ball");
//     engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.5);
// }
//
// fn handle_goal_collision(engine: &mut Engine, game_state: &mut GameState) {
//     game_state.score += 1;
//     let score_text = engine.texts.get_mut("score").unwrap();
//     score_text.value = format!("Score {}", game_state.score);
//     engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.5);
//     reposition_sprites(engine);
// }
//
// fn reposition_sprites(engine: &mut Engine) {
//     for sprite in engine.sprites.values_mut() {
//         if sprite.label.starts_with("obstacle") {
//             sprite.translation = Vec2::new(
//                 rng().random_range(-250.0..250.0),
//                 rng().random_range(-320.0..320.0),
//             );
//             sprite.rotation = rng().random_range(RIGHT..LEFT * 2.0);
//         } else if sprite.label.starts_with("goal") {
//             sprite.translation = Vec2::new(
//                 rng().random_range(250.0..600.0),
//                 rng().random_range(-330.0..300.0),
//             );
//         }
//     }
// }