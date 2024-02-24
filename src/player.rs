use macroquad::math::Vec2;

use crate::game::{Collidable, Color, Controllable, Drawable, Friction, Game, Moveable, Rect, Tickable, Velocity};

pub struct Player {
    pub rect: macroquad::math::Rect,
    pub color: macroquad::color::Color,
    pub velocity: macroquad::math::Vec2,
    pub friction_coefficient: f32,
    pub up_bind: macroquad::input::KeyCode,
    pub down_bind: macroquad::input::KeyCode,
    pub left_bind: macroquad::input::KeyCode,
    pub right_bind: macroquad::input::KeyCode,
}

impl Player {
    fn attack(&mut self, game: &mut Game) {
        
        let attack_line = geo::Line::<f32>::from(
            [
                (self.get_rect().x, self.get_rect().y),
                (self.get_rect().x + 50.0, self.get_rect().y)
            ]
        );

        

        for player in game.players.iter_mut() {
        }
    }
}
impl Rect for Player {
    fn get_rect(&self) -> macroquad::math::Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: macroquad::math::Rect) {
        self.rect = rect;
    }
}

impl Color for Player {
    fn color(&self) -> macroquad::color::Color {
        self.color
    }
}

impl Velocity for Player {
    fn get_velocity(&self) -> macroquad::math::Vec2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: macroquad::math::Vec2) {
        self.velocity = velocity
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

impl Friction for Player {
    fn friction_coefficient(&self) -> f32 {
        self.friction_coefficient   
    }
}


impl Tickable for Player {
    fn tick(&mut self, game: &mut crate::game::Game) {
    
        self.control(game.dt);

        for player in game.players.iter_mut() {
            self.collide(player, game.dt);
        }
        
        self.move_by_velocity(game.dt);

        self.apply_friction(game.dt);

    }
}
