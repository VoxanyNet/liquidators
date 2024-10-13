use diff::Diff;
use gamelibrary::rapier_mouse_world_pos;
use macroquad::math::{Rect, Vec2};
use serde::{Deserialize, Serialize};

use crate::portal_bullet::PortalBullet;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PortalGun {
    pub position: Vec2,
}

impl PortalGun {
    pub fn fire(&self, camera_rect: &Rect, portal_bullets: &mut Vec<PortalBullet>) {
        let mouse_pos = rapier_mouse_world_pos(camera_rect);

        let bullet_direction = (mouse_pos - self.position).normalize();

        let bullet = PortalBullet {
            position: self.position,
            direction: bullet_direction,
        };

        portal_bullets.push(bullet);

        
    }

    pub fn new() -> Self {
        Self {
            position: Vec2::ZERO,
        }
    }
}