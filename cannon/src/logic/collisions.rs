use rand::rng;
use rand::prelude::*;
use rusty_engine::audio::{AudioManager, SfxPreset};
use rusty_engine::game::Engine;
use rusty_engine::physics::CollisionEvent;
use rusty_engine::prelude::bevy::utils::HashMap;
use rusty_engine::prelude::{Sprite, Text, Vec2};
use rusty_engine::{LEFT, RIGHT};
use crate::game_state::GameState;

// handle_collisions() 2nd variant w/o memory allocation
pub fn handle_collisions(engine: &mut Engine, game_state: &mut GameState) {
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
    sprites.remove("ball");
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

// variant with additional memory allocation
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