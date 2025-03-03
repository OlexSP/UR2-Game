use rusty_engine::prelude::*;

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

   let player = game.add_sprite("my_player", SpritePreset::RacingCarBlue);
   player.translation = Vec2::new(200.0, 100.0); // Move the car up and to the right
   player.rotation = UP; // UP is one of the built-in constants you can use
   player.scale = 2.5; // It's a BIG car!
   player.layer = CHARACTER_LAYER; // as in previous code snippet


   // get your game stuff ready here
   let game_state = GameState::default();

   game.add_logic(game_logic); // Don't forget to add the logic function to the game!
   game.run(game_state);
}

const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 1.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
   // game_state.current_score += 1;
   // println!("Current score: {}", game_state.current_score);
}