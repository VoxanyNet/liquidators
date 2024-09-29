use diff::Diff;
use gamelibrary::{space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::math::Vec2;
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Shotgun {
    pub collider: ColliderHandle,
    pub rigid_body: RigidBodyHandle,
    pub sprite: String,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>
}

impl Shotgun {

    pub fn new(space: &mut Space, pos: Vec2) -> Self {

        let rigid_body = space.rigid_body_set.insert(
            RigidBodyBuilder::kinematic_velocity_based()
                .position(vector![pos.x, pos.y].into())
                .build()
        );

        let collider = space.collider_set.insert_with_parent(
            ColliderBuilder::cuboid(50., 11.).build(), 
            rigid_body, 
            &mut space.rigid_body_set
        );

        Self {
            collider,
            rigid_body,
            sprite: "assets/shotgun.png".to_string(),
            selected: false,
            dragging: false,
            drag_offset: None,
        }
    }
    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {

        self.draw_texture(space, &self.sprite, textures).await;

    }
}

impl HasPhysics for Shotgun {
    fn collider_handle(&self) -> &ColliderHandle {
        &self.collider
    }

    fn rigid_body_handle(&self) -> &RigidBodyHandle {
        &self.rigid_body
    }

    fn selected(&self) -> &bool {
        &self.selected
    }

    fn selected_mut(&mut self) -> &mut bool {
        &mut self.selected
    }

    fn dragging(&mut self) -> &mut bool {
        &mut self.dragging
    }

    fn drag_offset(&mut self) -> &mut Option<macroquad::prelude::Vec2> {
        &mut self.drag_offset
    }
}