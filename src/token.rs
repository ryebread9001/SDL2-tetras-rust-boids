use tetra::graphics::{Texture};
use tetra::math::Vec2;
use tetra::Context;
use rand::rngs::ThreadRng;
use rand::{self, Rng};

fn get_random_position(rng: &mut ThreadRng) -> Vec2<f32> {
    let x_pos = ((rng.gen::<f32>() * super::PLATFORM_ACROSS as f32).floor()) * super::PLATFORM_DIMENSIONS.x + super::PLATFORM_DIMENSIONS.x / 2.0 - 7.5;
    let y_pos = ((rng.gen::<f32>() * super::PLATFORM_DOWN as f32).floor()) * super::PLATFORM_DIMENSIONS.y + super::PLATFORM_DIMENSIONS.y / 2.0 - 8.0;
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

    fn respawn(&mut self) {
        self.position = get_random_position(&mut self.rng);
    }

    pub fn update(&mut self, down_speed: f32) {
        if self.hit {
            println!("TOKEN WAS HIT");
            // self.position.y -= super::WINDOW_HEIGHT as f32 + (1.0 * 15.0);
            // self.position.x = ((self.rng.gen::<f32>() * 30.0).floor()) * 16.0;
            self.respawn();
            self.hit = false;
            return;
        }

        self.position.y += down_speed;
        if self.position.y >= super::WINDOW_HEIGHT as f32 {
            // self.position.y -= super::WINDOW_HEIGHT as f32 + (1.0 * 15.0);
            // self.position.x = ((self.rng.gen::<f32>() * 30.0).floor()) * 16.0;
            self.respawn();

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