use diff::Diff;
use gamelibrary::{get_angle_to_mouse, rapier_mouse_world_pos, rapier_to_macroquad, sound::soundmanager::SoundHandle, space::Space, sync_arena::SyncArena, texture_loader::TextureLoader, traits::{draw_texture_onto_physics_body, HasPhysics}};
use macroquad::{color::RED, input::is_mouse_button_released, math::Vec2, shapes::draw_rectangle};
use nalgebra::{point, vector};
use parry2d::query::Ray;
use rapier2d::prelude::{ColliderHandle, QueryFilter, RigidBodyBuilder, RigidBodyHandle};
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
    pub picked_up: bool,
    pub sounds: Vec<SoundHandle> // is this the best way to do this?
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
            owner,
            sounds: vec![]
        }
    }

    pub fn spawn(space: &mut Space, pos: Vec2, shotguns: &mut Vec<Shotgun>, owner: String, textures: &mut TextureLoader) {

        shotguns.push(
            Self::new(space, pos, owner, textures)
        );
    }

    pub fn owner_tick(&mut self, players: &mut SyncArena<Player>, hit_markers: &mut Vec<Vec2>, space: &mut Space, ctx: &mut TickContext) {
        ctx.owned_rigid_bodies.push(self.rigid_body);
        ctx.owned_colliders.push(self.collider);

        self.fire(space, players, hit_markers, ctx);
        self.sync_sound(ctx);
    }

    pub fn all_tick(&mut self, players: &mut SyncArena<Player>, space: &mut Space, ctx: &mut TickContext) {
        self.grab(players, space, ctx);
    }

    pub fn tick(&mut self, players: &mut SyncArena<Player>, space: &mut Space, hit_markers: &mut Vec<Vec2>, ctx: &mut TickContext) {

        if *ctx.uuid == self.owner {
            self.owner_tick(players, hit_markers, space, ctx);
        }

        self.all_tick(players, space, ctx);
        
    }   

    pub fn sync_sound(&mut self, ctx: &mut TickContext) {
        for sound_handle in &mut self.sounds {
            ctx.sounds.sync_sound(sound_handle);
        }
    }

    pub fn fire(&mut self, space: &mut Space, players: &mut SyncArena<Player>, hit_markers: &mut Vec<Vec2>, ctx: &mut TickContext) {
        
        if !is_mouse_button_released(macroquad::input::MouseButton::Left) {
            return;
        }

        

        let player_pos = space.rigid_body_set.get(self.rigid_body).unwrap().position().translation;

        let mut fire_sound = SoundHandle::new("assets/sounds/shotgun/fire.wav", [0., 0., 0.]);

        fire_sound.play();

        self.sounds.push(fire_sound);


        let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

        let mouse_vector = Vec2::new(
            mouse_pos.x - player_pos.x,
            mouse_pos.y - player_pos.y 
        ).normalize();

        let ray = Ray::new(point![player_pos.x, player_pos.y], vector![mouse_vector.x, mouse_vector.y]);
        let max_toi = 1000.0;
        let solid = true;
        let filter = QueryFilter::default();

        space.query_pipeline.intersections_with_ray(&space.rigid_body_set, &space.collider_set, &ray, max_toi, solid, filter, 
        |handle, intersection| {

            if self.collider == handle {
                return true;
            }

            let collider_pos = space.collider_set.get(handle).unwrap().position().translation;

            

            let macroquad_hit_pos = rapier_to_macroquad(&Vec2::new(collider_pos.x, collider_pos.y));

            hit_markers.push(macroquad_hit_pos);


            true
        });


    }

    pub fn grab(&mut self, players: &mut SyncArena<Player>, space: &mut Space, ctx: &mut TickContext) {

        if self.picked_up {
            return;
        }

        // we need to have a more efficient way of finding the currently controlled player
        for (index, player) in players {
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