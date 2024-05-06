use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::game::{Color, Drawable, HasOwner, HasRect, Tickable};
use crate::proxies::macroquad::math::rect::Rect;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PhysicsSquare {
    scale: u32,
    rect: Rect,
    color: crate::proxies::macroquad::color::Color,
    pub owner: String
}

impl Color for PhysicsSquare {
    fn color(&self) -> crate::proxies::macroquad::color::Color {
        self.color
    }
}

impl HasOwner for PhysicsSquare {
    fn get_owner(&self) -> String {
        self.owner.clone()
    }

    fn set_owner(&mut self, uuid: String) {
        self.owner = uuid
    }
}

impl HasRect for PhysicsSquare {
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl Tickable for PhysicsSquare {
    fn tick(&mut self, context: &mut crate::game::TickContext) {
        
    }
}

impl Drawable for PhysicsSquare {}
