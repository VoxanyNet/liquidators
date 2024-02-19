use crate::game::{Color, Rect, Drawable};

pub struct Zombie {
    pub rect: macroquad::math::Rect,
    pub position: macroquad::math::Vec2,
    pub color: macroquad::color::Color
}

impl Rect for Zombie {
    fn rect(&self) -> macroquad::math::Rect {
        self.rect
    }
}

impl Color for Zombie {
    fn color(&self) -> macroquad::color::Color {
        self.color
    }
}

impl Drawable for Zombie {}