use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, sync_arena::SyncArena, traits::draw_hitbox};
use macroquad::{color::{RED, WHITE}, math::Vec2, shapes::draw_circle};
use nalgebra::{vector, Vector2};
use parry2d::math::Isometry;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, InteractionGroups, QueryFilter, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};
use parry2d::math::Real;
use crate::{player::player::Player, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Teleporter {
    body_handle: SyncRigidBodyHandle,
    collider_handle: SyncColliderHandle,
    // seperate collider for checking for valid teleport candidates
    teleport_candidate_collider: SyncColliderHandle,
    owner: String,
    destination: Option<SyncRigidBodyHandle>
}

impl Teleporter {

    pub fn get_rigid_body_handle(&self) -> SyncRigidBodyHandle {
        self.body_handle
    }
    pub fn set_destination(&mut self, destination: Option<SyncRigidBodyHandle>) {
        self.destination = destination;
    } 

    pub fn new(pos: Vec2, space: &mut Space, owner: &String) -> Self {
        let teleporter_body_handle = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .position(vector![pos.x, pos.y].into())
                .build()
        );

        // the collider for the actual teleporter, not the candidate field
        let teleporter_collider_handle = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(20., 5.)
                .position(vector![0., 0.].into()), 
            teleporter_body_handle, 
            &mut space.sync_rigid_body_set
        );

        // the collider for the candidate field
        let teleport_candidate_collider_handle = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(20., 20.)
                .position(vector![0., 50.].into())
                .collision_groups(InteractionGroups::none()), 
            teleporter_body_handle, 
            &mut space.sync_rigid_body_set
        );

        Self {
            body_handle: teleporter_body_handle,
            collider_handle: teleporter_collider_handle,
            teleport_candidate_collider: teleport_candidate_collider_handle,
            owner: owner.clone(),
            destination: None
        }
        
    }
    pub fn tick(&mut self, ctx: &mut TickContext, space: &mut Space, players: &mut SyncArena<Player>) {
        

        self.all_tick(ctx);

        if *ctx.uuid == self.owner {
            self.owner_tick(ctx, space, players);
        }
        
    }

    pub fn all_tick(&mut self, ctx: &mut TickContext) {
    }

    pub fn add_owned_physics_objects(&self, ctx: &mut TickContext) {
        ctx.owned_colliders.push(self.collider_handle);
        ctx.owned_colliders.push(self.teleport_candidate_collider);

        ctx.owned_rigid_bodies.push(self.body_handle);
    }

    pub fn get_teleport_candidate_bodies(&mut self, ctx: &mut TickContext, space: &mut Space, players: &mut SyncArena<Player>) -> Vec<SyncRigidBodyHandle> {
        let candidate_collider = space.sync_collider_set.get_sync(self.teleport_candidate_collider).unwrap();

        // all rigid bodies that are in the teleporter candidate field
        let mut candidate_bodies: Vec<SyncRigidBodyHandle> = Vec::new();

        space.query_pipeline.intersections_with_shape(
            &space.sync_rigid_body_set.rigid_body_set, 
            &space.sync_collider_set.collider_set, 
            candidate_collider.position(), 
            candidate_collider.shape(), 
            QueryFilter::default(),
            |handle| {

                let local_candidate_body_handle = space.sync_collider_set.get_local(handle).unwrap().parent().unwrap();

                let sync_candidate_body_handle = space.sync_rigid_body_set.get_sync_handle(local_candidate_body_handle);
                
                // dont teleport the teleporter
                if sync_candidate_body_handle == self.get_rigid_body_handle() {
                    return true;
                }
                
                println!("{}", players.len());
                for (_, player) in &*players {

                    let local_player_body_collider_handle = space.sync_collider_set.get_local_handle(player.body.collider_handle);
                    

                    // we only want to teleport player bodies, not their heads
                    if local_player_body_collider_handle != handle {
                        continue;
                    }

                }

                candidate_bodies.push(sync_candidate_body_handle);

                return true
            }
        );

        candidate_bodies

        
    }
    pub fn owner_tick(&mut self, ctx: &mut TickContext, space: &mut Space, players: &mut SyncArena<Player>) {

        
        self.add_owned_physics_objects(ctx);

        let candidates = self.get_teleport_candidate_bodies(ctx, space, players);

        self.teleport_candidates(candidates, space);
        
    }

    pub fn teleport_candidates(&mut self, candidates: Vec<SyncRigidBodyHandle>, space: &mut Space) {

        let destination_handle = match self.destination {
            Some(destination) => destination,
            None => return,
        };

        let teleporter_collider = space.sync_collider_set.get_sync(self.collider_handle).unwrap();

        let teleporter_half_extents = teleporter_collider.shape().as_cuboid().unwrap().half_extents;

        let destination_body_position = {
            space.sync_rigid_body_set.get_sync(destination_handle).unwrap().position().clone()
        };

        let teleporter_position = space.sync_rigid_body_set.get_sync(self.body_handle).unwrap().position().clone();

        for candidate_body_handle in &candidates {

            let candidate_body = space.sync_rigid_body_set.get_sync_mut(*candidate_body_handle).unwrap();

            let candidate_position = candidate_body.position().translation;

            // candidate_body.set_linvel(
            //     Vector::<Real>::new(0., 0.),
            //     true
            // );

            let position_relative_to_teleporter = Vec2 {
                x: candidate_position.x - teleporter_position.translation.x,
                y: candidate_position.y - teleporter_position.translation.y
            };

            // the position we are going to actually teleport the candidate
            let teleport_position = {

                // Isometry::<Real>::new(
                //     Vector2::<Real>::new(
                //         destination_body_position.translation.x + position_relative_to_teleporter.x, 
                //         destination_body_position.translation.y + position_relative_to_teleporter.y
                //     ), 
                //     destination_body_position.rotation.angle()
                // )

                Isometry::<Real>::new(
                    Vector2::<Real>::new(
                        destination_body_position.translation.x, 
                        destination_body_position.translation.y + 50.
                    ), 
                    0.
                )
            };

            candidate_body.set_position(teleport_position, true);
        }


    }
    pub fn draw(&self, space: &Space) {
        
        let pos = space.sync_rigid_body_set.get_sync(self.get_rigid_body_handle()).unwrap().position().translation;

        let pos = rapier_to_macroquad(&Vec2::new(pos.x, pos.y));
        
        draw_hitbox(space, self.body_handle, self.teleport_candidate_collider, RED);
        draw_hitbox(space, self.body_handle, self.collider_handle, WHITE);

        draw_circle(pos.x, pos.y, 5., RED);
        
    }
}