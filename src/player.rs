use diff::Diff;
use gamelibrary::macroquad_to_rapier;
use macroquad::{color::WHITE, input::{is_key_down, KeyCode}, math::{vec2, Vec2}, texture::DrawTextureParams};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::TickContext;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Player {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
    pub sprite_path: String,
}

impl Player {

    pub fn spawn(ctx: &mut TickContext, position: &Vec2) -> Self {

        let rigid_body = RigidBodyBuilder::dynamic()
            .position(vector![position.x, position.y].into())
            .build();

        let collider = ColliderBuilder::cuboid(18., 15.).build();

        let rigid_body_handle = ctx.game_state.level.space.rigid_body_set.insert(rigid_body);
        let collider_handle = ctx.game_state.level.space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut ctx.game_state.level.space.rigid_body_set);

        Player {
            rigid_body: rigid_body_handle,
            collider: collider_handle,
            sprite_path: "player/idle.png".to_string(),
        }
    }

    pub fn tick(&mut self, ctx: &mut TickContext) {
        self.control(ctx);
    }

    pub fn control(&mut self, ctx: &mut TickContext) {

        let rigid_body = ctx.game_state.level.space.rigid_body_set.get_mut(self.rigid_body).unwrap();

        if is_key_down(KeyCode::W) {

            // stop any y axis movement if going down
            if rigid_body.linvel().y.is_sign_negative() {
                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x, 0.],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x, rigid_body.linvel().y + 4.],
                true
            )
        }

        if is_key_down(KeyCode::S) {

            if rigid_body.linvel().y.is_sign_positive() {
                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x, 0.],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x, rigid_body.linvel().y - 4.],
                true
            )
        }
        
        if is_key_down(KeyCode::A) {

            if rigid_body.linvel().x.is_sign_positive() {
                rigid_body.set_linvel(
                    vector![0., rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x - 4., rigid_body.linvel().y],
                true
            )
        }

        if is_key_down(KeyCode::D) {

            if rigid_body.linvel().x.is_sign_negative() {
                rigid_body.set_linvel(
                    vector![0., rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x + 4., rigid_body.linvel().y],
                true
            )
        }
    }

    pub async fn draw(&self, ctx: &mut TickContext<'_>) {

        let texture = ctx.textures.get(&self.sprite_path).await;
        let rigid_body = ctx.game_state.level.space.rigid_body_set.get(self.rigid_body).unwrap();
        let body_position = rigid_body.position().translation;
        let rotation = rigid_body.rotation().angle();

        let draw_pos = macroquad_to_rapier(&vec2(body_position.x, body_position.y));

        macroquad::texture::draw_texture_ex(
            texture, 
            draw_pos.x, 
            draw_pos.y, 
            WHITE,
            DrawTextureParams {
                dest_size: None,
                source: None,
                rotation,
                flip_x: false,
                flip_y: false,
                pivot: None,
            }
        );
    }
}