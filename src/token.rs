use tetra::graphics::{Texture};
use tetra::math::Vec2;
use tetra::Context;
use rand::rngs::ThreadRng;
use rand::{self, Rng};


#[derive(Debug, Clone)]
pub struct Token {
    texture: Texture,
    position: Vec2<f32>,
    dimension: Vec2<f32>,
    rng: ThreadRng
}

impl Token {
    pub fn new(t: Texture, dim: Vec2<f32>, use_random: bool, pos: Vec2<f32>) -> Token {
        let mut rng = rand::thread_rng();
        if use_random {
            let y_pos = ((rng.gen::<f32>() * 42.6).floor()) * 15.0;
            let x_pos = ((rng.gen::<f32>() * 30.0).floor()) * 16.0;
            Token {        
                texture: t,
                position: Vec2::new(x_pos,y_pos),
                dimension: dim,     
                rng  
            }
        } else {
            Token {
                texture: t,
                position: pos,
                dimension: dim,
                rng
            }
        }
    }

    pub fn update(&mut self, down_speed: f32) {
        self.position.y += down_speed;
        if self.position.y >= super::WINDOW_HEIGHT as f32 {
            self.position.y -= super::WINDOW_HEIGHT as f32 + (1.0 * 15.0);
            self.position.x = ((self.rng.gen::<f32>() * 30.0).floor()) * 16.0;

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
}