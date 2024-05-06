use std::collections::HashMap;



use crate::{collider::Collider, rigid_body::{self, RigidBody}};

pub type RigidBodyHandle = String;
pub type ColliderHandle = String;

struct Space {
    rigid_bodies: HashMap<RigidBodyHandle, RigidBody>,
    colliders: HashMap<ColliderHandle, Collider>
}

impl Space {
    pub fn insert_rigid_body(&mut self, rigid_body: RigidBody) -> RigidBodyHandle {
        let handle: RigidBodyHandle = uuid::Uuid::new_v4().to_string();

        self.rigid_bodies.insert(handle.clone(), rigid_body);

        handle

    }

    pub fn get_rigid_body(&mut self, rigid_body_handle: RigidBodyHandle) -> Option<&mut RigidBody> {

        let rigid_body = self.rigid_bodies.get_mut(&rigid_body_handle);

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