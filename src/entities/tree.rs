use macroquad::math;

use crate::game::{Breakable, Damagable, Rect, Scale, Texture, Tickable};

pub struct Tree {
    texture_path: String,
    scale: u32,
    rect: math::Rect,
    highlighted: bool,
    health: i32
}

impl Tree {
    pub fn new(rect: math::Rect) -> Self{
        Self {
            texture_path: "assets/structure/tree.png".to_string(),
            scale: 2,
            rect: rect,
            highlighted: false,
            health: 100
        }
    }  
}

impl Texture for Tree {
    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }

    fn set_texture_path(&mut self, texture_path: String) {
        self.texture_path = texture_path
    }
}

impl Rect for Tree {
    fn get_rect(&self) -> macroquad::math::Rect {
        self.rect
    }

    fn set_rect(&mut self, rect: macroquad::math::Rect) {
        self.rect = rect;
    }
}

impl Scale for Tree {
    fn get_scale(&self) -> u32 {
        self.scale
    }

}

impl Damagable for Tree {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn set_health(&mut self, health: i32) {
        self.health = health
    }
}

impl Breakable for Tree {
    fn get_highlighted(&self) -> bool {
        self.highlighted
    }

    fn set_highlighted(&mut self, highlighted: bool) {
        self.highlighted = highlighted
    }
}

impl Tickable for Tree {
    fn tick(&mut self, game: &mut crate::game::Game) {

        self.highlight()
        
    }
}