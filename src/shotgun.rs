use diff::Diff;
use gamelibrary::{space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::math::Vec2;
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{player::player::Player, Grabbable, TickContext};


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
    pub drag_offset: Option<Vec2>,
    pub grabbing: bool,
    pub owner: String,
    pub picked_up: bool
}

impl Grabbable for Shotgun {
    fn grabbing(&mut self) -> &mut bool {
        &mut self.grabbing
    }
}

impl Shotgun {

    pub fn new(space: &mut Space, pos: Vec2, owner: String) -> Self {
        let rigid_body = space.rigid_body_set.insert(
            RigidBodyBuilder::dynamic()
                .ccd_enabled(true)
                .position(vector![pos.x, pos.y].into())
                .build()
        );

        let collider = space.collider_set.insert_with_parent(
            ColliderBuilder::cuboid(37.5, 8.25)
                .mass(10.)
                .build(), 
            rigid_body, 
            &mut space.rigid_body_set
        );

        Self {
            picked_up: false,
            collider,
            rigid_body,
            sprite: "assets/shotgun.png".to_string(),
            selected: false,
            dragging: false,
            drag_offset: None,
            grabbing: false,
            owner
        }
    }

    pub fn spawn(space: &mut Space, pos: Vec2, shotguns: &mut Vec<Shotgun>, owner: String) {

        shotguns.push(
            Self::new(space, pos, owner)
        );
    }

    pub fn tick(&mut self, players: &mut Vec<Player>, space: &mut Space, ctx: &mut TickContext) {

        self.grab(players, space, ctx);

        ctx.owned_rigid_bodies.push(self.rigid_body);
        ctx.owned_colliders.push(self.collider);
    }   

    pub fn fire(&mut self) {
        
    }

    pub fn grab(&mut self, players: &mut Vec<Player>, space: &mut Space, ctx: &mut TickContext) {

        if self.picked_up {
            return;
        }

        // we need to have a more efficient way of finding the currently controlled player
        for player in players {
            if player.owner == *ctx.uuid {

                let reference_body = player.body.body_handle;

                self.update_grabbing(space, ctx.camera_rect, Vec2::new(250., 250.), reference_body);

                break;

            }
        }

        self.update_grab_velocity(space);

    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, flip_x: bool) {

        self.draw_texture(space, &self.sprite, textures, flip_x, false, 0.).await;

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