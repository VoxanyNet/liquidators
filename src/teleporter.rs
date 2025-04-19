use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::Space, traits::{draw_hitbox, HasPhysics}, uuid};
use macroquad::{color::{BLUE, RED, WHITE}, math::Vec2, shapes::draw_circle};
use nalgebra::{vector, Vector2};
use parry2d::math::{Isometry, Vector};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, InteractionGroups, QueryFilter, RigidBodyBuilder, RigidBodyHandle};
use serde::{de, Deserialize, Serialize};
use parry2d::math::Real;
use web_sys::console::time_stamp;

use crate::{collider_contains_point, player::{self, player::Player}, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Teleporter {
    body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    // seperate collider for checking for valid teleport candidates
    teleport_candidate_collider: ColliderHandle,
    owner: String,
    destination: Option<RigidBodyHandle>
}

impl Teleporter {

    pub fn get_rigid_body_handle(&self) -> RigidBodyHandle {
        self.body_handle
    }
    pub fn set_destination(&mut self, destination: Option<RigidBodyHandle>) {
        self.destination = destination;
    } 

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
                .position(vector![0., 50.].into())
                .collision_groups(InteractionGroups::none()), 
            teleporter_body_handle, 
            &mut space.rigid_body_set
        );

        Self {
            body_handle: teleporter_body_handle,
            collider_handle: teleporter_collider_handle,
            teleport_candidate_collider: teleport_candidate_collider_handle,
            owner: owner.clone(),
            destination: None
        }
        
    }
    pub fn tick(&mut self, ctx: &mut TickContext, space: &mut Space, players: &mut Vec<Player>) {
        

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

    pub fn get_teleport_candidate_bodies(&mut self, ctx: &mut TickContext, space: &mut Space, players: &mut Vec<Player>) -> Vec<RigidBodyHandle> {
        let candidate_collider = space.collider_set.get(self.teleport_candidate_collider).unwrap();

        // all rigid bodies that are in the teleporter candidate field
        let mut candidate_bodies: Vec<RigidBodyHandle> = Vec::new();

        space.query_pipeline.intersections_with_shape(
            &space.rigid_body_set, 
            &space.collider_set, 
            candidate_collider.position(), 
            candidate_collider.shape(), 
            QueryFilter::default(),
            |handle| {

                let candidate_body_handle = space.collider_set.get(handle).unwrap().parent().unwrap();
                
                // dont teleport the teleporter
                if candidate_body_handle == self.get_rigid_body_handle() {
                    return true;
                }
                
                println!("{}", players.len());
                for player in &*players {

                    // we only want to teleport player bodies, not their heads
                    if player.body.collider_handle != handle {
                        continue;
                    }

                }

                candidate_bodies.push(candidate_body_handle);

                return true
            }
        );

        candidate_bodies

        
    }
    pub fn owner_tick(&mut self, ctx: &mut TickContext, space: &mut Space, players: &mut Vec<Player>) {

        
        self.add_owned_physics_objects(ctx);

        let candidates = self.get_teleport_candidate_bodies(ctx, space, players);

        self.teleport_candidates(candidates, space);
        
    }

    pub fn teleport_candidates(&mut self, candidates: Vec<RigidBodyHandle>, space: &mut Space) {

        let destination_handle = match self.destination {
            Some(destination) => destination,
            None => return,
        };

        let teleporter_collider = space.collider_set.get(self.collider_handle).unwrap();

        let teleporter_half_extents = teleporter_collider.shape().as_cuboid().unwrap().half_extents;

        let destination_body_position = {
            space.rigid_body_set.get(destination_handle).unwrap().position().clone()
        };

        let teleporter_position = space.rigid_body_set.get(self.body_handle).unwrap().position().clone();

        for candidate_body_handle in &candidates {

            let candidate_body = space.rigid_body_set.get_mut(*candidate_body_handle).unwrap();

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
        
        let pos = space.rigid_body_set.get(self.get_rigid_body_handle()).unwrap().position().translation;

        let pos = rapier_to_macroquad(&Vec2::new(pos.x, pos.y));
        
        draw_hitbox(space, self.body_handle, self.teleport_candidate_collider, RED);
        draw_hitbox(space, self.body_handle, self.collider_handle, WHITE);

        draw_circle(pos.x, pos.y, 5., RED);
        
    }
}