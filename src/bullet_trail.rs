use diff::Diff;
use macroquad::{color::{RED, WHITE}, math::Vec2, shapes::draw_line};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone, Default)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct BulletTrail {
    start: Vec2,
    end: Vec2
}

impl BulletTrail {
    
    pub fn draw(&self) {

        // brain damage gaming
        let mut color = RED.clone();
        color.a = 0.2;

        draw_line(self.start.x, self.start.y, self.end.x, self.end.y, 5., color);
    }

    pub fn new(
        start: Vec2,
        end: Vec2
    ) -> Self {
        Self {
            start,
            end,
        }
    }
}