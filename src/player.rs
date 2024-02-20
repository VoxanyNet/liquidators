use std::time::Instant;

use crate::game::{Collidable, Color, Controllable, Drawable, Friction, Moveable, Rect, Tickable, Velocity};

pub struct Player {
    pub rect: macroquad::math::Rect,
    pub color: macroquad::color::Color,
    pub velocity: macroquad::math::Vec2,
    pub up_bind: macroquad::input::KeyCode,
    pub down_bind: macroquad::input::KeyCode,
    pub left_bind: macroquad::input::KeyCode,
    pub right_bind: macroquad::input::KeyCode,
}

impl Rect for Player {
    fn rect(&mut self) -> &mut macroquad::math::Rect {
        &mut self.rect
    }
}

impl Color for Player {
    fn color(&self) -> macroquad::color::Color {
        self.color
    }
}

impl Velocity for Player {
    fn velocity(&mut self) -> &mut macroquad::math::Vec2 {
        &mut self.velocity
    }
}

impl Drawable for Player {}

impl Moveable for Player {}

impl Collidable for Player {}

impl Controllable for Player {
    fn up_bind(&mut self) -> macroquad::input::KeyCode {
        self.up_bind
    }
    fn down_bind(&mut self) -> macroquad::input::KeyCode {
        self.down_bind
    }
    fn left_bind(&mut self) -> macroquad::input::KeyCode {
        self.left_bind
    }
    fn right_bind(&mut self) -> macroquad::input::KeyCode {
        self.right_bind
    }
}

impl Friction for Player {}


impl Tickable for Player {
    fn tick(&mut self, game: &mut crate::game::Game) {
    
        self.control(game.dt);

        self.move_by_velocity(game.dt);

        self.apply_friction(game.dt);

        for player in game.players.iter_mut() {
            self.collide(player);
        }

        

    }
}
