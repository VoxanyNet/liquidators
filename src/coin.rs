use macroquad::math::Vec2;

use crate::{game::{Color, Drawable, Friction, Moveable, Rect, Tickable, Velocity}, player::Player};

pub struct Coin {
    color: macroquad::color::Color,
    rect: macroquad::math::Rect,
    velocity: Vec2,
    friction_coefficient: f32
}

impl Coin {
    pub fn new(x: f32, y: f32) -> Self {
        Coin {
            color: macroquad::color::YELLOW,
            rect: macroquad::math::Rect { x: x, y: y, w: 10.0, h: 10.0 },
            velocity: Vec2{x: 0.0, y: 0.0},
            friction_coefficient: 0.01
        }
    }

    pub fn gravitate(&mut self, player: &mut Player) {
        let distance = Vec2 {
            x: self.get_rect().center().x - player.get_rect().center().x,
            y: self.get_rect().center().y - player.get_rect().center().y
        };

        if distance.length() <= 300.0 {

            let exponential_factor = -0.0001 * distance.length();

            self.velocity += distance.normalize() * exponential_factor;

        }
    }
}

impl Moveable for Coin {}

impl Friction for Coin {
    fn friction_coefficient(&self) -> f32 {
        self.friction_coefficient
    }
}

impl Velocity for Coin {
    fn get_velocity(&self) -> macroquad::math::Vec2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: macroquad::math::Vec2) {
        self.velocity = velocity
    }
}

impl Tickable for Coin {
    fn tick(&mut self, game: &mut crate::game::Game) {
        for player in game.players.iter_mut() {
            self.gravitate(player);
        }

        self.move_by_velocity(game.dt);

        self.apply_friction(game.dt)
    }
}

impl Color for Coin {
    fn color(&self) -> macroquad::color::Color {  
        self.color
    }
}

impl Rect for Coin {
    fn get_rect(&self) -> macroquad::math::Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: macroquad::math::Rect) {
        self.rect = rect;
    }
}

impl Drawable for Coin {}

