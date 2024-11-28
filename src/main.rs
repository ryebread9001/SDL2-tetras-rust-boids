use queue::Queue;
use enemy::Enemy;
use tetra::graphics::{self, Color, DrawParams, Texture};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use rand::rngs::ThreadRng;
use rand::{self, Rng};

pub mod queue;
pub mod enemy;

const PLATFORM_NUM: usize = 25;
const ENEMY_NUM: usize = 40;
const POSITION_BUFFER_SIZE: usize = 100;
pub const WINDOW_WIDTH: i32 = 640;
pub const WINDOW_HEIGHT: i32 = 480;


struct Platform {
    texture: Texture,
    position: Vec2<f32>,
    dimension: Vec2<f32>
}

impl Platform {
    fn new(t: Texture, dim: Vec2<f32>, rng: &mut ThreadRng) -> Platform {
        let x_pos = rng.gen::<f32>() * 640.0;
        let y_pos = rng.gen::<f32>() * 480.0 + 100.0;
        Platform {        
            texture: t,
            position: Vec2::new(x_pos,y_pos),
            dimension: dim,       
        }
    }
}

struct GameState {
    textures: Vec<Texture>,
    position: Vec2<f32>,
    position_buf: Queue<Vec2<f32>>,
    dimension: Vec2<f32>,
    velocity: Vec2<f32>,
    is_facing_left: bool,
    is_jumping: bool,
    is_wall_jumping: bool,
    move_speed: f32,
    platforms: Vec<Platform>,
    enemies: Vec<Enemy>
}



impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let mut platforms = Vec::with_capacity(PLATFORM_NUM);
        let mut enemies = Vec::with_capacity(ENEMY_NUM);
        let mut rng = rand::thread_rng();
        let mut next_available_id: u16 = 0;

        for _ in 0..PLATFORM_NUM {
            platforms.push(Platform::new(Texture::new(ctx, "./resources/platform.png")?, Vec2::new(78.0,50.0), &mut rng));
        }
        let mut bat_textures = Vec::new();
        bat_textures.push(Texture::new(ctx, "./resources/bat-l.png")?);
        bat_textures.push(Texture::new(ctx, "./resources/bat-r.png")?);
        bat_textures.push(Texture::new(ctx, "./resources/wabbit_alpha.png")?);

        let bat_visual_range = 100.0;
        for _ in 0..ENEMY_NUM-1 {
            enemies.push(Enemy::new(next_available_id, bat_textures.clone(), Vec2::new(20.0,20.0), &mut rng, bat_visual_range, false));
            next_available_id += 1;
        }
        // set debug to true for one bat's debug console out
        enemies.push(Enemy::new(next_available_id, bat_textures.clone(), Vec2::new(20.0,20.0), &mut rng, bat_visual_range, true));
        
        let mut textures = Vec::new();
        textures.push(Texture::new(ctx, "./resources/player-l.png")?);
        textures.push(Texture::new(ctx, "./resources/player.png")?);
        let start_pos = Vec2::new(32.0, 32.0);
        let mut q = Queue::new(POSITION_BUFFER_SIZE);
        q.fill_with(start_pos);
        Ok(GameState {
            textures: textures,
            position: start_pos,
            position_buf: Queue::new(POSITION_BUFFER_SIZE),
            dimension: Vec2::new(10.0, 10.0),
            velocity: Vec2::new(0.0,0.0),
            is_facing_left: false,
            is_jumping: false,
            is_wall_jumping: false,
            move_speed: 1.2,
            platforms: platforms,
            enemies: enemies
        })
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        if self.position.y < 460.0 {
            self.velocity.y += 0.75;
        } else {
            self.position.y = 460.0;
            self.velocity.y = 0.0;
            self.is_jumping = false;
        }
        self.velocity.x *= 0.8;

        
        if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
            self.velocity.x -= self.move_speed;
            self.is_facing_left = true;
        }

        if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
            self.velocity.x += self.move_speed;
            self.is_facing_left = false;

        }

        if !self.is_jumping && (input::is_key_pressed(ctx, Key::W) || input::is_key_pressed(ctx, Key::Up)) {
            self.velocity.y -= 12.0;
            self.is_jumping = true;
        }

        if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
            self.velocity.y += 0.4;
        }
        let mut toggle_bat_state = false;
        if input::is_key_released(ctx, Key::T) {
            toggle_bat_state = true;
        }

        for platform in &self.platforms {

            let fut_x = self.position.x + self.velocity.x;
            if  fut_x >= platform.position.x + platform.dimension.x {
                
            } else if fut_x + self.dimension.x <= platform.position.x {
                
            } else if self.position.y >= platform.position.y + platform.dimension.y {
                
            } else if self.position.y + self.dimension.y <= platform.position.y {
                
            } else {
                self.velocity.x = 0.0;
                if !self.is_wall_jumping {
                    self.is_wall_jumping = true;
                    self.is_jumping = false;
                }
                    
            }
            
            let fut_y = self.position.y + self.velocity.y;
            if  self.position.x >= platform.position.x + platform.dimension.x {
                
            } else if self.position.x + self.dimension.x <= platform.position.x {
                
            } else if fut_y >= platform.position.y + platform.dimension.y {
                
            } else if fut_y + self.dimension.y <= platform.position.y {
                
            } else {
                self.velocity.y = 0.0;
                if self.position.y > platform.position.y {
                    self.is_jumping = true;
                } else {
                    self.is_jumping = false;
                    self.is_wall_jumping = false;
                }
            }
            platform.texture.draw(ctx, platform.position);
        }


        self.position_buf.push(self.position);
        if self.position_buf.get_items_in_q() > POSITION_BUFFER_SIZE - 1 {
            self.position_buf.pop();
        }


        self.position.y += self.velocity.y;
        self.position.x += self.velocity.x;

        // fn boid_center(boids: &Vec<Enemy>) -> Vec2<f32> {
        //     let mut pos_sum = Vec2::<f32>::new(0.0,0.0);
        //     for enemy in boids {
        //         pos_sum += enemy.get_pos();
        //     }
        //     pos_sum / (boids.len() as f32)
        // }
        // let center = boid_center(&self.enemies);
        let n = self.enemies.len();
        if n > 0 {
            let mut boids_pos:Vec<Vec2<f32>> = vec![Vec2::new(0.0,0.0); n];
            let mut boids_vel:Vec<Vec2<f32>> = vec![Vec2::new(0.0,0.0); n];
            for i in 0..(self.enemies).len() {
                boids_pos[i] = self.enemies[i].get_pos();
                boids_vel[i] = self.enemies[i].get_vel();
            }
            for enemy in &mut self.enemies {
                // formula for finding mean without some sample value, in this case, the current enemy boid pos
                // let center_minus_curr = ((n as f32 * center) - enemy.get_pos()) /  ((n - 1) as f32);
                if toggle_bat_state {
                    enemy.toggle_state();
                }
                enemy.update(&self.position_buf, &boids_pos, &boids_vel);
            }            
        }

        let mut pressed = input::get_keys_pressed(ctx).peekable();
        if pressed.peek().is_some() {
            println!(
                "Keys pressed this update: {:?}",
                pressed.collect::<Vec<_>>()
            );
        }

        let mut released = input::get_keys_released(ctx).peekable();
        if released.peek().is_some() {
            println!(
                "Keys released this update: {:?}",
                released.collect::<Vec<_>>()
            );
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.769, 0.812, 0.631));
        if self.is_facing_left {
            self.textures[0].draw(
                ctx,
                DrawParams::new()
                    .position(self.position)
                    .origin(Vec2::new(8.0, 8.0))
                    .scale(Vec2::new(2.0, 2.0)),
            );
        } else {
            self.textures[1].draw(
                ctx,
                DrawParams::new()
                    .position(self.position)
                    .origin(Vec2::new(8.0, 8.0))
                    .scale(Vec2::new(2.0, 2.0)),
            );
        }

        for platform in &self.platforms {
            platform.texture.draw(ctx, platform.position);
        }

        for enemy in &self.enemies {
            enemy.draw(ctx);
        }

        Ok(())
    }
}

fn main() -> tetra::Result {
    ContextBuilder::new("Keyboard Input", WINDOW_WIDTH, WINDOW_HEIGHT)
        .quit_on_escape(true)
        .build()?
        .run(GameState::new)
}