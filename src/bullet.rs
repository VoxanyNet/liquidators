use crate::game::{Collidable, Color, Controllable, Drawable, Friction, Moveable, Rect, Tickable, Velocity};

#[derive(Clone)]
pub struct Bullet {
    pub rect: macroquad::math::Rect,
    pub color: macroquad::color::Color,
    pub velocity: macroquad::math::Vec2
}

impl Rect for Bullet {
    fn rect(&mut self) -> &mut macroquad::math::Rect {
        &mut self.rect
    }
}

impl Color for Bullet {
    fn color(&self) -> macroquad::color::Color {
        self.color
    }
}

impl Velocity for Bullet {
    fn velocity(&mut self) -> &mut macroquad::math::Vec2 {
        &mut self.velocity
    }
}

impl Drawable for Bullet {}

impl Moveable for Bullet {}

impl Collidable for Bullet {}

impl Friction for Bullet {}


impl Tickable for Bullet {
    fn tick(&mut self, game: &mut crate::game::Game) {
        
        //println!("{}", self.velocity.x);

        self.move_by_velocity(game.dt);

        self.apply_friction(game.dt)

        // for player in game.players.iter_mut() {
        //     self.collide(player);
        // }

    }
}