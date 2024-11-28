
use tetra::graphics::{Texture, DrawParams};
use tetra::math::Vec2;
use tetra::Context;
use rand::rngs::ThreadRng;
use rand::{self, Rng};
use super::{Queue, POSITION_BUFFER_SIZE};

#[derive(Debug, Clone)]

enum BatState {
    Chase,
    Boid,
}
#[derive(Debug, Clone)]
pub struct Enemy {
    id: u16,
    textures: Vec<Texture>,
    position: Vec2<f32>,
    velocity: Vec2<f32>,
    dimension: Vec2<f32>,
    dest_ind: usize,
    is_facing_left: bool,
    state: BatState,
    debug: bool,
    boids_center: Vec2<f32>,
    visual_range: f32
}



impl Enemy {
    pub fn new(id: u16, ts: Vec<Texture>, dim: Vec2<f32>, rng: &mut ThreadRng, visual_range: f32, debug: bool) -> Enemy {
        let x_pos = rng.gen::<f32>() * 640.0;
        let y_pos = rng.gen::<f32>() * 480.0 - 200.0;

        Enemy {     
            id,
            textures: ts,
            position: Vec2::new(x_pos,y_pos),
            velocity: Vec2::new(0.0,0.0),
            dimension: dim,
            dest_ind: rng.gen_range(0..POSITION_BUFFER_SIZE),
            is_facing_left: true,
            state: BatState::Boid,
            debug,
            boids_center: Vec2::new(0.0,0.0),
            visual_range,
        }
    }

    pub fn get_pos(&self) -> Vec2<f32> {
        self.position
    }

    pub fn get_vel(&self) -> Vec2<f32> {
        self.velocity
    }

    pub fn set_state(&mut self, is_chase: bool) {
        if is_chase {
            self.state = BatState::Chase;
        } else {
            self.state = BatState::Boid;
        }
    }

    fn oldest_player_pos(&self, player_pos_buf: &Queue<Vec2<f32>>) -> Vec2<f32> {
        if self.dest_ind <= POSITION_BUFFER_SIZE - 1 {
            if let Some(&oldest_player_pos) = player_pos_buf.get_at(self.dest_ind) {
                oldest_player_pos
            } else {
                Vec2::new(0.0,0.0)
            }
        } else {
            Vec2::new(0.0,0.0)
        }
    }

    fn chase_player(&self, player_pos_buf: &Queue<Vec2<f32>>)  -> Vec2<f32> {
        self.oldest_player_pos(player_pos_buf) - self.position
    }

    fn rule1(&self) -> Vec2<f32> {
        (self.boids_center - self.position) * 0.005
    }

    fn distance(&self, b: &Vec2<f32>) -> f32 {
        ((self.position.x - b.x).powi(2) + (self.position.y - b.y).powi(2)).sqrt()
    }

    fn rule2(&self, boids_pos: &Vec<Vec2<f32>>) -> Vec2<f32> {
        let distance: f32 = 15.0;
        let avoid = 0.005;
        let mut result:Vec2<f32> = Vec2::new(0.0,0.0);
        for b in boids_pos {
            let distance_from = self.distance(b);
            if distance_from < distance { // && distance_from != 0.0
                result += self.position - b;
            }
        }
        result * avoid
    }

    fn rule3(&self, boids_vel: &Vec<Vec2<f32>>) -> Vec2<f32> {
        let mut result:Vec2<f32> = Vec2::new(0.0,0.0);
        let factor = 0.005;
        for b in boids_vel {
            if *b != self.velocity {
                result += *b;
            }
        }
        ((result / ((boids_vel.len() - 1) as f32)) - self.velocity) * factor
    }

    fn limit_speed(&mut self) {
        let max_speed = 4.5;
        let magnitude = (self.velocity.x * self.velocity.x + self.velocity.y * self.velocity.y).sqrt();
        if magnitude > max_speed {
            self.velocity.x = (self.velocity.x / magnitude) * max_speed;
            self.velocity.y = (self.velocity.y / magnitude) * max_speed;
        }
    }

    fn stay_within_window(&self) -> Vec2<f32> {
        let mut result:Vec2<f32> = Vec2::new(0.0,0.0);
        let margin = 10.0;
        let turn = 0.2;

        if self.position.x < margin {
            result.x += turn;
        } 
        if self.position.x > crate::WINDOW_WIDTH as f32 - margin {
            result.x -= turn;
        }
        if self.position.y < margin {
            result.y += turn;
        }
        if self.position.y > crate::WINDOW_HEIGHT as f32 - margin {
            result.y -= turn;
        }
        result
    }

    fn boids_towards_player(&self, player_pos_buf: &Queue<Vec2<f32>>) -> Vec2<f32> {
        let player_pos = self.oldest_player_pos(player_pos_buf);
        let factor = 0.005;
        let distance_to_player = self.distance(&player_pos);
        if distance_to_player < self.visual_range {
            (player_pos - self.position) * factor
        } else {
            Vec2::new(0.0,0.0)
        }
    }

    fn boids_in_range(&self, boids_pos: &Vec<Vec2<f32>>) -> Vec<Vec2<f32>> {
        boids_pos
            .iter() // Iterate over references to the positions
            .filter(|&pos| self.distance(pos) <= self.visual_range) // Filter based on distance
            .cloned() // Clone the Vec2 to return owned data
            .collect()
    }

    fn calculate_center(&mut self, boids_seen: &Vec<Vec2<f32>>) {
        let mut pos_sum = Vec2::<f32>::new(0.02,0.0);
            for b in boids_seen {
                pos_sum += *b;
            }
        self.boids_center = pos_sum / (boids_seen.len() as f32);
        // if self.boids_center.x == 0.0 && self.boids_center.y == 0.0 {
        //     self.boids_center = Vec2::<f32>::new(crate::WINDOW_WIDTH as f32 / 2.0,crate::WINDOW_HEIGHT as f32 / 2.0);
        // }
    }

    fn boid(&mut self, player_pos_buf: &Queue<Vec2<f32>>, boids_pos: &Vec<Vec2<f32>>, boids_vel: &Vec<Vec2<f32>>) -> Vec2<f32> {
        let boids_seen = self.boids_in_range(boids_pos);
        self.calculate_center(&boids_seen);
        let mut new_vel: Vec2<f32> = Vec2 { x: (0.0), y: (0.0) };

        let towards_player = self.boids_towards_player(player_pos_buf);
        let rule1 = self.rule1(); // towards boids center
        let rule2 = self.rule2(boids_pos); // avoid others
        let rule3 = self.rule3(boids_vel); // match speed
        let stay_inside = self.stay_within_window();

        // if new_vel.x == 0.0 && new_vel.y == 0.0 {
        //     new_vel.x = 0.05;
        //     new_vel.y = 0.05;
        // }

        new_vel += 1.0 * (rule1 + rule2 + rule3 + stay_inside + towards_player);
        new_vel
    }

    pub fn toggle_state(&mut self) {
        match self.state {
            BatState::Chase => {self.state = BatState::Boid;
            self.velocity.x *= 10.74; self.velocity.y *= 10.5;},
            BatState::Boid => self.state = BatState::Chase,
        }
    }

    pub fn update(&mut self, player_pos_buf: &Queue<Vec2<f32>>, boids_pos: &Vec<Vec2<f32>>, boids_vel: &Vec<Vec2<f32>>) {
        match self.state {
            BatState::Chase => {self.velocity = self.chase_player(player_pos_buf)},
            BatState::Boid => {self.velocity = self.velocity * 1.01 + self.boid(player_pos_buf, boids_pos, boids_vel)}
        }
        self.limit_speed();
        // self.boids_center = boids_center;
        if self.velocity.x < 0.0 {
            self.is_facing_left = true;
        } else {
            self.is_facing_left = false;
        }
        self.position += self.velocity;
        if self.debug {
            // println!("Pos: {},{} | Vel: {},{}", self.position.x, self.position.y, self.velocity.x, self.velocity.y);
        }
    }

    pub fn draw(&self, ctx: &mut Context) {
        if self.is_facing_left {
            self.textures[0].draw(ctx, DrawParams::new()
            .position(self.position)
            .origin(Vec2::new(8.0, 8.0))
            .scale(Vec2::new(2.0, 2.0)),);
        } else {
            self.textures[1].draw(ctx, DrawParams::new()
            .position(self.position)
            .origin(Vec2::new(8.0, 8.0))
            .scale(Vec2::new(2.0, 2.0)),);
        }

        // draws percieved center for debugging
        // if self.debug {
        //     self.textures[2].draw(ctx, DrawParams::new()
        //     .position(self.boids_center)
        //     .origin(Vec2::new(8.0, 8.0))
        //     .scale(Vec2::new(2.0, 2.0)),);
        // }
    }
}