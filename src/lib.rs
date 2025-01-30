use console::Console;
use diff::Diff;
use gamelibrary::{rapier_mouse_world_pos, space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::{GamepadId, Gilrs};
use macroquad::{input::{is_mouse_button_down, mouse_delta_position}, math::{Rect, Vec2}, prelude::camera::mouse};
use nalgebra::vector;
use rapier2d::prelude::RigidBodyHandle;
use serde::{Deserialize, Serialize};

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
pub mod arm;

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
    pub last_tick: &'a web_time::Instant,
    pub uuid: &'a String,
    pub camera_rect: &'a Rect,
    pub camera_offset: &'a mut Vec2,
    pub active_gamepad: &'a Option<GamepadId>,
    pub console: &'a mut Console
}

#[derive(PartialEq, Serialize, Deserialize, Diff)]
pub struct SoundHandle {
    id: i32
}