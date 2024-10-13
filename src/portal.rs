use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::Space};
use macroquad::{color::BLUE, math::{Rect, Vec2}, shapes::draw_line};
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};
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

        let collider_pos_macroquad = rapier_to_macroquad(&collider_pos);

        // ROTATE LINE WITH ANGLE OF ROTATION FOR COLLIDER

    

    }
}