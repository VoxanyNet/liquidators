use diff::Diff;
use futures::executor::block_on;
use gamelibrary::{space::Space, texture_loader::TextureLoader, traits::{draw_texture_onto_physics_body, HasPhysics}};
use macroquad::{input::is_mouse_button_released, math::Vec2};
use nalgebra::{point, vector};
use parry2d::query::Ray;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, QueryFilter, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{collider_from_texture_size, player::player::Player, Grabbable, TickContext};


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

    pub fn new(space: &mut Space, pos: Vec2, owner: String, textures: &mut TextureLoader) -> Self {

        let sprite_path = "assets/shotgun.png".to_string();

        let texture_size = textures.get_blocking(&sprite_path).size() 
            * 2.; // scale the size of the shotgun

        dbg!(texture_size);
        
        let rigid_body = space.rigid_body_set.insert(
            RigidBodyBuilder::dynamic()
                .ccd_enabled(true)
                .position(vector![pos.x, pos.y].into())
                .build()
        );

        dbg!(collider_from_texture_size(texture_size).build().shape().as_cuboid().unwrap().half_extents);

        let collider = space.collider_set.insert_with_parent(
            collider_from_texture_size(texture_size)
                .mass(1.)
                .build(), 
            rigid_body, 
            &mut space.rigid_body_set
        );

        Self {
            picked_up: false,
            collider,
            rigid_body,
            sprite: sprite_path,
            selected: false,
            dragging: false,
            drag_offset: None,
            grabbing: false,
            owner
        }
    }

    pub fn spawn(space: &mut Space, pos: Vec2, shotguns: &mut Vec<Shotgun>, owner: String, textures: &mut TextureLoader) {

        shotguns.push(
            Self::new(space, pos, owner, textures)
        );
    }

    pub fn owner_tick(&mut self, players: &mut Vec<Player>, space: &mut Space, ctx: &mut TickContext) {
        ctx.owned_rigid_bodies.push(self.rigid_body);
        ctx.owned_colliders.push(self.collider);
    }

    pub fn all_tick(&mut self, players: &mut Vec<Player>, space: &mut Space, ctx: &mut TickContext) {
        self.grab(players, space, ctx);
    }

    pub fn tick(&mut self, players: &mut Vec<Player>, space: &mut Space, ctx: &mut TickContext) {

        if *ctx.uuid == self.owner {
            self.owner_tick(players, space, ctx);
            self.fire(space, players);
        }

        self.all_tick(players, space, ctx);
        
    }   

    pub fn fire(&mut self, space: &mut Space, players: &mut Vec<Player>) {
        
        if !is_mouse_button_released(macroquad::input::MouseButton::Left) {
            return;
        }

        let player_pos = space.rigid_body_set.get(self.rigid_body).unwrap().position().translation;

        let ray = Ray::new(point![player_pos.x, player_pos.y], vector![1.0, 0.]);
        let max_toi = 4.0;
        let solid = true;
        let filter = QueryFilter::default();

        if let Some((handle, toi)) = space.query_pipeline.cast_ray(&space.rigid_body_set, &space.collider_set, &ray, max_toi, solid, filter)
        {
            // The first collider hit has the handle `handle` and it hit after
            // the ray travelled a distance equal to `ray.dir * toi`.
            let hit_point = ray.point_at(toi); // Same as: `ray.origin + ray.dir * toi`
            println!("Collider {:?} hit at point {}", handle, hit_point);
        }

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

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, flip_x: bool, flip_y: bool) {

        draw_texture_onto_physics_body(
            self.rigid_body, 
            self.collider, 
            space, 
            &self.sprite, 
            textures, 
            flip_x, 
            flip_y, 
            0.
        ).await

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