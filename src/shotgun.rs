use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

pub struct Shotgun {
    pub collider: ColliderHandle,
    pub rigid_body: RigidBodyHandle
}