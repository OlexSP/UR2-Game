
use rusty_engine::prelude::*;
use rand::prelude::*;
use rand::rng;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            health_amount: 5,
            lost: false,
        }
    }
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 500.0;

fn main() {
    let mut game = Game::new();
    let  game_state = GameState::default();

    // music
    game.audio_manager.play_music(MusicPreset::WhimsicalPopsicle, 0.2);

    // add player sprite
    let player1 = game.add_sprite("player1", SpritePreset::RacingCarBlue);
    player1.translation = Vec2::new(-500.0, 0.0);
    player1.layer = 10.0;
    player1.collision = true;

    // add road lines
    for i in 0..10 {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.2;
        roadline.translation.x = -600.0 + i as f32 * 150.0;
    }

    // add road obstacles
    let obstacle_presets = vec![
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingConeStraight,
        SpritePreset::RacingConeStraight,
        SpritePreset::RollingBlockCorner,
        SpritePreset::RollingBlockSquare,
        SpritePreset::RollingBlockSmall,
    ];

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.translation.x = rng().random_range(800.0..1600.0);
        obstacle.translation.y = rng().random_range(-300.0..300.0);
        obstacle.collision = true;
    }

    // text plates
    let health_message = game.add_text("health_message", format!("Health: {}", game_state.health_amount));
    health_message.translation = Vec2::new(550.0, 320.0);
    health_message.layer = 20.0;


    game.add_logic(game_logic);
    game.run(game_state);
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost {
        return;
    }

    let mut direction: f32 = 0.0;
    if engine.keyboard_state.pressed_any(&[KeyCode::Up, KeyCode::W]) {
        direction += 1.0;
    }
    if engine.keyboard_state.pressed_any(&[KeyCode::Down, KeyCode::S]) {
        direction -= 1.0;
    }

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15;

    if player1.translation.y > 360.0 || player1.translation.y < -360.0 {
        game_state.health_amount = 0;
    }

    // move road objects
    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
            }
        }
        if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = rng().random_range(800.0..1600.0);
                sprite.translation.y = rng().random_range(-300.0..300.0);
            }
        }
    }

    let health_message = engine.texts.get_mut("health_message").unwrap();
    health_message.translation.x = engine.window_dimensions.x / 2.0 - 80.0;
    health_message.translation.y = engine.window_dimensions.y / 2.0 - 30.0;

    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("player1") || event.state.is_end() { continue; }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_message.value = format!("Health: {}", game_state.health_amount);
        }
        engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        event.pair.into_iter().filter(|label| label != "player1").for_each(|label|{
            let obstacle = engine.sprites.get_mut(&label).unwrap();
            obstacle.translation.x = rng().random_range(800.0..1600.0);
            obstacle.translation.y = rng().random_range(-300.0..300.0);
        });
    }


    // Game Over
    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over = engine.add_text("game_over", "Game Over");
        game_over.font_size = 128.0;
        game_over.translation.y = 100.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}