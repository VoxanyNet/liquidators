use diff::Diff;
use gamelibrary::{space::Space, traits::draw_hitbox};
use macroquad::{color::{RED, WHITE}, math::Vec2};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{collider_contains_point, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Teleporter {
    body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    // seperate collider for checking for valid teleport candidates
    teleport_candidate_collider: ColliderHandle,
    owner: String
}

impl Teleporter {

    pub fn new(pos: Vec2, space: &mut Space, owner: &String) -> Self {
        let teleporter_body_handle = space.rigid_body_set.insert(
            RigidBodyBuilder::dynamic()
                .position(vector![pos.x, pos.y].into())
                .build()
        );

        // the collider for the actual teleporter, not the candidate field
        let teleporter_collider_handle = space.collider_set.insert_with_parent(
            ColliderBuilder::cuboid(20., 5.)
                .position(vector![0., 0.].into()), 
            teleporter_body_handle, 
            &mut space.rigid_body_set
        );

        // the collider for the candidate field
        let teleport_candidate_collider_handle = space.collider_set.insert_with_parent(
            ColliderBuilder::cuboid(20., 20.)
                .position(vector![0., 0.].into()), 
            teleporter_body_handle, 
            &mut space.rigid_body_set
        );

        Self {
            body_handle: teleporter_body_handle,
            collider_handle: teleporter_collider_handle,
            teleport_candidate_collider: teleport_candidate_collider_handle,
            owner: owner.clone(),
        }
        
    }
    pub fn tick(&mut self, ctx: &mut TickContext, space: &mut Space) {
        
        self.all_tick(ctx);

        if *ctx.uuid == self.owner {
            self.owner_tick(ctx, space);
        }
        
    }

    pub fn all_tick(&mut self, ctx: &mut TickContext) {
        
    }

    pub fn owner_tick(&mut self, ctx: &mut TickContext, space: &mut Space) {
        ctx.owned_colliders.push(self.collider_handle);
        ctx.owned_colliders.push(self.teleport_candidate_collider);

        ctx.owned_rigid_bodies.push(self.body_handle);
    }

    pub fn draw(&self, space: &Space) {
        
        draw_hitbox(space, self.body_handle, self.teleport_candidate_collider, RED);
        draw_hitbox(space, self.body_handle, self.collider_handle, WHITE);
        
    }
}