use std::hash::Hash;

use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}};
use macroquad::{color::{Color, RED}, math::{vec2, Vec2}, shapes::{draw_rectangle, draw_rectangle_ex, DrawRectangleParams}, texture::draw_texture_ex};
use nalgebra::vector;
use parry2d::math::{Isometry, Vector};
use rapier2d::prelude::{ColliderBuilder, Group, InteractionGroups, RigidBodyBuilder};
use serde::{Deserialize, Serialize};

use crate::{collider_groups::PARTICLES_GROUP, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Blood {
    color: Color,
    collider_handle: SyncColliderHandle,
    rigid_body_handle: SyncRigidBodyHandle,
    owner: String
}

// color doesnt derive hash so hopefully this isnt an issue!!!!!!!!!!
impl Hash for Blood {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.collider_handle.hash(state);
        self.rigid_body_handle.hash(state);
        self.owner.hash(state);
    }
}

impl Eq for Blood {

}

impl Blood {
    pub fn new(
        pos: Isometry<f32>, 
        space: &mut Space, 
        velocity: Option<Vector<f32>>,
        owner: String
    ) -> Self {

        let velocity = match velocity {
            Some(velocity) => velocity,
            None => Vector::zeros(),
        };

        

        let interaction_groups = InteractionGroups::none()
            .with_filter(Group::ALL)
            .with_memberships(PARTICLES_GROUP);

        let rigid_body = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .ccd_enabled(true)
                .soft_ccd_prediction(20.)
                .position(vector![pos.translation.x, pos.translation.y].into())
                .linvel(vector![velocity.x, velocity.y])
        );

        let collider = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(1., 1.)
                .collision_groups(interaction_groups),
            rigid_body,
            &mut space.sync_rigid_body_set
        );

        
        
        Self {
            color: RED,
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
}