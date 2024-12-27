use diff::Diff;
use gamelibrary::{rapier_to_macroquad, space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{color::WHITE, math::Vec2, text, texture::{draw_texture_ex, DrawTextureParams}};
use nalgebra::vector;
use rapier2d::prelude::{Collider, ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Boat {
    pub top_sprite_path: String,
    pub bottom_sprite_path: String,
    pub owner: String,
    pub collider_handle: ColliderHandle,
    pub rigid_body_handle: RigidBodyHandle,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub scale: f32
}

impl Boat {
    pub fn spawn(pos: Vec2, owner: String, space: &mut Space, boats: &mut Vec<Boat>) {

        let scale = 3.;

        let rigid_body = RigidBodyBuilder::dynamic()
            .position(vector![pos.x, pos.y].into())
            .ccd_enabled(true)
            .build();

        let collider = ColliderBuilder::cuboid((75. / 2.) * scale, (12. / 2.) * scale)
            .mass(1000.)
            .build();

        let rigid_body_handle = space.rigid_body_set.insert(rigid_body);

        let collider_handle = space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut space.rigid_body_set);

        let boat = Self {
            top_sprite_path: "assets/boat/top.png".to_string(),
            bottom_sprite_path: "assets/boat/bottom.png".to_string(),
            owner,
            collider_handle,
            rigid_body_handle,
            selected: false,
            dragging: false,
            drag_offset: None,
            scale
        };

        boats.push(boat);

    }

    pub async fn draw_top(&self, space: &Space, textures: &mut TextureLoader) {

        // THIS IS STUPID

        let texture = textures.get(&self.top_sprite_path).await;
        let rigid_body = space.rigid_body_set.get(self.rigid_body_handle).unwrap();
        let position = rigid_body.translation();
        let rotation = rigid_body.rotation().angle();
        let collider = space.collider_set.get(self.collider_handle).unwrap();
        let shape = collider.shape().as_cuboid().unwrap();
        let mut params = DrawTextureParams::default();

        let draw_pos = rapier_to_macroquad(
            &Vec2::new(
                position.x - shape.half_extents.x, 
                position.y + 170.
            )
        );

        params.rotation = rotation * -1.;

        params.dest_size = Some(Vec2 {
            x: 75. * self.scale,
            y: 63. * self.scale,
        });

        // we need to rotate the texture around the center of rigid body

        // params.pivot = Some(
        //     Vec2::new(
        //         (texture.height() / 2) + ,
        //         texture.width() / 2.
        //     )
        // )


        draw_texture_ex(texture, draw_pos.x, draw_pos.y, WHITE, params);

    }

    pub async fn draw_bottom(&self, space: &Space, textures: &mut TextureLoader) {

        let texture = textures.get(&self.bottom_sprite_path).await;
        let rigid_body = space.rigid_body_set.get(self.rigid_body_handle).unwrap();
        let position = rigid_body.translation();
        let rotation = rigid_body.rotation().angle();
        let collider = space.collider_set.get(self.collider_handle).unwrap();
        let shape = collider.shape().as_cuboid().unwrap();

        let mut params = DrawTextureParams::default();

        let draw_pos = rapier_to_macroquad(
            &Vec2::new(
                position.x - shape.half_extents.x,
                position.y + 169.,
            )
        );

        params.dest_size = Some(
            Vec2 {
                x: 75. * self.scale,
                y: 63. * self.scale,
            }
        );

        params.rotation = rotation * -1.;


        draw_texture_ex(texture, draw_pos.x, draw_pos.y, WHITE, params);

    }
}

impl HasPhysics for Boat {
    fn collider_handle(&self) -> &rapier2d::prelude::ColliderHandle {
        &self.collider_handle
    }

    fn rigid_body_handle(&self) -> &rapier2d::prelude::RigidBodyHandle {
        &self.rigid_body_handle
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