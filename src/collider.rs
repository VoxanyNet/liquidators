use crate::proxies::macroquad::math::rect::Rect;

pub type Collider = Rect;

impl From<rapier2d::dynamics::RigidBody> for RigidBody {
    fn from(value: rapier2d::dynamics::RigidBody) -> Self {
        Self {
            position: Vec2::new(value.position().translation.x, value.position().translation.y),
            velocity: Vec2::new(value.linvel().x, value.linvel().y),
            body_type: value.body_type().into()
        }
    }
}

impl From<&mut rapier2d::dynamics::RigidBody> for RigidBody {
    fn from(value: &mut rapier2d::dynamics::RigidBody) -> Self {
        Self {
            position: Vec2::new(value.position().translation.x, value.position().translation.y),
            velocity: Vec2::new(value.linvel().x, value.linvel().y),
            body_type: value.body_type().into()
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