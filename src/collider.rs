use diff::Diff;
use rapier2d::{dynamics::RigidBodyBuilder, geometry::ColliderBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Collider {
    pub hx: f32,
    pub hy: f32,
    pub restitution: f32,
    pub mass: f32
}

impl From<rapier2d::geometry::Collider> for Collider {
    fn from(value: rapier2d::geometry::Collider) -> Self {
        Self {
            hx: value.shape().as_cuboid().unwrap().half_extents.x,
            hy: value.shape().as_cuboid().unwrap().half_extents.y,
            restitution: value.restitution(),
            mass: value.mass(),
        }
    }
}

impl From<&mut rapier2d::geometry::Collider> for Collider {
    fn from(value: &mut rapier2d::geometry::Collider) -> Self {
        Self {
            hx: value.shape().as_cuboid().unwrap().half_extents.x,
            hy: value.shape().as_cuboid().unwrap().half_extents.y,
            restitution: value.restitution(),
            mass: value.mass(),
        }
    }
}


impl Into<rapier2d::geometry::Collider> for Collider {
    fn into(self) -> rapier2d::geometry::Collider {

        rapier2d::geometry::ColliderBuilder::cuboid(self.hx, self.hy)
            .restitution(self.restitution)
            .mass(self.mass)
            .build()

    }
}

impl Into<rapier2d::geometry::Collider> for &Collider {
    fn into(self) -> rapier2d::geometry::Collider {
        
        rapier2d::geometry::ColliderBuilder::cuboid(self.hx, self.hy)
            .restitution(self.restitution)
            .mass(self.mass)
            .build()

    }
}

impl Into<rapier2d::geometry::Collider> for &mut Collider {
    fn into(self) -> rapier2d::geometry::Collider {
        rapier2d::geometry::ColliderBuilder::cuboid(self.hx, self.hy)
            .restitution(self.restitution)
            .mass(self.mass)
            .build()
    }
}