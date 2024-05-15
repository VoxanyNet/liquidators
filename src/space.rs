use std::collections::HashMap;



use diff::Diff;
use nalgebra::vector;
use rapier2d::{dynamics::{CCDSolver, ImpulseJointSet, IntegrationParameters, IslandManager, MultibodyJointSet, RigidBodySet}, geometry::{BroadPhase, ColliderSet, NarrowPhase}, pipeline::{PhysicsPipeline, QueryPipeline}};
use serde::{Deserialize, Serialize};

use crate::{collider::Collider, rigid_body::{self, RigidBody}};

pub type RigidBodyHandle = String;
pub type ColliderHandle = String;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Space {
    rigid_bodies: HashMap<RigidBodyHandle, RigidBody>,
    colliders: HashMap<ColliderHandle, Collider>
}

impl Space {

    pub fn new() -> Self {
        Self {
            rigid_bodies: HashMap::new(),
            colliders: HashMap::new()
        }
    }

    pub fn step(&mut self, owner: &String) {

        // convert all of the rigid bodies proxies to the actual rapier rigid body, step them all, then convert them back into the proxy type

        // this maps all of the real rigid bodies to their proxy types, so the proxy types can be updated
        let mut rigid_body_map: HashMap<rapier2d::dynamics::RigidBodyHandle, RigidBodyHandle> = HashMap::new();
        // this maps all of the real colliders to their proxy types, so that the proxy types can be updated
        let mut collider_map: HashMap<rapier2d::geometry::ColliderHandle, ColliderHandle> = HashMap::new();

        // create all of the temporary structs needed to step the rigid bodies
        let gravity = vector![0., 0.];
        let integration_parameters = IntegrationParameters::default();
        let mut island_manager = IslandManager::default();
        let mut broad_phase = BroadPhase::new();
        let mut narrow_phase = NarrowPhase::new();
        let mut rigid_body_set = RigidBodySet::new();
        let mut collider_set = ColliderSet::new();
        let mut impulse_joint_set = ImpulseJointSet::new();
        let mut multibody_joint_set = MultibodyJointSet::new();
        let mut ccd_solver = CCDSolver::new();
        let mut query_pipeline = QueryPipeline::new();
        let physics_hooks = ();
        let event_handler = ();

        let mut physics_pipeline = PhysicsPipeline::new();
        
        
        for (rigid_body_proxy_handle, rigid_body_proxy) in self.rigid_bodies.iter_mut() {
            let mut rigid_body: rapier2d::dynamics::RigidBody = rigid_body_proxy.into();

            let rigid_body_handle = rigid_body_set.insert(rigid_body);

            rigid_body_map.insert(rigid_body_handle, rigid_body_proxy_handle.clone());
        }
        
        for (collider_proxy_handle, collider_proxy) in self.colliders.iter_mut() {
            let mut collider: rapier2d::geometry::Collider = collider_proxy.into();

            let collider_handle = collider_set.insert(collider);

            collider_map.insert(collider_handle, collider_proxy_handle.clone());
        }

        physics_pipeline.step(
            &gravity,
            &integration_parameters,
            &mut island_manager,
            &mut broad_phase,
            &mut narrow_phase,
            &mut rigid_body_set,
            &mut collider_set,
            &mut impulse_joint_set,
            &mut multibody_joint_set,
            &mut ccd_solver,
            Some(&mut query_pipeline),
            &physics_hooks,
            &event_handler
        );

        // update the proxies
        for (rigid_body_handle, rigid_body_proxy_handle ) in rigid_body_map {
            
            let rigid_body_proxy = self.rigid_bodies.get_mut(&rigid_body_proxy_handle)
                .expect("Invalid rigid body proxy handle");

            // we only update the proxy rigid type if we own it
            if rigid_body_proxy.owner != *owner {
                continue;
            }

            // fetch the corresponding rigid body
            let rigid_body = rigid_body_set.get(rigid_body_handle)
                .expect("Invalid rigid body handle");

            // update the rigid body proxy with the actual rigid body
            rigid_body_proxy.update_from_rigid_body(rigid_body);
        }

        for (collider_handle, collider_proxy_handle) in collider_map {
            let collider_proxy = self.colliders.get_mut(&collider_proxy_handle)
                .expect("Invalid collider proxy handle");
            
            
        }



    }
    pub fn insert_rigid_body(&mut self, rigid_body: RigidBody) -> RigidBodyHandle {
        let handle: RigidBodyHandle = uuid::Uuid::new_v4().to_string();

        self.rigid_bodies.insert(handle.clone(), rigid_body);

        handle

    }

    pub fn get_rigid_body(&mut self, rigid_body_handle: &RigidBodyHandle) -> Option<&mut RigidBody> {

        let rigid_body = self.rigid_bodies.get_mut(rigid_body_handle);

        rigid_body
    }

    pub fn insert_collider_body(&mut self, collider: Collider) -> ColliderHandle {
        let handle: ColliderHandle = uuid::Uuid::new_v4().to_string();

        self.colliders.insert(handle.clone(), collider);

        handle
    }

    pub fn get_collider_body(&mut self, collider_handle: ColliderHandle) -> Option<&mut Collider> {
        
        let collider = self.colliders.get_mut(&collider_handle);
        
        collider

    }
    
}