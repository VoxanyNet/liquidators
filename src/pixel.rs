use std::hash::Hash;

use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}};
use macroquad::{color::Color, math::{vec2, Vec2}, shapes::{draw_rectangle, draw_rectangle_ex, DrawRectangleParams}, texture::draw_texture_ex};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};
use serde::{Deserialize, Serialize};

use crate::TickContext;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Pixel {
    color: Color,
    collider_handle: SyncColliderHandle,
    rigid_body_handle: SyncRigidBodyHandle,
    owner: String
}

// color doesnt derive hash so hopefully this isnt an issue!!!!!!!!!!
impl Hash for Pixel {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.collider_handle.hash(state);
        self.rigid_body_handle.hash(state);
        self.owner.hash(state);
    }
}

impl Eq for Pixel {

}

impl Pixel {
    pub fn new(
        color: Color, 
        pos: Vec2, 
        space: &mut Space, 
        mass: Option<f32>, 
        velocity: Option<Vec2>,
        owner: String
    ) -> Self {

        let velocity = match velocity {
            Some(velocity) => velocity,
            None => Vec2::ZERO,
        };

        let mass = match mass {
            Some(mass) => mass,
            None => 1.,
        };

        

        let rigid_body = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .ccd_enabled(true)
                .soft_ccd_prediction(20.)
                .position(vector![pos.x, pos.y].into())
                .additional_mass(mass)
                .linvel(vector![velocity.x, velocity.y])
        );

        let collider = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(1., 1.),
            rigid_body,
            &mut space.sync_rigid_body_set
        );

        
        
        Self {
            color,
            collider_handle: collider,
            rigid_body_handle: rigid_body,
            owner
        }

        
    }

    pub async fn draw(&self, space: &Space) {

        let body = space.sync_rigid_body_set.get_sync(self.rigid_body_handle).unwrap();

        let macroquad_pos = rapier_to_macroquad(&vec2(body.translation().x, body.translation().y));

        let shape = space.sync_collider_set.get_sync(self.collider_handle).unwrap().shape().as_cuboid().unwrap();


        draw_rectangle_ex(
            macroquad_pos.x, 
            macroquad_pos.y, 
            shape.half_extents.x * 2., 
            shape.half_extents.y * 2., 
            DrawRectangleParams { 
                offset: macroquad::math::Vec2::new(0.5, 0.5), 
                rotation: body.rotation().angle() * -1., 
                color: self.color 
            }
        );
    }

    pub fn tick(&self, ctx: &mut TickContext) {

        if self.owner != *ctx.uuid {
            return;
        }
        ctx.owned_colliders.push(self.collider_handle);
        ctx.owned_rigid_bodies.push(self.rigid_body_handle);
    }
}