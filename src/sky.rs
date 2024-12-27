use diff::Diff;
use macroquad::{color::Color, window::clear_background};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Sky {
    
}

impl Sky {

    pub fn new() -> Self {
        Self {

        }
    }
    pub fn draw(&self) {
        clear_background(Color::from_rgba(52, 96, 191, 255));
    }
}