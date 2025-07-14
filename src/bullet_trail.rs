use diff::Diff;
use gamelibrary::{rapier_to_macroquad, time::Time};
use macroquad::{color::{self, Color, RED, WHITE}, math::Vec2, shapes::draw_line};
use serde::{Deserialize, Serialize};

use crate::TickContext;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone, Default)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct BulletTrail {
    start: Vec2,
    end: Vec2,
    color: Color,
    pub owner: String
}

impl BulletTrail {
    
    pub fn draw(&self) {

        let start_pos = rapier_to_macroquad(&self.start);
        let end_pos = rapier_to_macroquad(&self.end);

        draw_line(start_pos.x, start_pos.y, end_pos.x, end_pos.y, 5., self.color);
    }

    pub fn tick(&mut self, ctx: &TickContext) {

        self.color.a -= 0.3 * ctx.last_tick_duration.as_secs_f32();
    }

    pub fn new(
        start: Vec2,
        end: Vec2,
        color: Option<Color>,
        owner: String
    ) -> Self {

        let color = match color {
            Some(color) => color,
            None => {
                let mut color = WHITE.clone();
                color.a = 0.2;

                color
            },
        };
        Self {
            start,
            end,
            color,
            owner: owner.clone()
        }
    }
}