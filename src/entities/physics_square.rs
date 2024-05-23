use diff::Diff;
use serde::{Deserialize, Serialize};

use crate::collider::Collider;
use crate::game::{Color, HasOwner, HasRigidBody, Tickable};
use crate::proxies::macroquad::color::colors::WHITE;
use crate::proxies::macroquad::math::rect::Rect;

use crate::proxies::macroquad::math::vec2::Vec2;
use crate::rigid_body::{RigidBody, RigidBodyType};
use crate::space::{RigidBodyHandle, Space};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PhysicsSquare {
    pub scale: u32,
    pub color: crate::proxies::macroquad::color::Color,
    pub owner: String,
    pub rigid_body_handle: RigidBodyHandle,
    pub follow_cursor: bool
}

impl PhysicsSquare {
    pub fn new(space: &mut Space, position: Vec2, body_type: RigidBodyType, hx: f32, hy: f32, owner: &String, follow_cursor: bool) -> Self {
        let rigid_body_handle = space.insert_rigid_body(
            RigidBody { 
                position: position, 
                velocity: Vec2::new(0., 0.), 
                body_type: body_type, 
                owner: owner.clone(), 
                collider: Collider { 
                    hx: hx, 
                    hy: hy, 
                    restitution: 0., 
                    mass: 10., 
                    owner: owner.clone() 
                }
            }
        );

        Self {
            scale: 1,
            color: WHITE,
            owner: owner.clone(),
            rigid_body_handle,
            follow_cursor: follow_cursor
        }
        
    }
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
        if self.follow_cursor {
            let rigid_body = context.game_state.space.get_rigid_body_mut(self.get_rigid_body_handle()).expect("shit");

            rigid_body.position = Vec2::new(
                macroquad::input::mouse_position().0,
                macroquad::input::mouse_position().1 * -1. + macroquad::window::screen_height()
            );
        }
    }
}
