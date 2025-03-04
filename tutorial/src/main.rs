#![allow(dead_code, unused_variables)]

use rusty_engine::prelude::*;
use rusty_engine::prelude::KeyCode::{Down, Left, Right, Up, A, D, S, W};

#[derive(Resource)]
struct GameState {
   high_score: u32,
   current_score: u32,
   enemy_labels: Vec<String>,
   spawn_timer: Timer,
}

impl Default for GameState {
   fn default() -> Self {
      Self {
         high_score: 0,
         current_score: 0,
         enemy_labels: Vec::new(),
         spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
      }
   }
}



fn main() {
   let mut game = Game::new();
   game.show_colliders = true;

   let game_state = GameState::default();

   let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
   player.translation = Vec2::new(0.0, 0.0);
   player.rotation = SOUTH_WEST;
   player.scale = 1.0;
   player.layer = CHARACTER_LAYER;
   player.collision = true;

   let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
   car1.translation = Vec2::new(300.0, 0.0);
   car1.layer = CHARACTER_LAYER;
   car1.collision = true;

   game.add_logic(game_logic); // Don't forget to add the logic function to the game!
   game.run(game_state);
}

// Layers
const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
   // handle collisions
   for event in engine.collision_events.drain(..) {
      if event.state == CollisionState::Begin && event.pair.one_starts_with("player"){
         // remove the sprite the player is colliding with
         for label in [event.pair.0, event.pair.1] {
             if label != "player" {
                 engine.sprites.remove(&label);
             }
         }
          game_state.current_score += 1;
          println!("Current score: {}", game_state.current_score);
      }
   };
   // handle movements
   let player = engine.sprites.get_mut("player").unwrap();
   const MOVEMENT_SPEED: f32 = 100.0;
   if engine.keyboard_state.pressed_any(&[Up, W]) {
      player.translation.y += MOVEMENT_SPEED * engine.delta_f32;
   }
   if engine.keyboard_state.pressed_any(&[Down, S]) {
      player.translation.y -= MOVEMENT_SPEED * engine.delta_f32;
   }
   if engine.keyboard_state.pressed_any(&[Left, A]) {
      player.translation.x -= MOVEMENT_SPEED * engine.delta_f32;
   }
   if engine.keyboard_state.pressed_any(&[Right, D]) {
      player.translation.x += MOVEMENT_SPEED * engine.delta_f32;
   }

}