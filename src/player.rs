use diff::Diff;
use ears::Music;
use gamelibrary::{macroquad_to_rapier, space::Space, traits::HasPhysics};
use macroquad::{color::WHITE, input::{is_key_down, KeyCode}, math::{vec2, Vec2}, texture::DrawTextureParams};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{game_state::GameState, level::Level, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Player {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
    pub sprite_path: String,
    pub owner: String,
    pub sounds: Vec<Music>,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>
}

impl Player {

    pub fn spawn(players: &mut Vec<Player>, space: &mut Space, owner: String, position: &Vec2) {

        let rigid_body = RigidBodyBuilder::dynamic()
            .position(vector![position.x, position.y].into())
            .lock_rotations()
            .build();

        let collider = ColliderBuilder::cuboid(18., 15.).build();

        let rigid_body_handle = space.rigid_body_set.insert(rigid_body);
        let collider_handle = space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut space.rigid_body_set);

        players.push(
            Player {
                rigid_body: rigid_body_handle,
                collider: collider_handle,
                sprite_path: "assets/player/idle.png".to_string(),
                owner,
                sounds: vec![],
                selected: false,
                dragging: false,
                drag_offset: None
            }
        )
    }

    pub fn tick(&mut self, level: &mut Level, ctx: &mut TickContext) {
        self.control(level, ctx);
    }

    pub fn control(&mut self, level: &mut Level, ctx: &mut TickContext) {

        let rigid_body = level.space.rigid_body_set.get_mut(self.rigid_body).unwrap();

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

    pub async fn draw(&self, game_state: &GameState, ctx: &mut TickContext<'_>) {

        self.draw_texture(&game_state.level.space, &self.sprite_path, &mut ctx.textures).await;
    }
}

impl HasPhysics for Player {
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

    fn drag_offset(&mut self) -> &mut Option<Vec2> {
        &mut self.drag_offset
    }
}