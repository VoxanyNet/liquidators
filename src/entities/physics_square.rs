use core_lib::collider::Collider;
use core_lib::proxies::macroquad::color::colors::WHITE;
use core_lib::proxies::macroquad::math::vec2::Vec2;
use core_lib::rigid_body::{RigidBody, RigidBodyType};
use core_lib::space::{RigidBodyHandle, Space};
use core_lib::traits::{Color, HasOwner, HasRigidBody};
use diff::Diff;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::window;
use serde::{Deserialize, Serialize};

use crate::traits::{IsClient, Tickable};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PhysicsSquare {
    pub scale: u32,
    pub color: core_lib::proxies::macroquad::color::Color,
    pub owner: String,
    pub rigid_body_handle: RigidBodyHandle,
    pub controllable: bool
}

impl PhysicsSquare {
    pub fn new(space: &mut Space, position: Vec2, body_type: RigidBodyType, hx: f32, hy: f32, owner: &String, controlable: bool) -> Self {
        let rigid_body_handle = space.insert_rigid_body(
            RigidBody { 
                position: position, 
                rotation: 0.,
                angular_velocity: 0.,
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
            controllable: controlable
        }
        
    }
}

impl Color for PhysicsSquare {
    fn color(&self) -> core_lib::proxies::macroquad::color::Color {
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
    fn tick(&mut self, client: &mut dyn IsClient) {

        let rigid_body = client.get_game_state().space.get_rigid_body_mut(self.get_rigid_body_handle()).expect("shit");

        if rigid_body.position.x >= window::screen_width() || rigid_body.position.x <= 0. {
            rigid_body.velocity.x = rigid_body.velocity.x * -1.;
        }

        if rigid_body.position.y >= window::screen_height() || rigid_body.position.y <= 0. {
            rigid_body.velocity.y = rigid_body.velocity.y * -1.;
        }

        if self.controllable {
            let rigid_body = client.get_game_state().space.get_rigid_body_mut(self.get_rigid_body_handle()).expect("shit");

            if is_key_down(KeyCode::W) {

                if rigid_body.velocity.y.is_sign_negative() {
                    rigid_body.velocity.y = 0.
                }

                rigid_body.velocity.y += 4.
            }

            if is_key_down(KeyCode::S) {

                if rigid_body.velocity.y.is_sign_positive() {
                    rigid_body.velocity.y = 0.
                }

                rigid_body.velocity.y -= 4.
            }
            
            if is_key_down(KeyCode::A) {

                if rigid_body.velocity.x.is_sign_positive() {
                    rigid_body.velocity.x = 0.
                }

                rigid_body.velocity.x -= 4.
            }

            if is_key_down(KeyCode::D) {

                if rigid_body.velocity.x.is_sign_negative() {
                    rigid_body.velocity.x = 0.
                }

                rigid_body.velocity.x += 4.
            }
        }
    }
}
