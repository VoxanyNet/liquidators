use diff::Diff;
use rapier2d::{dynamics::RigidBodyBuilder, na::vector};
use serde::{Deserialize, Serialize};

use crate::proxies::macroquad::math::vec2::Vec2;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum RigidBodyType {
    Dynamic,
    Fixed,
    KinematicPositionBased,
    KinematicVelocityBased
}

impl From<rapier2d::dynamics::RigidBodyType> for RigidBodyType {
    fn from(value: rapier2d::dynamics::RigidBodyType) -> Self {
        match value {
            rapier2d::dynamics::RigidBodyType::Dynamic => RigidBodyType::Dynamic,
            rapier2d::dynamics::RigidBodyType::Fixed => RigidBodyType::Fixed,
            rapier2d::dynamics::RigidBodyType::KinematicPositionBased => RigidBodyType::KinematicPositionBased,
            rapier2d::dynamics::RigidBodyType::KinematicVelocityBased => RigidBodyType::KinematicVelocityBased
        }
    }
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct RigidBody {
    pub position: Vec2,
    pub velocity: Vec2,
    pub body_type: RigidBodyType,
    pub owner: String
}

impl RigidBody {
    pub fn from_rigid_body(value: rapier2d::dynamics::RigidBody, owner: String) -> Self {
        Self {
            position: Vec2::new(value.position().translation.x, value.position().translation.y),
            velocity: Vec2::new(value.linvel().x, value.linvel().y),
            body_type: value.body_type().into(),
            owner
        }
    }


    pub fn from_rigid_body_mut(value: &mut rapier2d::dynamics::RigidBody, owner: String) -> Self {
        Self {
            position: Vec2::new(value.position().translation.x, value.position().translation.y),
            velocity: Vec2::new(value.linvel().x, value.linvel().y),
            body_type: value.body_type().into(),
            owner
        }
    }
}

impl Into<rapier2d::dynamics::RigidBody> for RigidBody {
    fn into(self) -> rapier2d::dynamics::RigidBody {

        match self.body_type {
            RigidBodyType::Dynamic => {
                RigidBodyBuilder::dynamic()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::Fixed => {
                RigidBodyBuilder::fixed()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::KinematicPositionBased => {
                RigidBodyBuilder::kinematic_position_based()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::KinematicVelocityBased => {
                RigidBodyBuilder::kinematic_velocity_based()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
        }
    }
}

impl Into<rapier2d::dynamics::RigidBody> for &RigidBody {
    fn into(self) -> rapier2d::dynamics::RigidBody {
        match self.body_type {
            RigidBodyType::Dynamic => {
                RigidBodyBuilder::dynamic()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::Fixed => {
                RigidBodyBuilder::fixed()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::KinematicPositionBased => {
                RigidBodyBuilder::kinematic_position_based()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::KinematicVelocityBased => {
                RigidBodyBuilder::kinematic_velocity_based()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
        }
    }
}

impl Into<rapier2d::dynamics::RigidBody> for &mut RigidBody {
    fn into(self) -> rapier2d::dynamics::RigidBody {
        match self.body_type {
            RigidBodyType::Dynamic => {
                RigidBodyBuilder::dynamic()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::Fixed => {
                RigidBodyBuilder::fixed()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::KinematicPositionBased => {
                RigidBodyBuilder::kinematic_position_based()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
            RigidBodyType::KinematicVelocityBased => {
                RigidBodyBuilder::kinematic_velocity_based()
                    .translation(vector![self.position.x, self.position.y])
                    .linvel(vector![self.velocity.x, self.velocity.y])
                    .build()
            },
        }
    }
}
