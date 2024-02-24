use macroquad::{color::RED, input::{self, MouseButton}, math::Vec2};

use crate::game::{Collidable, Color, Controllable, Damagable, Drawable, Friction, Game, Moveable, Rect, Tickable, Velocity};

pub struct Player {
    pub rect: macroquad::math::Rect,
    pub color: macroquad::color::Color,
    pub velocity: macroquad::math::Vec2,
    pub friction_coefficient: f32,
    pub health: i32,
    pub up_bind: macroquad::input::KeyCode,
    pub down_bind: macroquad::input::KeyCode,
    pub left_bind: macroquad::input::KeyCode,
    pub right_bind: macroquad::input::KeyCode,
}

impl Player {
    fn attack(&mut self, game: &mut Game) {
        
        let attack_hitbox = macroquad::math::Rect::new(self.get_rect().right(), self.get_rect().y, 30.0, 50.0);

        macroquad::shapes::draw_rectangle(attack_hitbox.x, attack_hitbox.y, attack_hitbox.w, attack_hitbox.h, RED);

        for player in game.players.iter_mut() {
            if attack_hitbox.overlaps(&player.rect) {
                player.damage(2);

                println!("{}", player.get_health());
            }
        }
    }
}

impl Damagable for Player {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn set_health(&mut self, health: i32) {
        self.health = health
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
        
        if input::is_mouse_button_pressed(MouseButton::Left) {
            self.attack(game)
        }

        self.move_by_velocity(game.dt);

        self.apply_friction(game.dt);

    }
}
