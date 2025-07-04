use diff::Diff;
use gamelibrary::{space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, texture_loader::TextureLoader, traits::draw_texture_onto_physics_body};
use macroquad::math::Vec2;
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::TickContext;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct BodyPart {
    pub collider_handle: SyncColliderHandle,
    pub body_handle: SyncRigidBodyHandle,
    pub sprite_path: String,
    pub scale: u16,
    pub owner: String
}

impl BodyPart {

    pub fn new(
        sprite_path: String, 
        scale: u16, 
        mass: f32,
        pos: Vec2,
        space: &mut Space,
        textures: &mut TextureLoader,
        owner: String
    ) -> Self {

        // use the dimensions of the texture to create the collider
        let texture_size = futures::executor::block_on(textures.get(&sprite_path)).size();

        let rigid_body_handle = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .position(
                    vector![pos.x, pos.y].into()
                )
                .ccd_enabled(true)
                .build()
        );

        let collider_handle = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(
                // these are HALF extents!!!!!!!!!!
                (texture_size.x / 2.) * scale as f32, 
                (texture_size.y / 2.) * scale as f32
            ).mass(mass), 
            rigid_body_handle, 
            &mut space.sync_rigid_body_set
        );

        Self {
            collider_handle,
            body_handle: rigid_body_handle,
            sprite_path,
            scale,
            owner
        }
    }

    pub fn tick(&mut self, space: &Space, ctx: &mut TickContext) {
        if *ctx.uuid == self.owner {
            self.owner_tick(ctx);
        }

        self.all_tick(space, ctx);

    }
    fn owner_tick(&mut self, ctx: &mut TickContext) {
        ctx.owned_colliders.push(self.collider_handle);
        ctx.owned_rigid_bodies.push(self.body_handle);
    }

    fn all_tick(&mut self, space: &Space, ctx: &mut TickContext) {
        let rigid_body = space.sync_rigid_body_set.get_sync(self.body_handle).unwrap();

        //println!("body part angle: {}", rigid_body.rotation().angle());
    }
    pub async fn draw(&self, textures: &mut TextureLoader, space: &Space, flip_x: bool) {

        draw_texture_onto_physics_body(
            self.body_handle, 
            self.collider_handle, 
            space, 
            &self.sprite_path, 
            textures, 
            flip_x, 
            false, 
            0.
        ).await
        
    }
}