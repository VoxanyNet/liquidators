use std::{hash::Hash, time::Duration};

use diff::Diff;
use gamelibrary::{font_loader::FontLoader, rapier_to_macroquad, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, traits::draw_hitbox};
use macroquad::{color::{Color, GREEN, RED}, math::{vec2, Vec2}, text::{draw_text, draw_text_ex, Font, TextParams}};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, MassProperties, RigidBodyBuilder};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct DamageNumber {
    number: i32,
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub collider_handle: SyncColliderHandle,
    font_size: u16,
    font_color: Color,
}

impl Hash for DamageNumber {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.number.hash(state);
        self.rigid_body_handle.hash(state);
        self.collider_handle.hash(state);
        self.font_size.hash(state);
    }
}

impl Eq for DamageNumber {}

impl DamageNumber {
    pub fn new(
        space: &mut Space, 
        number: i32,
        pos: Vec2,
        font_size: Option<u16>, 
        font_color: Option<Color>
    ) -> Self {
        let rigid_body_handle = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .position(vector![pos.x, pos.y].into())
        );


        let collider_handle = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(5., 10.)
                .mass(0.1),
            rigid_body_handle, 
            &mut space.sync_rigid_body_set
        );

        let font_size = match font_size {
            Some(font_size) => font_size,
            None => 20,
        };

        let font_color = match font_color {
            Some(font_color) => font_color,
            None => RED,
        };

        Self {
            number,
            rigid_body_handle,
            collider_handle,
            font_size,
            font_color
        }
        
    }

    pub async fn draw(&self, space: &Space, fonts: &mut FontLoader) {

    
        let body = space.sync_rigid_body_set.get_sync(self.rigid_body_handle).unwrap();

        let pos = body.translation();

        let draw_pos = rapier_to_macroquad(&vec2(pos.x, pos.y));

        //draw_hitbox(space, self.rigid_body_handle, self.collider_handle, GREEN);

        draw_text_ex(
            self.number.to_string().as_str(), 
            draw_pos.x - 5., // we need to draw the text with an offset to line up with the rigid body
            draw_pos.y + 5., 
            TextParams {
                font: Some(fonts.get(&"assets/fonts/CutePixel.ttf".to_string()).await),
                font_size: 24,
                font_scale: 1.,
                font_scale_aspect: 1.,
                rotation: -body.rotation().angle(),
                color: self.font_color,
            }
        );

    }

}