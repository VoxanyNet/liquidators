use diff::Diff;
use rapier2d::{dynamics::RigidBodyBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]

pub struct Collider {
    pub hx: f32,
    pub hy: f32,
    pub restitution: f32,
    pub mass: f32,
    pub owner: String
}

impl Collider {
    fn update(&mut self, value: &rapier2d::geometry::Collider) {
        
        self.hx = value.shape().as_cuboid().unwrap().half_extents.x;
        self.hy = value.shape().as_cuboid().unwrap().half_extents.y;
        self.restitution = value.restitution();
        self.mass = value.mass();
    }

    fn update_mut(&mut self, value: &mut rapier2d::geometry::Collider) {
        
        self.hx = value.shape().as_cuboid().unwrap().half_extents.x;
        self.hy = value.shape().as_cuboid().unwrap().half_extents.y;
        self.restitution = value.restitution();
        self.mass = value.mass();
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