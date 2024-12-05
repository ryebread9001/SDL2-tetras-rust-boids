use tetra::graphics::Texture;
use tetra::math::Vec2;

struct GameObject {
    position: Vec2<f32>,
    dimension: Vec2<f32>,
    velocity: Vec2<f32>,
    textures: Vec<Texture>,
}

impl GameObject {
    fn new(txts: Vec<Texture>, pos: Vec2<f32>, vel: Vec2<f32>, dim: Vec2<f32>) -> GameObject {
        GameObject {        
            position: pos,
            dimension: dim,
            velocity: vel,      
            textures: txts,
        }
    }

    fn update(&mut self) {
        self.position.y += self.velocity.y;
        self.position.x += self.velocity.x;
    }
}