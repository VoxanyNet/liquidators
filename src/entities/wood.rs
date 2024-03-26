use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::game::{Draggable, HasOwner, HasRect, Scale, Texture, Tickable, Velocity};
use crate::proxies::macroquad::math::{vec2::Vec2, rect::Rect};
use crate::proxies::uuid::lib::Uuid;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Wood {
    dragging: bool,
    velocity: Vec2,
    rect: Rect,
    texture_path: String,
    scale: u32,
    pub owner: Uuid
}

impl Wood {
    fn new(rect: Rect, owner_uuid: Uuid) -> Self {
        Self {
            dragging: false,
            velocity: Vec2::new(0., 0.),
            rect: rect,
            texture_path: "assets/wood.png".to_string(),
            scale: 2,
            owner: owner_uuid
        }
    }
}

impl HasOwner for Wood {
    fn get_owner(&self) -> crate::proxies::uuid::lib::Uuid {
        self.owner
    }

    fn set_owner(&mut self, uuid: Uuid) {
        self.owner = uuid
    }
}

impl Tickable for Wood {
    fn tick(&mut self, game: &mut crate::game::Game) {
        
    }
}

impl Texture for Wood {
    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }
    
    fn set_texture_path(&mut self, texture_path: String) {
        self.texture_path = texture_path;
    }
}

impl Scale for Wood {
    fn get_scale(&self) -> u32 {
        self.scale
    }
}

impl Draggable for Wood {
    fn get_dragging(&self) -> bool {
        self.dragging
    }

    fn set_dragging(&mut self, dragging: bool) {
        self.dragging = dragging;
    }
}

impl Velocity for Wood {
    fn get_velocity(&self) -> Vec2 {
        self.velocity
    }
    
    fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity;
    }
}

impl HasRect for Wood {
    fn get_rect(&self) -> Rect {
        self.rect
    }

    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}