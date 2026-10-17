use enemy::Enemy;
use player::Player;
use platform::Platform;
use rand::rngs::ThreadRng;
use rand::{self, Rng};
use tetra::graphics::mesh::{ShapeStyle, Mesh};
use tetra::input::KeyLabel::S;
use token::Token;
use tetra::graphics::{self, Color, Texture, Rectangle};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

pub mod player;
pub mod platform;
pub mod enemy;
pub mod token;
pub mod game_object;
pub mod queue;
const BACKGROUND_COLOR: Color = Color::rgb(245.0 / 255.0, 226.0 / 255.0, 210.0 / 255.0); //Color::rgb(0.769, 0.812, 0.631)
const PLATFORM_NUM: usize = 45;
const ENEMY_NUM: usize = 30;
const TOKEN_NUM: usize = 10;
const POSITION_BUFFER_SIZE: usize = 20;
const BAT_VISUAL_RANGE: f32 = 10.0;
const DOWN_SPEED_INITIAL: f32 = 0.0;
pub const PLATFORM_DIMENSIONS: Vec2<f32> = Vec2::new(68.0, 37.0);
pub const PLATFORM_ACROSS: i32 = 16;
pub const PLATFORM_DOWN: i32 = 16;
pub const WINDOW_WIDTH: i32 = PLATFORM_DIMENSIONS.x as i32 * PLATFORM_ACROSS;
pub const WINDOW_HEIGHT: i32 = PLATFORM_DIMENSIONS.y as i32 * PLATFORM_DOWN;
pub const PLAYER_WIDTH: f32 = 32.0;
pub const PLAYER_HEIGHT: f32 = 32.0;

struct GameState {
    player: Player,
    platforms: Vec<Platform>,
    enemies: Vec<Enemy>,
    tokens: Vec<Token>,
    down_speed: f32
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let mut textures = Vec::new();
        textures.push(Texture::new(ctx, "./resources/player-l.png")?);
        textures.push(Texture::new(ctx, "./resources/player.png")?);
        textures.push(Texture::new(ctx, "./resources/health.png")?);
        let start_pos = Vec2::new(WINDOW_WIDTH as f32 / 2.0, WINDOW_HEIGHT as f32 / 2.0);
        let simple: Mesh = Mesh::rectangle(
            ctx,
            ShapeStyle::Stroke(1.0),
            Rectangle::new(0.0, 0.0, PLAYER_WIDTH, PLAYER_HEIGHT),
        )?;
        let player = Player::new(ctx, textures, start_pos, simple);

        let mut platforms = Vec::with_capacity(PLATFORM_NUM);
        let mut enemies = Vec::with_capacity(ENEMY_NUM);
        let mut tokens: Vec<Token> = Vec::with_capacity(TOKEN_NUM);
        let mut rng = rand::thread_rng();
        let mut next_available_id: u16 = 0;

        // first platform under player
        platforms.push(Platform::new(Texture::new(ctx, "./resources/platform.png")?, PLATFORM_DIMENSIONS, false, Vec2::new(0.0, 148.0)));
        
        for plat_x in 0..PLATFORM_ACROSS {
            platforms.push(Platform::new(Texture::new(ctx, "./resources/platform.png")?, PLATFORM_DIMENSIONS, false, Vec2::new((plat_x as f32) * PLATFORM_DIMENSIONS.x, PLATFORM_DIMENSIONS.y * (PLATFORM_DOWN - 1) as f32)));
        }
        for _ in 1..PLATFORM_NUM {
            platforms.push(Platform::new(Texture::new(ctx, "./resources/platform.png")?, PLATFORM_DIMENSIONS, true, Vec2::new(0.0, 0.0)));
        }

        for _ in 0..TOKEN_NUM {
            tokens.push(Token::new(Texture::new(ctx, "./resources/token.png")?, Vec2::new(15.0,16.0), true, Vec2::new(0.0,0.0)));
        }

        let mut bat_textures = Vec::new();
        bat_textures.push(Texture::new(ctx, "./resources/bat-l.png")?);
        bat_textures.push(Texture::new(ctx, "./resources/bat-r.png")?);

        for _ in 0..ENEMY_NUM-1 {
            let triangle_mesh = Mesh::polygon(
                ctx,
                ShapeStyle::Fill,
                &[
                    Vec2::new(10.0, 0.0),    // Tip (pointing East / Right)
                    Vec2::new(-10.0, -5.0), // Bottom Left
                    Vec2::new(-10.0, 5.0),  // Bottom Right
                ],
            )?;
            enemies.push(Enemy::new(next_available_id, bat_textures.clone(), triangle_mesh, Vec2::new(20.0,20.0), &mut rng, BAT_VISUAL_RANGE, false));
            next_available_id += 1;
        }
        // set debug to true for one bat's debug console out
        //enemies.push(Enemy::new(next_available_id, bat_textures.clone(), Vec2::new(20.0,20.0), &mut rng, BAT_VISUAL_RANGE, true));
        
        let mut textures = Vec::new();
        textures.push(Texture::new(ctx, "./resources/player-l.png")?);
        textures.push(Texture::new(ctx, "./resources/player.png")?);
        textures.push(Texture::new(ctx, "./resources/health.png")?);

        Ok(GameState {
            player,
            platforms,
            enemies,
            tokens,
            down_speed: DOWN_SPEED_INITIAL
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        let player = &mut self.player;
        let platforms = &mut self.platforms;
        let enemies = &mut self.enemies;
        let tokens = &mut self.tokens;

        let n = enemies.len();
        let mut boids_pos:Vec<Vec2<f32>> = vec![Vec2::new(0.0,0.0); n];
        let mut boids_vel:Vec<Vec2<f32>> = vec![Vec2::new(0.0,0.0); n];
        if n > 0 {
            for i in 0..(enemies).len() {
                boids_pos[i] = enemies[i].get_pos();
                boids_vel[i] = enemies[i].get_vel();
            }
        }

        // UPDATES HERE
        for i in 0..platforms.len() {
            platforms[i].update(self.down_speed);
        }

        let player_oldest_pos = player.oldest_player_pos();
        let player_score = player.get_score();
        for j in 0..enemies.len() {
            enemies[j].update(player_oldest_pos, player_score, &boids_pos, &boids_vel)
        }

        player.update(ctx, platforms.clone(), enemies.clone(), tokens);

        for i in 0..tokens.len() {
            tokens[i].update(self.down_speed);
        }

        // let mut pressed = input::get_keys_pressed(ctx).peekable();
        // if pressed.peek().is_some() {
        //     println!(
        //         "Keys pressed this update: {:?}",
        //         pressed.collect::<Vec<_>>()
        //     );
        // }

        // let mut released = input::get_keys_released(ctx).peekable();
        // if released.peek().is_some() {
        //     println!(
        //         "Keys released this update: {:?}",
        //         released.collect::<Vec<_>>()
        //     );
        // }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, BACKGROUND_COLOR);
        
        for token in &self.tokens {
            token.draw(ctx);
        }
        
        for platform in &self.platforms {
            platform.draw(ctx);
        }

        
        for enemy in &self.enemies {
            enemy.draw(ctx);
        }
        
        self.player.draw(ctx);

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Keyboard Input", WINDOW_WIDTH, WINDOW_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}