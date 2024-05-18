use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::game::{Color, HasOwner, HasRigidBody, Tickable};
use crate::proxies::macroquad::math::rect::Rect;

use crate::space::RigidBodyHandle;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PhysicsSquare {
    pub scale: u32,
    pub rect: Rect,
    pub color: crate::proxies::macroquad::color::Color,
    pub owner: String,
    pub rigid_body_handle: RigidBodyHandle
}

impl Color for PhysicsSquare {
    fn color(&self) -> crate::proxies::macroquad::color::Color {
        self.color
    }
}

impl HasRigidBody for PhysicsSquare {

    fn get_rigid_body_handle(&self) -> &RigidBodyHandle {
        &self.rigid_body_handle
    }
}

impl HasOwner for PhysicsSquare {
    fn get_owner(&self) -> String {
        self.owner.clone()
    }

    fn set_owner(&mut self, uuid: String) {
        self.owner = uuid
    }
}

impl Tickable for PhysicsSquare {
    fn tick(&mut self, context: &mut crate::game::TickContext) {
        
    }
}
