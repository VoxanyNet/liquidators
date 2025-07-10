use diff::Diff;
use gamelibrary::{space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, texture_loader::TextureLoader, traits::draw_texture_onto_physics_body};
use macroquad::math::Vec2;
use nalgebra::vector;
use parry2d::math::Vector;
use rapier2d::prelude::{ColliderBuilder, Group, InteractionGroups, RigidBody, RigidBodyBuilder};
use serde::{Deserialize, Serialize};

use crate::collider_groups::PARTICLES_GROUP;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone, Hash, Eq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct BulletCasing {
    rigid_body_handle: SyncRigidBodyHandle,
    collider_handle: SyncColliderHandle,
    sprite_path: String
}

impl BulletCasing {
    pub fn new(
        pos: Vec2, 
        size: Vec2, 
        sprite_path: String,
        space: &mut Space,
        mut velocity: Vector<f32>,
    ) -> Self {

        velocity.y += 150.;
        
        let body = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .position(vector![pos.x, pos.y].into())
                .linvel(velocity.into())
        );

        

        let interaction_groups = InteractionGroups::none()
            .with_filter(Group::ALL)
            .with_memberships(PARTICLES_GROUP);

        let collider = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(size.x / 2., size.y / 2.)
                .collision_groups(interaction_groups), // half extents!
            body, 
            &mut space.sync_rigid_body_set
        );

        Self {
            rigid_body_handle: body,
            collider_handle: collider,
            sprite_path,
        }
    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {
        draw_texture_onto_physics_body(
            self.rigid_body_handle, 
            self.collider_handle, 
            space, &self.sprite_path, 
            textures, 
            false, 
            false, 
            0.
        ).await
    }
}