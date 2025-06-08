use diff::Diff;
use gamelibrary::{space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, texture_loader::TextureLoader, time::Time, traits::draw_texture_onto_physics_body};
use macroquad::math::Vec2;
use nalgebra::vector;
use parry2d::shape::Cuboid;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, QueryFilter, RigidBodyBuilder};
use serde::{Deserialize, Serialize};

use crate::TickContext;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Grenade {
    body_handle: SyncRigidBodyHandle,
    collider_handle: SyncColliderHandle,
    spawned: Time,
    exploded: bool
}

impl Grenade {
    pub fn new(position: Vec2, space: &mut Space) -> Self {
        
        let body_handle = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .position(vector![position.x, position.y].into())

        );

        let collider_handle = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(10., 10.), 
            body_handle, 
            &mut space.sync_rigid_body_set
        );

        Self {
            body_handle,
            collider_handle,
            spawned: Time::now(),
            exploded: false
        }
    }

    pub fn tick(&mut self, space: &mut Space, ctx: &mut TickContext) {
        ctx.owned_colliders.push(self.collider_handle);
        ctx.owned_rigid_bodies.push(self.body_handle);

        if self.spawned.elapsed().num_seconds() > 2 && self.exploded == false {

            let position = space.sync_collider_set.get_sync(self.collider_handle).unwrap().position();

            let mut collisions: Vec<ColliderHandle> = vec![];

            space.query_pipeline.intersections_with_shape(
                &space.sync_rigid_body_set.rigid_body_set, 
                &space.sync_collider_set.collider_set, 
                position, 
                &Cuboid::new(vector![100., 100.].into()), 
                QueryFilter::exclude_fixed(),
                |collider| {
                    collisions.push(collider);

                    true
                }
            );

            for collider_handle in collisions {
                let parent_handle = space.sync_collider_set.get_local(collider_handle).unwrap().parent().unwrap();

                let parent = space.sync_rigid_body_set.get_local_mut(parent_handle).unwrap();

                let epic = Vec2::new(parent.position().translation.x - position.translation.x, parent.position().translation.y - position.translation.y);
                
                parent.set_linvel(vector![epic.x * 50., epic.y * 50.].into(), true);
            }

            self.exploded = true;
        }

    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {
        draw_texture_onto_physics_body(
            self.body_handle, 
            self.collider_handle, 
            space, 
            &"assets/grenade/grenade.png".to_string(), 
            textures, 
            false, 
            false, 
            0.
        ).await;
    }
}