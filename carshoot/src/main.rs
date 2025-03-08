#![allow(dead_code, unused_variables)]

use rand::prelude::*;
use rusty_engine::prelude::*;

// constants
const MARBLE_SPEED: f32 = 600.0;
const PLAYER_SPEED: f32 = 300.0;
const CAR_SPEED: f32 = 250.0;

const BACKGROUND_LAYER: f32 = 0.0;
const CHARACTER_LAYER: f32 = 10.0;
const EFFECTS_LAYER: f32 = 2.0;
const UI_BOTTOM_LAYER: f32 = 3.0;
const UI_TOP_LAYER: f32 = 4.0;

#[derive(Resource)]
struct GameState {
    marble_labels: Vec<String>,
    cars_left: u32,
    score: u32,
    spawn_timer: Timer
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            marble_labels: vec!["marble1".into(), "marble2".into(), "marble3".into()],
            cars_left: 25,
            score: 0,
            spawn_timer: Timer::from_seconds(rand::rng().random_range(0.1..1.25), TimerMode::Once)
        }
    }
}


fn main() {
    let game_states = GameState::default();
    let mut shoot_game = Game::new();

    // set window settings
    shoot_game.window_settings(Window {
        title: "Car Shoot!".to_string(),
        ..Default::default()
    });

    // set ambient audio
    shoot_game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.12);

    // set text
    let score = shoot_game.
        add_text("score", format!("Score: {}", game_states.score));
    score.translation = Vec2::new(520.0, 320.0);
    let cars_left = shoot_game.
        add_text("cars_left", format!("Cars left: {}", game_states.cars_left));
    cars_left.translation = Vec2::new(-520.0, 320.0);

    // set player sprite
    let player = shoot_game.add_sprite("player", SpritePreset::RacingBarrierRed);
    player.translation = Vec2::new(0.0, -355.0);
    player.rotation = NORTH;
    player.scale = 0.5;
    player.layer = CHARACTER_LAYER;



    // game run
    shoot_game.add_logic(game_logic);
    shoot_game.run(game_states);

}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {

    // car sprites
    if game_state.spawn_timer.tick(engine.delta).just_finished(){
        game_state.spawn_timer = Timer::from_seconds(rand::rng().random_range(0.1..1.25), TimerMode::Once);
        if game_state.cars_left > 0 {
            game_state.cars_left -= 1;
            let cars_left = engine.texts.get_mut("cars_left").unwrap();
            cars_left.value = format!("Cars left: {}", game_state.cars_left);
            let label = format!("car{}", game_state.cars_left);

            let car_choices = vec![
                SpritePreset::RacingCarBlack,
                SpritePreset::RacingCarBlue,
                SpritePreset::RacingCarGreen,
                SpritePreset::RacingCarRed,
                SpritePreset::RacingCarYellow
            ];
            let car_preset = car_choices.iter().choose(&mut rand::rng()).unwrap().clone();
            let car = engine.add_sprite(label, car_preset);
            car.translation.x = -740.0;
            car.translation.y = rand::rng().random_range(-100.0..325.0);
            car.collision = true;
        }
    }

    // player movement
    let player = engine.sprites.get_mut("player").unwrap();
    if let Some(mouse_location) = engine.mouse_state.location() {
        player.translation.x = mouse_location.x;
    }
    let player_x = player.translation.x;

    // fire a marble
    if engine.mouse_state.just_pressed(MouseButton::Left) {
        if let Some(label) = game_state.marble_labels.pop() {
            let marble_sprite = engine.
                add_sprite(&label, SpritePreset::RollingBallBlue );
            marble_sprite.translation = Vec2::new(player_x, -275.0);
            marble_sprite.layer = UI_TOP_LAYER;
            marble_sprite.collision = true;
            engine.audio_manager.play_sfx(SfxPreset::Impact2, 0.4);
        }
    }

    // sprites movement
    let mut labels_to_delete = vec![];
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("marble") {
            sprite.translation.y += MARBLE_SPEED * engine.delta_f32;
        }
        if sprite.label.starts_with("car") {
            sprite.translation.x += CAR_SPEED * engine.delta_f32;
        }

        if sprite.translation.y > 400.0 || sprite.translation.x > 750.0 {
            labels_to_delete.push(sprite.label.clone())
        }
    }

    // remove  sprites
    for label in labels_to_delete{
        engine.sprites.remove(&label);
        if label.starts_with("marble") {
            game_state.marble_labels.push(label)
        }
    }
}