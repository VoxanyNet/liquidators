

use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::game::{HasOwner, HasRect, Scale, Texture, Tickable};
use crate::proxies::macroquad::math::rect::Rect;
use crate::proxies::uuid::lib::Uuid;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Zombie {
    pub rect: Rect,
    pub position: crate::proxies::macroquad::math::vec2::Vec2,
    pub texture_path: String,
    pub scale: u32,
    pub owner: Uuid
}

impl HasRect for Zombie {
    fn get_rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl HasOwner for Zombie {
    fn get_owner(&self) -> crate::proxies::uuid::lib::Uuid {
        self.owner
    }
    
    fn set_owner(&mut self, uuid: Uuid) {
        self.owner = uuid
    }
}
impl Tickable for Zombie {
    fn tick(&mut self, game: &mut crate::game::Game) {
        
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