#![allow(dead_code, unused_variables)]

use rusty_engine::prelude::*;
use rusty_engine::prelude::KeyCode::{Down, Left, Right, Up, A, D, S, W};

#[derive(Resource)]
struct GameState {
   high_score: u32,
   score: u32,
   ferris_index: i32,
   // enemy_labels: Vec<String>,
   // spawn_timer: Timer,
}

impl Default for GameState {
   fn default() -> Self {
      Self {
         high_score: 0,
         score: 0,
         ferris_index: 0,
         // enemy_labels: Vec::new(),
         // spawn_timer: Timer::from_seconds(1.0, TimerMode::Once),
      }
   }
}



fn main() {
   let mut game = Game::new();
   game.show_colliders = false;

   let game_state = GameState::default();

   // Add the player car
   let player = game.add_sprite("player", SpritePreset::RacingCarBlue);
   player.translation = Vec2::new(0.0, 0.0);
   player.rotation = SOUTH_WEST;
   player.scale = 1.0;
   player.layer = CHARACTER_LAYER;
   player.collision = true;

   // let car1 = game.add_sprite("car1", SpritePreset::RacingCarYellow);
   // car1.translation = Vec2::new(300.0, 0.0);
   // car1.layer = CHARACTER_LAYER;
   // car1.collision = true;

   // Add text plate
   let score = game.add_text("score", "Score: 0");
   score.translation = Vec2::new(520.0, 320.0);
   score.layer = UI_TOP_LAYER;

   let high_score = game.add_text("high_score", "High score: 0");
   high_score.translation = Vec2::new(-520.0, 320.0);
   high_score.layer = UI_TOP_LAYER;


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
         game_state.score += 1;
         // change the text plate
         let score_text = engine.texts.get_mut("score").unwrap();
         score_text.value = format!("Score: {}", game_state.score);
         if game_state.score > game_state.high_score {
            game_state.high_score = game_state.score;
            let high_score_text = engine.texts.get_mut("high_score").unwrap();
            high_score_text.value = format!("High score: {}", game_state.high_score);
         }
      }
   };
   // handle movements
   let player = engine.sprites.get_mut("player").unwrap();
   const MOVEMENT_SPEED: f32 = 200.0;
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

   // handle mouse input
   if engine.mouse_state.just_pressed(MouseButton::Left) {
      if let Some(mouse_location) = engine.mouse_state.location() {
         let label = format!("ferris_{}", game_state.ferris_index);
         game_state.ferris_index += 1;
         let ferris = engine.add_sprite(&label.clone(), SpritePreset::RacingConeStraight);
         ferris.translation = mouse_location;
         ferris.collision = true;
      }
   }

   // reset score
   if engine.keyboard_state.just_pressed(KeyCode::R) {
      game_state.score = 0;
      let score_text = engine.texts.get_mut("score").unwrap();
      score_text.value = format!("Score: {}", game_state.score);
   }

}