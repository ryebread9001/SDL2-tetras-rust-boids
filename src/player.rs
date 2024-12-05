use tetra::math::Vec2;
use tetra::graphics::{DrawParams, Texture, Color, self};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::Context;

use crate::WINDOW_HEIGHT;

use super::{queue::Queue, platform::Platform, enemy::Enemy, token::Token};

pub struct Player {
    textures: Vec<Texture>,
    position: Vec2<f32>,
    spawn_pos: Vec2<f32>,
    dimension: Vec2<f32>,
    velocity: Vec2<f32>,
    pos_buf: Queue<Vec2<f32>>,
    health: f32,
    is_facing_left: bool,
    is_jumping: bool,
    is_wall_jumping: bool,
    collision: bool,
    move_speed: f32,
    restart_text: Text,
    score: u32
}

impl Player {
    pub fn new(txts: Vec<Texture>, start_pos: Vec2<f32>, restart_text: Text) -> Player {

        let mut q = Queue::new(super::POSITION_BUFFER_SIZE);
        q.fill_with(start_pos);
        Player {
            textures: txts,
            position: start_pos,
            spawn_pos: start_pos,
            dimension: Vec2::new(10.0, 10.0),
            velocity: Vec2::new(0.0,0.0),
            pos_buf: q,
            health: 100.0,
            is_facing_left: false,
            is_jumping: false,
            is_wall_jumping: false,
            collision: false,
            move_speed: 1.2,
            restart_text,
            score: 0
        }
    }

    pub fn oldest_player_pos(&self) -> Vec2<f32> {
        if 4 <= super::POSITION_BUFFER_SIZE - 1 {
            if let Some(&oldest_player_pos) = self.pos_buf.get_at(4) {
                oldest_player_pos
            } else {
                Vec2::new(0.0,0.0)
            }
        } else {
            Vec2::new(0.0,0.0)
        }
    }

    fn platform_collisions(&mut self, platforms: Vec<Platform>) {
        for platform in &platforms {
            let pos = platform.get_pos();
            let dim = platform.get_dim();
            let fut_x = self.position.x + self.velocity.x;
            if  fut_x >= pos.x + dim.x {
                
            } else if fut_x + self.dimension.x <= pos.x {
                
            } else if self.position.y >= pos.y + dim.y {
                
            } else if self.position.y + self.dimension.y <= pos.y {
                
            } else {
                self.velocity.x = 0.0;
                if !self.is_wall_jumping {
                    self.is_wall_jumping = true;
                    self.is_jumping = false;
                }
                    
            }
            
            let fut_y = self.position.y + self.velocity.y;
            if  self.position.x >= pos.x + dim.x {
                
            } else if self.position.x + self.dimension.x <= pos.x {
                
            } else if fut_y >= pos.y + dim.y {
                
            } else if fut_y + self.dimension.y <= pos.y {
                
            } else {
                self.velocity.y = 0.0;
                self.collision = true;
                if self.position.y > pos.y {
                    self.is_jumping = true;
                } else {
                    self.is_jumping = false;
                    self.position.y -= 0.1;
                    self.is_wall_jumping = false;
                }
            }
        }
    }

    fn enemy_collisions(&mut self, enemies: Vec<Enemy>) {
        for enemy in &enemies {
            // formula for finding mean without some sample value, in this case, the current enemy boid pos
            // let center_minus_curr = ((n as f32 * center) - enemy.get_pos()) /  ((n - 1) as f32);
            let e_pos = enemy.get_pos();
            let e_dim = enemy.get_dim();
            // player-bat collision detection
            let fut_x = self.position.x + self.velocity.x;
            if  fut_x >= e_pos.x + e_dim.x {
                
            } else if fut_x + self.dimension.x <= e_pos.x {
                
            } else if self.position.y >= e_pos.y + e_dim.y {
                
            } else if self.position.y + self.dimension.y <= e_pos.y {
                
            } else {
                if self.health > 0.0 {
                    self.health -= 0.1;
                }
            }
            
            let fut_y = self.position.y + self.velocity.y;
            if  self.position.x >= e_pos.x + e_dim.x {
                
            } else if self.position.x + self.dimension.x <= e_pos.x {

            } else if fut_y >= e_pos.y + e_dim.y {
                
            } else if fut_y + self.dimension.y <= e_pos.y {
                
            } else {
                if self.health > 0.0 {
                    self.health -= 0.1;
                }
            }
        }            
    }

    fn token_collisions(&mut self, tokens: Vec<Token>) {
        for token in &tokens {
            // formula for finding mean without some sample value, in this case, the current enemy boid pos
            // let center_minus_curr = ((n as f32 * center) - enemy.get_pos()) /  ((n - 1) as f32);
            let e_pos = token.get_pos();
            let e_dim = token.get_dim();

            let mut hit_y = false;
            let mut hit_x = false;
            // player-bat collision detection
            let fut_x = self.position.x + self.velocity.x;
            if  fut_x >= e_pos.x + e_dim.x {
                
            } else if fut_x + self.dimension.x <= e_pos.x {
                
            } else if self.position.y >= e_pos.y + e_dim.y {
                
            } else if self.position.y + self.dimension.y <= e_pos.y {
                
            } else {
                hit_x = true;
            }
            
            let fut_y = self.position.y + self.velocity.y;
            if  self.position.x >= e_pos.x + e_dim.x {
                
            } else if self.position.x + self.dimension.x <= e_pos.x {

            } else if fut_y >= e_pos.y + e_dim.y {
                
            } else if fut_y + self.dimension.y <= e_pos.y {
                
            } else {
                hit_y = true;
            }

            if hit_x || hit_y {
                if self.health > 0.0 {
                    self.health += 10.0;
                    self.score += 1;
                }
                if self.health > 100.0 {
                    self.health = 100.0
                }
            }
        }            
    }

    pub fn update(&mut self, ctx: &mut Context, platforms: Vec<Platform>, enemies: Vec<Enemy>, tokens: Vec<Token>) {

        if self.position.y < WINDOW_HEIGHT as f32 + self.dimension.y {
            self.velocity.y += 0.75;
        } else {
            if self.health > 0.0 {
                self.health -= 2.5;
            }
        }
        self.velocity.x *= 0.8;

        if self.position.y + self.velocity.y < 0.0 {
            self.velocity.y = 0.0;
        }
        
        if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
            self.velocity.x -= self.move_speed;
            self.is_facing_left = true;
        }

        if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
            self.velocity.x += self.move_speed;
            self.is_facing_left = false;

        }

        if !self.is_jumping && (input::is_key_pressed(ctx, Key::W) || input::is_key_pressed(ctx, Key::Up)) {
            self.velocity.y -= 15.0;
            self.is_jumping = true;
        }

        if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
            self.velocity.y += 0.4;
        }

        if input::is_key_pressed(ctx, Key::R) {
            self.health = 100.0;
            self.position = self.spawn_pos;
        }

        // Platform Collision Check Here?
        self.platform_collisions(platforms);
        self.enemy_collisions(enemies);
        self.token_collisions(tokens);

        self.pos_buf.push(self.position);
        if self.pos_buf.get_items_in_q() > super::POSITION_BUFFER_SIZE - 1 {
            self.pos_buf.pop();
        }

        self.position.y += self.velocity.y;
        self.position.x += self.velocity.x;

        // println!("Player Health: {}", self.health);
    }

    pub fn draw(&mut self, ctx: &mut Context) {

        // LEFT RIGHT PLAYER TEXTURES
        if self.health > 0.0 {
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
        } else {
            graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
            self.restart_text.draw(ctx, Vec2::new(super::WINDOW_WIDTH as f32 /2.0 - 90.0, super::WINDOW_HEIGHT as f32 / 2.0 - 48.0))
        }

        // HEALTH BAR
        self.textures[2].draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(-6.4 * (100.0 - self.health),0.0))
                .origin(Vec2::new(0.0,0.0))
                .scale(Vec2::new(1.0, 1.0))
        );
    }
}


/* Platform Collision


 */

/* enemy collision


*/