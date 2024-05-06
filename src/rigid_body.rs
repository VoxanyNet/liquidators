use diff::Diff;
use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::proxies::rapier2d::dynamics::RigidBody::RigidBody;


#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum RigidBodyType {
    Dynamic,
    Static,
    Kinematic
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct RigidBody {
    position: Vec2,
    velocity: Vec2,
    body_type: RigidBodyType
}

impl From<rapier2d::dynamics::RigidBody> for RigidBody {
    fn from(value: rapier2d::dynamics::RigidBody) -> Self {
        Self {
            position: Vec2::new(value.position().translation.x, value.position().translation.y)
            velocity: value.linvel()[0]
        }
    }
}

impl From<&mut rapier2d::dynamics::RigidBody> for RigidBody {
    fn from(value: &mut rapier2d::dynamics::RigidBody) -> Self {
        // Self {
        //     x: value.x,
        //     y: value.y
        // }
    }
}


impl Into<rapier2d::dynamics::RigidBody> for RigidBody {
    fn into(self) -> rapier2d::dynamics::RigidBody {
        // rapier2d::dynamics::RigidBody {
        //     x: self.x,
        //     y: self.y,
        // }
    }
}

impl Into<rapier2d::dynamics::RigidBody> for &RigidBody {
    fn into(self) -> rapier2d::dynamics::RigidBody {
        // rapier2d::dynamics::RigidBody {
        //     x: self.x,
        //     y: self.y,
        // }
    }
}

impl Into<rapier2d::dynamics::RigidBody> for &mut RigidBody {
    fn into(self) -> rapier2d::dynamics::RigidBody {
        // rapier2d::dynamics::RigidBody {
        //     x: self.x,
        //     y: self.y,
        // }
    }
}
