

use crate::game::{HasRect, Scale, Texture};
use crate::proxies::macroquad::math::rect::Rect;

pub struct Zombie {
    pub rect: Rect,
    pub position: macroquad::math::Vec2,
    pub texture_path: String,
    pub scale: u32
}

impl HasRect for Zombie {
    fn get_rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl Texture for Zombie {
    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }

    fn set_texture_path(&mut self, texture_path: String) {
        self.texture_path = texture_path
    }
}

impl Scale for Zombie {
    fn get_scale(&self) -> u32 {
        self.scale
    }
}