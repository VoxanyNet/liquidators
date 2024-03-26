use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::game::{Collidable, Color, Drawable, Friction, HasOwner, HasRect, Moveable, Tickable, Velocity};
use crate::proxies::macroquad::math::{vec2::Vec2, rect::Rect};
use crate::proxies::uuid::lib::Uuid;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Bullet {
    pub rect: Rect,
    pub color: crate::proxies::macroquad::color::Color,
    pub velocity: Vec2,
    pub friction_coefficient: f32,
    pub owner: Uuid
}

impl HasRect for Bullet {
    fn get_rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect
    }
}

impl Color for Bullet {
    fn color(&self) -> crate::proxies::macroquad::color::Color {
        self.color
    }
}

impl Velocity for Bullet {
    fn get_velocity(&self) -> Vec2 {
        self.velocity
    }
    fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}

impl Drawable for Bullet {}

impl Moveable for Bullet {}

impl Collidable for Bullet {}

impl Friction for Bullet {
    fn friction_coefficient(&self) -> f32 {
        self.friction_coefficient
    }
}

impl HasOwner for Bullet {
    fn get_owner(&self) -> Uuid {
        self.owner
    }
    fn set_owner(&mut self, uuid: Uuid) {
        self.owner = uuid
    }
}
impl Tickable for Bullet {
    fn tick(&mut self, game: &mut crate::game::Game) {
        
        //println!("{}", self.velocity.x);

        self.move_by_velocity(game.last_tick.elapsed());

        self.apply_friction(game.last_tick.elapsed())

        // for player in game.players.iter_mut() {
        //     self.collide(player);
        // }

    }
}