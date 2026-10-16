use tetra::graphics::{Texture};
use tetra::math::Vec2;
use tetra::Context;
use rand::rngs::ThreadRng;
use rand::{self, Rng};

const x_multiplier: f32 = 72.0;
const x_rand: f32 = 9.41;
const x_offset: f32 = 18.5;
const y_multiplier: f32 = 38.0;
const y_rand: f32 = 12.97;
const y_offset: f32 = 37.0;


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
            let x_pos = ((rng.gen::<f32>() * x_rand).floor()) * x_multiplier + x_offset;
            let y_pos = ((rng.gen::<f32>() * y_rand).floor()) * y_multiplier + y_offset;
            Token {        
                texture: t,
                position: Vec2::new(x_pos,y_pos),
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
        // let y_pos = ((rng.gen::<f32>() * 42.6).floor()) * 15.0;
        // let x_pos = ((rng.gen::<f32>() * 30.0).floor()) * 16.0;
        self.position.x = ((self.rng.gen::<f32>() * x_rand).floor()) * x_multiplier + x_offset;
        self.position.y -= super::WINDOW_HEIGHT as f32 - ((self.rng.gen::<f32>() * y_rand).floor()) * y_multiplier + y_offset;

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