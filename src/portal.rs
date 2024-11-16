use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::Space};
use macroquad::math::Vec2;
use rapier2d::prelude::ColliderHandle;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Portal {
    pub attached_collider: ColliderHandle
}

impl Portal {
    pub async fn draw(&self, space: &Space) {

        let collider = space.collider_set.get(self.attached_collider).unwrap();

        let collider_pos = Vec2::new(
            collider.position().translation.x, 
            collider.position().translation.y
        );

        let _collider_pos_macroquad = rapier_to_macroquad(&collider_pos);

        // ROTATE LINE WITH ANGLE OF ROTATION FOR COLLIDER
        // or just draw a square jutting out underneath dummy

    

    }
}