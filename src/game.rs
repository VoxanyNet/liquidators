use std::time::Duration;

use crate::player::{self, Player};
use crate::zombie::Zombie;

pub trait Velocity {
    fn velocity(&self) -> macroquad::math::Vec2;
}

pub trait Collidable: Rect + Velocity {
    fn collide(&mut self, collider: &dyn Collidable) {
        if collider.rect().overlaps(&self.rect()) {
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
        if macroquad::input::is_key_down(macroquad::input::KeyCode::D) {
            self.velocity().x += 0.01 * dt.as_millis() as f32
        }

        if macroquad::input::is_key_down(macroquad::input::KeyCode::A) {
            self.velocity().x -= 0.01 * dt.as_millis() as f32
        }

        if macroquad::input::is_key_down(macroquad::input::KeyCode::W) {
            self.velocity().y -= 0.01 * dt.as_millis() as f32
        }

        if macroquad::input::is_key_down(macroquad::input::KeyCode::S) {
            self.velocity().y += 0.01 * dt.as_millis() as f32
        }
    }
}

pub trait Moveable: Rect + Velocity {
    fn move_by_velocity(&mut self, dt: Duration) {
        self.rect().x += self.velocity().x * dt.as_millis() as f32;
        self.rect().y += self.velocity().y * dt.as_millis() as f32;
    }
}
pub trait Rect {
    fn rect(&self) -> macroquad::math::Rect;
}

pub trait Color {
    fn color(&self) -> macroquad::color::Color;
}

pub trait Drawable: Rect + Color {
    fn draw(&self) {
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
    pub fn draw(&self) {
        for player in self.players.iter() {
            player.draw();
        }
    }

    pub fn tick(&self) {
        for player in self.players.iter() {
            player.tick(&mut self);
        }
    }
}