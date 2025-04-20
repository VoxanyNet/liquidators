use std::{fs, path::Path, time::Instant};

use console::Console;
use diff::Diff;
use gamelibrary::{rapier_mouse_world_pos, sound::soundmanager::SoundManager, space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::GamepadId;
use macroquad::{input::{is_mouse_button_down, mouse_delta_position}, math::{Rect, Vec2}};
use nalgebra::vector;
use rand::{rng, seq::IndexedRandom};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, QueryFilter, RigidBodyHandle};
use serde::{Deserialize, Serialize};
use nalgebra::point;

pub mod game_state;
pub mod physics_square;
pub mod level;
pub mod structure;
pub mod shotgun;
pub mod player;
pub mod radio;
pub mod chat;
pub mod vec_remove_iter;
pub mod brick;
pub mod portal;
pub mod portal_bullet;
pub mod portal_gun;
pub mod console;
pub mod sky;
pub mod boat;
pub mod teleporter;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct BodyCollider {
    body: RigidBodyHandle,
    collider: ColliderHandle
}

// used to identify an entity within an hashmap
pub struct EntityKey {
    
}

pub fn collider_from_texture_size(texture_size: Vec2) -> ColliderBuilder {
    ColliderBuilder::cuboid(texture_size.x / 2., texture_size.y / 2.)
}

// this should be moved to gamelibrary but i dont want to update it right now
pub fn collider_contains_point(collider_handle: ColliderHandle, space: &mut Space, point: Vec2) -> bool {
    let mut contains_point: bool = false;

    space.query_pipeline.update(&space.collider_set);

    space.query_pipeline.intersections_with_point(
        &space.rigid_body_set, &space.collider_set, &point![point.x, point.y], QueryFilter::default(), |handle| {
            if collider_handle == handle {
                contains_point = true;
                return false
            }

            return true
        }
    );

    contains_point
} 

pub fn get_random_file_from_dir(dir: &str) -> Option<String> {
    let path = Path::new(dir);

    if let Ok(entries) = fs::read_dir(path) {
        let files: Vec<String> = entries
            .filter_map(|entry| entry.ok()) // Filter out errors
            .filter(|entry| entry.path().is_file()) // Only keep files
            .filter_map(|entry| entry.path().to_str().map(String::from)) // Convert paths to strings
            .collect();

        let mut rng = rng();
        files.choose(&mut rng).cloned() // Pick a random file
    } else {
        None
    }
}

pub trait Grabbable: HasPhysics {

    fn grabbing(&mut self) -> &mut bool; 

    /// Update the entity's velocity if being grabbed
    fn update_grab_velocity(&mut self, space: &mut Space) {

        // dont update grab velocity if not being grabbed
        if !*self.grabbing() {
            return
        }

        let body = space.rigid_body_set.get_mut(*self.rigid_body_handle()).unwrap();

        let current_velocity = body.linvel();

        let mouse_movement = mouse_delta_position();


        //println!("{}", vector![current_velocity.x - (mouse_movement.x * 1000.), current_velocity.y + (mouse_movement.y * 1000.)].to_string());

        body.set_linvel(
            vector![current_velocity.x + (mouse_movement.x * -1000.), current_velocity.y + (mouse_movement.y * 1000.)].into(), 
            true
        );

    }

    /// Update whether this entity is being grabbed
    fn update_grabbing(&mut self, space: &mut Space, camera_rect: &Rect, max_grab_distance: Vec2, reference_body_handle: RigidBodyHandle) {
        let body = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap();

        let reference_body = space.rigid_body_set.get(reference_body_handle).unwrap();

        let distance = Vec2::new(
            body.translation().x - reference_body.translation().x,
            body.translation().y - reference_body.translation().y
        );

        if distance.x.abs() > max_grab_distance.x.abs() {
            *self.grabbing() = false;
            return;
        }

        if distance.y.abs() > max_grab_distance.y.abs() {
            *self.grabbing() = false;
            return;
        }

        if !is_mouse_button_down(macroquad::input::MouseButton::Left) {
            *self.grabbing() = false;
            return;
        }

        // mouse must be inside the collider 
        // might want to add padding around the collider so the user has more space to click?
        if !self.contains_point(space, rapier_mouse_world_pos(camera_rect)) {

            // dont update self.grabbing() so user can keep dragging even if their mouse isnt perfectly in the collider
            return

        }

        *self.grabbing() = true;

    }
}

pub struct TickContext<'a> {
    pub is_host: &'a mut bool,
    pub textures: &'a mut TextureLoader,
    pub last_tick: &'a Instant,
    pub uuid: &'a String,
    pub camera_rect: &'a Rect,
    pub camera_offset: &'a mut Vec2,
    pub active_gamepad: &'a Option<GamepadId>,
    pub console: &'a mut Console,
    pub owned_rigid_bodies: &'a mut Vec<RigidBodyHandle>,
    pub owned_colliders: &'a mut Vec<ColliderHandle>,
    pub sounds: &'a mut dyn SoundManager
}