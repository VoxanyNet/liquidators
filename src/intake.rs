use gamelibrary::space::{SyncColliderHandle, SyncRigidBodyHandle};

pub struct Intake {
    rigid_body_handle: SyncRigidBodyHandle,
    collider_handle: SyncColliderHandle   
}