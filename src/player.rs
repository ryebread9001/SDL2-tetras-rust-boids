use tetra::math::Vec2;
use tetra::graphics::mesh::Mesh;
use tetra::graphics::{self, Color, DrawParams, Rectangle, Texture};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::{time, Context};

use crate::WINDOW_HEIGHT;

use super::{queue::Queue, platform::Platform, enemy::Enemy, token::Token};

pub struct Player {
    simple: Mesh,
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
    score: u32
}

const EPS: f32 = 0.001;


impl Player {
    pub fn new(ctx: &mut Context, txts: Vec<Texture>, start_pos: Vec2<f32>, simple: Mesh) -> Player {

        let mut q = Queue::new(super::POSITION_BUFFER_SIZE);
        q.fill_with(start_pos);


        Player {
            simple: simple,
            textures: txts,
            position: start_pos,
            spawn_pos: start_pos,
            dimension: Vec2::new(super::PLAYER_WIDTH, super::PLAYER_HEIGHT),
            velocity: Vec2::new(0.0,0.0),
            pos_buf: q,
            health: 100.0,
            is_facing_left: false,
            is_jumping: false,
            is_wall_jumping: false,
            collision: false,
            move_speed: 1.2,
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

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn window_collision(&mut self) {
        if self.position.x < 0.0 {
            self.position.x = 0.0;
            self.velocity.x = 0.0;
        } else if self.position.x + self.dimension.x > super::WINDOW_WIDTH as f32 {
            self.position.x = super::WINDOW_WIDTH as f32 - self.dimension.x;
            self.velocity.x = 0.0;
        }
    }


    fn platform_collisions(&mut self, platforms: &[Platform]) {
        // x axis
        self.position.x += self.velocity.x;

        for platform in platforms {
            let pos = platform.get_pos();
            let dim = platform.get_dim();
            let (left, right) = (pos.x, pos.x + dim.x);
            let (top, bottom) = (pos.y, pos.y + dim.y);

            // Current Y (not future Y). EPS keeps floating-point rounding from
            // making a player standing on a platform look like it overlaps it.
            let overlaps_x = self.position.x + self.dimension.x > left && self.position.x < right;
            let overlaps_y = self.position.y + self.dimension.y > top + EPS
                && self.position.y < bottom - EPS;

            if overlaps_x && overlaps_y {
                if self.velocity.x > 0.0 {
                    self.position.x = left - self.dimension.x;
                } else if self.velocity.x < 0.0 {
                    self.position.x = right;
                }
                self.velocity.x = 0.0;
                if !self.is_wall_jumping {
                    self.is_wall_jumping = true;
                    self.is_jumping = false;
                }
            }
        }

        // y axis
        self.position.y += self.velocity.y;

        for platform in platforms {
            let pos = platform.get_pos();
            let dim = platform.get_dim();
            let (left, right) = (pos.x, pos.x + dim.x);
            let (top, bottom) = (pos.y, pos.y + dim.y);

            // X is already resolved, so use the final X.
            let overlaps_x = self.position.x + self.dimension.x > left + EPS
                && self.position.x < right - EPS;
            let overlaps_y = self.position.y + self.dimension.y > top && self.position.y < bottom;

            if overlaps_x && overlaps_y {
                if self.velocity.y > 0.0 {
                    self.position.y = top - self.dimension.y;
                    self.velocity.y = 0.0;
                    self.is_jumping = false;
                    self.is_wall_jumping = false;
                    self.collision = true;
                } else if self.velocity.y < 0.0 {
                    self.position.y = bottom;
                    self.velocity.y = 0.0;
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

    fn token_collisions(&mut self, tokens: &mut Vec<Token>) {
        for token in tokens {
            // formula for finding mean without some sample value, in this case, the current enemy boid pos
            // let center_minus_curr = ((n as f32 * center) - enemy.get_pos()) /  ((n - 1) as f32);
            let e_pos = token.get_pos();
            let e_dim = token.get_dim();

            let mut hit_y = false;
            let mut hit_x = false;
            // player-token collision detection
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
                println!("SET HIT");
                token.set_hit(true);
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

    pub fn update(&mut self, ctx: &mut Context, platforms: Vec<Platform>, enemies: Vec<Enemy>, tokens: & mut Vec<Token>) {
        let frame_scale = {
            let dt = time::get_delta_time(ctx).as_secs_f32();
            if dt > 0.0 { dt * 60.0 } else { 1.0 }
        };

        if self.position.y < WINDOW_HEIGHT as f32 + self.dimension.y {
            self.velocity.y += 0.75 * frame_scale;
        } else {
            self.health = 0.0;
        }
        self.velocity.x *= 0.8_f32.powf(frame_scale);

        if self.position.y + self.velocity.y < 0.0 {
            self.velocity.y = 0.0;
        }
        
        if input::is_key_down(ctx, Key::A) || input::is_key_down(ctx, Key::Left) {
            self.velocity.x -= self.move_speed * frame_scale;
            self.is_facing_left = true;
        }

        if input::is_key_down(ctx, Key::D) || input::is_key_down(ctx, Key::Right) {
            self.velocity.x += self.move_speed * frame_scale;
            self.is_facing_left = false;

        }

        if !self.is_jumping && (input::is_key_pressed(ctx, Key::W) || input::is_key_pressed(ctx, Key::Up)) {
            self.velocity.y -= 15.0 * frame_scale;
            self.is_jumping = true;
        }

        if input::is_key_down(ctx, Key::S) || input::is_key_down(ctx, Key::Down) {
            self.velocity.y += 0.4 * frame_scale;
        }

        if input::is_key_pressed(ctx, Key::R) {
            self.health = 100.0;
            self.score = 0;
            self.position = self.spawn_pos;
        }

        self.window_collision();
        self.platform_collisions(&platforms);
        self.enemy_collisions(enemies);
        self.token_collisions(tokens);

        self.pos_buf.push(self.position);
        if self.pos_buf.get_items_in_q() > super::POSITION_BUFFER_SIZE - 1 {
            self.pos_buf.pop();
        }
    }

    pub fn draw(&mut self, ctx: &mut Context) -> Result<(), Box<dyn std::error::Error>> {

        // LEFT RIGHT PLAYER TEXTURES
        if self.health > 0.0 {
            self.simple.draw(ctx, self.position);

            if self.is_facing_left {
                self.textures[0].draw(
                    ctx,
                    DrawParams::new()
                        .position(self.position)
                        .origin(Vec2::new(0.0, 0.0))
                        .scale(Vec2::new(2.0, 2.0)),
                );
            } else {
                self.textures[1].draw(
                    ctx,
                    DrawParams::new()
                        .position(self.position)
                        .origin(Vec2::new(0.0, 0.0))
                        .scale(Vec2::new(2.0, 2.0)),
                );
            }
        } else { // WOMP WOMP
            graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));
            let content = format!("     YOU DIED!\nPRESS \"R\" TO RESTART\n\n SCORE: {}", self.score);
            let mut restart_text = Text::new(
                content,
                Font::bmfont(ctx, "./resources/DejaVuSansMono.fnt")?,
            );
            restart_text.draw(ctx, Vec2::new(super::WINDOW_WIDTH as f32 /2.0 - 90.0, super::WINDOW_HEIGHT as f32 / 2.0 - 48.0));
            // self.score = 0;
        }

        // HEALTH BAR
        self.textures[2].draw(
            ctx,
            DrawParams::new()
                .position(Vec2::new(-6.4 * (100.0 - self.health),0.0))
                .origin(Vec2::new(0.0,0.0))
                .scale(Vec2::new(1.0, 1.0))
        );
        Ok(())
    }
}