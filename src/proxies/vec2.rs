use diff::Diff;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Vec2 {
    x: f32,
    y: f32
}

impl From<macroquad::math::Vec2> for Vec2 {
    fn from(value: macroquad::math::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

impl Into<macroquad::math::Vec2> for Vec2 {
    fn into(self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2 {
            x: self.x,
            y: self.y
        }
    }
}