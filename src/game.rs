use std::collections::HashMap;
use std::time::Duration;

use macroquad::color::{RED, WHITE};
use macroquad::math::Vec2;
use macroquad::texture::{self, load_texture, Texture2D};

use crate::coin::Coin;
use crate::player::Player;
use crate::zombie::Zombie;

pub trait Velocity {
    fn get_velocity(&self) -> macroquad::math::Vec2;
    fn set_velocity(&mut self, velocity: macroquad::math::Vec2);
}

pub trait Damagable {
    fn damage(&mut self, damage: i32) {
        self.set_health(
            self.get_health() - damage
        )
    }

    fn get_health(&self) -> i32;
    fn set_health(&mut self, health: i32);
}

pub trait Breakable: Damagable + Rect {
    fn highlight(&mut self) {
        let mouse_pos = Vec2::from(macroquad::input::mouse_position());

        if self.get_rect().contains(mouse_pos) {
            self.set_highlighted(true);
        }

        else {
            self.set_highlighted(false);
        }
    }

    fn get_highlighted(&self) -> bool;
    fn set_highlighted(&mut self, highlighted: bool) -> bool;
}

pub trait Item { 
}
pub trait Collidable: Rect + Velocity {

    fn collide(&mut self, collider: &mut dyn Collidable, dt: Duration) {

        // check where our rect will be when it next moves
        let mut next_rect = self.get_rect().clone();

        next_rect.x += self.get_velocity().x * dt.as_millis() as f32;
        next_rect.y += self.get_velocity().y * dt.as_millis() as f32;

        if collider.get_rect().overlaps(&mut next_rect) {
            
            // add our velocity to the collider
            collider.set_velocity(
                collider.get_velocity() + self.get_velocity()
            );

            // invert current velocity
            self.set_velocity(
                self.get_velocity() * -0.05
            );

        }

    }
}
pub trait Friction: Rect + Velocity {
    fn apply_friction(&mut self, dt: Duration) {

        self.set_velocity(
            self.get_velocity() + ((-self.get_velocity() * self.friction_coefficient()) * dt.as_millis() as f32)
        );
    }

    fn friction_coefficient(&self) -> f32;
}

pub trait Controllable: Rect + Velocity {
    fn control(&mut self, dt: Duration) {

        let mut velocity = self.get_velocity();
        let acceleration = self.get_acceleration();

        if macroquad::input::is_key_down(self.right_bind()) {
            velocity.x += acceleration * dt.as_millis() as f32;
        }

        if macroquad::input::is_key_down(self.left_bind()) {
            velocity.x -= acceleration * dt.as_millis() as f32

        }

        if macroquad::input::is_key_down(self.up_bind()) {
            velocity.y -= acceleration * dt.as_millis() as f32
        }

        if macroquad::input::is_key_down(self.down_bind()) {
            velocity.y += acceleration * dt.as_millis() as f32
        }

        // update to the final velocity
        self.set_velocity(
            velocity
        );

    }

    fn get_acceleration(&self) -> f32;

    fn up_bind(&mut self) -> macroquad::input::KeyCode;
    fn down_bind(&mut self) -> macroquad::input::KeyCode;
    fn left_bind(&mut self) -> macroquad::input::KeyCode;
    fn right_bind(&mut self) -> macroquad::input::KeyCode;
}

pub trait Moveable: Rect + Velocity {
    fn move_by_velocity(&mut self, dt: Duration) {

        let mut rect = self.get_rect();

        rect.x += self.get_velocity().x * dt.as_millis() as f32;
        rect.y += self.get_velocity().y * dt.as_millis() as f32;

        self.set_rect(rect);
    }
}
pub trait Rect {
    fn get_rect(&self) -> macroquad::math::Rect;
    fn set_rect(&mut self, rect: macroquad::math::Rect);
}

pub trait Color {
    fn color(&self) -> macroquad::color::Color;
}

pub trait Drawable: Rect + Color {
    fn draw(&mut self) {
        macroquad::shapes::draw_rectangle(self.get_rect().x, self.get_rect().y, self.get_rect().w, self.get_rect().h, self.color());
    }
}

pub trait Texture: Rect + Scale {
    async fn draw(&mut self, textures: &mut HashMap<String, Texture2D>) {

        // load texture if not already
        if !textures.contains_key(&self.get_texture_path()) {
            let texture = load_texture(&self.get_texture_path()).await.unwrap();

            textures.insert(self.get_texture_path(), texture);
        }

        let texture = textures.get(&self.get_texture_path()).unwrap();

        texture.set_filter(texture::FilterMode::Nearest);

        let scaled_texture_size = Vec2 {
            x: texture.width() * self.get_scale().x,
            y: texture.height() * self.get_scale().y
        };

        // macroquad::shapes::draw_rectangle(
        //     self.get_rect().x,
        //     self.get_rect().y,
        //     self.get_rect().w, 
        //     self.get_rect().h,
        //     RED
        // );

        macroquad::texture::draw_texture_ex(
            texture,
            self.get_rect().x,
            self.get_rect().y,
            WHITE,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(scaled_texture_size),
                ..Default::default()
            },
         );

    }

    fn get_texture_path(&self) -> String;
}

pub trait Tickable {
    fn tick(&mut self, game: &mut Game);
}

pub trait Scale {
    fn get_scale(&self) -> Vec2;
}

pub struct Game {
    pub players: Vec<Player>,
    pub zombies: Vec<Zombie>,
    pub dt: Duration,
    pub coins: Vec<Coin>,
    pub textures: HashMap<String, Texture2D>
}

impl Game {
    pub async fn draw(&mut self) {
        for player in self.players.iter_mut() {
            player.draw(&mut self.textures).await;
        }

        for coin in self.coins.iter_mut() {
            coin.draw();
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.players.len() {

            // take the player out, tick it, then put it back in
            let mut player = self.players.swap_remove(i);

            player.tick(self);

            self.players.push(player);

        }

        for i in 0..self.coins.len() {

            // take the player out, tick it, then put it back in
            let mut coin = self.coins.swap_remove(i);

            coin.tick(self);

            self.coins.push(coin);


        }
    }
}