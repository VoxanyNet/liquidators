use std::time::Duration;

use crate::player::Player;
use crate::zombie::Zombie;

pub trait Velocity {
    fn velocity(&mut self) -> &mut macroquad::math::Vec2;
}

pub trait Collidable: Rect + Velocity {
    fn collide(&mut self, collider: &mut dyn Collidable) {
        if collider.rect().overlaps(&mut self.rect()) {
            self.velocity().x *= -0.5;
            self.velocity().y *= -0.5;
        }
    }
}
pub trait Friction: Rect + Velocity {
    fn apply_friction(&mut self, dt: Duration) {
        self.velocity().x += (-self.velocity().x * 0.01)  * dt.as_millis() as f32;

        self.velocity().y += (-self.velocity().y * 0.01) * dt.as_millis() as f32;
    }
}

pub trait Controllable: Rect + Velocity {
    fn control(&mut self, dt: Duration) {

        if macroquad::input::is_key_down(self.right_bind()) {
            self.velocity().x += 0.01 * dt.as_millis() as f32;
        }

        if macroquad::input::is_key_down(self.left_bind()) {
            self.velocity().x -= 0.01 * dt.as_millis() as f32
        }

        if macroquad::input::is_key_down(self.up_bind()) {
            self.velocity().y -= 0.01 * dt.as_millis() as f32
        }

        if macroquad::input::is_key_down(self.down_bind()) {
            self.velocity().y += 0.01 * dt.as_millis() as f32
        }
    }

    fn up_bind(&mut self) -> macroquad::input::KeyCode;
    fn down_bind(&mut self) -> macroquad::input::KeyCode;
    fn left_bind(&mut self) -> macroquad::input::KeyCode;
    fn right_bind(&mut self) -> macroquad::input::KeyCode;
}

pub trait Moveable: Rect + Velocity {
    fn move_by_velocity(&mut self, dt: Duration) {

        self.rect().x += self.velocity().x * dt.as_millis() as f32;
        self.rect().y += self.velocity().y * dt.as_millis() as f32;
    }
}
pub trait Rect {
    fn rect(&mut self) -> &mut macroquad::math::Rect;
}

pub trait Color {
    fn color(&self) -> macroquad::color::Color;
}

pub trait Drawable: Rect + Color {
    fn draw(&mut self) {
        macroquad::shapes::draw_rectangle(self.rect().x, self.rect().y, self.rect().w, self.rect().h, self.color());
    }
}

pub trait Tickable {
    fn tick(&mut self, game: &mut Game);
}

pub struct Game {
    pub players: Vec<Player>,
    pub zombies: Vec<Zombie>,
    pub dt: Duration
}

impl Game {
    pub fn draw(&mut self) {
        for player in self.players.iter_mut() {
            player.draw();
        }
    }

    pub fn tick(&mut self) {
        for i in 0..self.players.len() {

            // take the player out, tick it, then put it back in
            let mut player = self.players.swap_remove(i);

            player.tick(self);

            self.players.push(player);


        }
    }
}