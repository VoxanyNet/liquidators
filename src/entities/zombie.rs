use macroquad::math::Vec2;

use crate::game::{Rect, Scale, Texture};

pub struct Zombie {
    pub rect: macroquad::math::Rect,
    pub position: macroquad::math::Vec2,
    pub texture_path: String,
    pub scale: Vec2
}

impl Rect for Zombie {
    fn get_rect(&self) -> macroquad::math::Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: macroquad::math::Rect) {
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
    fn get_scale(&self) -> Vec2 {
        self.scale
    }
}