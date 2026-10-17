use tetra::graphics::{Texture};
use tetra::math::Vec2;
use tetra::Context;
use rand::rngs::ThreadRng;
use rand::{self, Rng};

use super::platform::Platform;

fn get_random_position(rng: &mut ThreadRng) -> Vec2<f32> {
    let x_pos = ((rng.gen::<f32>() * super::PLATFORM_ACROSS as f32).floor()) * super::PLATFORM_DIMENSIONS.x + super::PLATFORM_DIMENSIONS.x / 2.0 - 7.5;
    let y_pos = ((rng.gen::<f32>() * super::PLATFORM_DOWN as f32).floor()) * super::PLATFORM_DIMENSIONS.y + super::PLATFORM_DIMENSIONS.y / 2.0 - 8.0;
    Vec2::new(x_pos, y_pos)
}

fn get_random_position_not_on_platform(rng: &mut ThreadRng, platforms: &[Platform]) -> Vec2<f32> {
    let x_pos = ((rng.gen::<f32>() * super::PLATFORM_ACROSS as f32).floor()) * super::PLATFORM_DIMENSIONS.x + super::PLATFORM_DIMENSIONS.x / 2.0 - 7.5;
    let y_pos = ((rng.gen::<f32>() * super::PLATFORM_DOWN as f32).floor()) * super::PLATFORM_DIMENSIONS.y + super::PLATFORM_DIMENSIONS.y / 2.0 - 8.0;
    for platform in platforms {
        let pos = platform.get_pos();
        let dim = platform.get_dim();
        if x_pos >= pos.x && x_pos <= pos.x + dim.x && y_pos >= pos.y && y_pos <= pos.y + dim.y {
            return get_random_position_not_on_platform(rng, platforms);
        }
    }
    Vec2::new(x_pos, y_pos)
}

#[derive(Debug, Clone)]
pub struct Token {
    texture: Texture,
    position: Vec2<f32>,
    dimension: Vec2<f32>,
    rng: ThreadRng,
    hit: bool,
}

impl Token {
    pub fn new(t: Texture, dim: Vec2<f32>, use_random: bool, pos: Vec2<f32>) -> Token {
        let mut rng = rand::thread_rng();
        if use_random {
            Token {        
                texture: t,
                position: get_random_position(&mut rng),
                dimension: dim,     
                rng: rng,
                hit: false,
            }
        } else {
            Token {
                texture: t,
                position: pos,
                dimension: dim,
                rng: rng,
                hit: false,
            }
        }
    }

    fn respawn(&mut self, platforms: &[Platform]) {
        self.position = get_random_position_not_on_platform(&mut self.rng, platforms);
    }

    pub fn update(&mut self, platforms: Vec<Platform>) {
        if self.hit {
            println!("TOKEN WAS HIT");
            // self.position.y -= super::WINDOW_HEIGHT as f32 + (1.0 * 15.0);
            // self.position.x = ((self.rng.gen::<f32>() * 30.0).floor()) * 16.0;
            self.respawn(&platforms);
            self.hit = false;
            return;
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        self.texture.draw(ctx, self.position);
    }

    pub fn get_pos(&self) -> Vec2<f32> {
        self.position
    }

    pub fn get_dim(&self) -> Vec2<f32> {
        self.dimension
    }

    pub fn get_hit(&self) -> bool {
        self.hit
    }

    pub fn set_hit(& mut self, is_hit: bool) {
        self.hit = is_hit;
    }
}