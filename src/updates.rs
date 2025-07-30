use gamelibrary::space::{SyncColliderHandle, SyncRigidBodyHandle};
use nalgebra::{Isometry2, Vector2};
use parry2d::shape::SharedShape;
use rapier2d::prelude::{InteractionGroups, RigidBodyType};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Update {
    RigidBodyPosition(RigidBodyPositionUpdate),
    RigidBodyVelocity(RigidBodyVelocityUpdate),
    RigidBodyAngularVelocity(RigidBodyAngularVelocityUpdate),
    RigidBodyNewCollider(RigidBodyNewColliderUpdate),
    RigidBodyRemoveCollider(RigidBodyRemoveColliderUpdate),
    RigidBodyBodyType(RigidBodyBodyTypeUpdate),
    RigidBodyMass(RigidBodyMassUpdate),
    ColliderShape(ColliderShapeUpdate),
    ColliderParent(ColliderParentUpdate),
    ColliderPosition(ColliderPositionUpdate),
    ColliderCollisionGroups(ColliderCollisionGroupsUpdate),
    ColliderMass(ColliderMassUpdate),
}
#[derive(Serialize, Deserialize)]
pub struct RigidBodyPositionUpdate {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub position: Isometry2<f32>
}

#[derive(Serialize, Deserialize)]
pub struct RigidBodyVelocityUpdate {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub velocity: Vector2<f32>
}

#[derive(Serialize, Deserialize)]
pub struct RigidBodyAngularVelocityUpdate {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub angular_velocity: f32
}

#[derive(Serialize, Deserialize)]
pub struct RigidBodyNewColliderUpdate {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub new_collider: SyncColliderHandle
}

#[derive(Serialize, Deserialize)]
pub struct RigidBodyRemoveColliderUpdate {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub removed_collider: SyncColliderHandle
}

#[derive(Serialize, Deserialize)]
pub struct RigidBodyBodyTypeUpdate {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub body_type: RigidBodyType
}

#[derive(Serialize, Deserialize)]
pub struct RigidBodyMassUpdate {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub mass: f32
}

#[derive(Serialize, Deserialize)]
pub struct ColliderShapeUpdate {
    pub collider_handle: SyncColliderHandle,
    pub shape: SharedShape
}

#[derive(Serialize, Deserialize)]
pub struct ColliderParentUpdate {
    pub collider_handle: SyncColliderHandle,
    pub parent: SyncRigidBodyHandle
}

#[derive(Serialize, Deserialize)]
pub struct ColliderPositionUpdate {
    pub collider_handle: SyncColliderHandle,
    pub position: Isometry2<f32>
}

#[derive(Serialize, Deserialize)]
pub struct ColliderCollisionGroupsUpdate {
    pub collider_handle: SyncColliderHandle,
    pub collision_groups: InteractionGroups
}

#[derive(Serialize, Deserialize)]
pub struct ColliderMassUpdate {
    pub collider_handle: SyncColliderHandle,
    pub mass: f32
}