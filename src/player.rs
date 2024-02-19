use crate::game::{Collidable, Color, Drawable, Moveable, Rect, Tickable, Velocity, Controllable};

pub struct Player {
    pub rect: macroquad::math::Rect,
    pub color: macroquad::color::Color,
    pub velocity: macroquad::math::Vec2

}

impl Rect for Player {
    fn rect(&self) -> macroquad::math::Rect {
        self.rect
    }
}

impl Color for Player {
    fn color(&self) -> macroquad::color::Color {
        self.color
    }
}

impl Velocity for Player {
    fn velocity(&self) -> macroquad::math::Vec2 {
        self.velocity
    }
}

impl Drawable for Player {}

impl Moveable for Player {}

impl Collidable for Player {}

impl Controllable for Player {}


impl Tickable for Player {
    fn tick(&mut self, game: &mut crate::game::Game) {

        self.control(game.dt);

        for player in game.players.iter_mut() {
            self.collide(player);
        }

    }
}