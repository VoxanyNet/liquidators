use std::{fs, path::Path, time::{Duration, Instant}};

use console::Console;
use diff::Diff;
use futures::executor::block_on;
use gamelibrary::{font_loader::FontLoader, rapier_mouse_world_pos, sound::soundmanager::SoundManager, space::{Space, SyncColliderHandle, SyncImpulseJointHandle, SyncRigidBodyHandle}, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::GamepadId;
use macroquad::{audio::{load_sound, play_sound_once}, input::{is_mouse_button_down, mouse_delta_position}, math::{Rect, Vec2}};
use nalgebra::{coordinates::X, vector};
use noise::Perlin;
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
pub mod teleporter;
pub mod grenade;
pub mod main_menu;
pub mod enemy;
pub mod pixel;
pub mod damage_number;
pub mod bullet_trail;
pub mod muzzle_flash;
pub mod pistol;
pub mod weapon;
pub mod intake;
pub mod server;
pub mod editor_client;
pub mod editor_server;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct BodyCollider {
    body: RigidBodyHandle,
    collider: ColliderHandle
}

// #[derive(Serialize)]
// // this is temporary for diff serialize reasons
// pub struct Sound {
//     #[serde(skip)]
//     sound: macroquad::audio::Sound,
//     path: String,
//     playing: bool
// }

// impl<'de> Deserialize<'de> for Sound {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: serde::Deserializer<'de> {
//         #[derive(Deserialize)]
//         pub struct SoundHelper {
//             path: String
//         }

//         let helper = SoundHelper::deserialize(deserializer)?;

//         // this is an issue. we need to be able to use a cache
//         let sound = block_on(load_sound(&helper.path)).unwrap();

//         Ok(
//             Self {
//                 sound,
//                 path: helper.path,
//                 playing: false
//             }
//         )
//     }
// }

// pub struct SoundDiff {
//     play: bool,
//     path: String
// }


// impl Diff for Sound {
//     type Repr = SoundDiff;

//     fn diff(&self, other: &Self) -> Self::Repr {
//         if self.sound.
//     }

//     fn apply(&mut self, diff: &Self::Repr) {
//         todo!()
//     }

//     fn identity() -> Self {
//         todo!()
//     }
// }



// impl Sound {
//     pub fn new(path: &str) -> Self {

//         let sound = block_on(
//             load_sound(path)
//         ).unwrap();

//         Self {
//             sound: sound,
//             path: path.to_string()
//         }
//     }

//     pub fn play(&mut self) {
//         play_sound_once(&self.sound);
//     }
// }

// used to identify an entity within an hashmap
pub struct EntityKey {
    
}

pub fn collider_from_texture_size(texture_size: Vec2) -> ColliderBuilder {
    ColliderBuilder::cuboid(texture_size.x / 2., texture_size.y / 2.)
}

// this should be moved to gamelibrary but i dont want to update it right now
pub fn collider_contains_point(collider_handle: ColliderHandle, space: &mut Space, point: Vec2) -> bool {
    let mut contains_point: bool = false;

    space.query_pipeline.update(&space.sync_collider_set.collider_set);

    space.query_pipeline.intersections_with_point(
        &space.sync_rigid_body_set.rigid_body_set, &space.sync_collider_set.collider_set, &point![point.x, point.y], QueryFilter::default(), |handle| {
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

        let body = space.sync_rigid_body_set.get_sync_mut(*self.rigid_body_handle()).unwrap();

        let current_velocity = body.linvel();

        let mouse_movement = mouse_delta_position();


        //println!("{}", vector![current_velocity.x - (mouse_movement.x * 1000.), current_velocity.y + (mouse_movement.y * 1000.)].to_string());

        body.set_linvel(
            vector![current_velocity.x + (mouse_movement.x * -1000.), current_velocity.y + (mouse_movement.y * 1000.)].into(), 
            true
        );

    }

    /// Update whether this entity is being grabbed
    fn update_grabbing(&mut self, space: &mut Space, camera_rect: &Rect, max_grab_distance: Vec2, reference_body_handle: SyncRigidBodyHandle) {
        let body = space.sync_rigid_body_set.get_sync(*self.rigid_body_handle()).unwrap();

        let reference_body = space.sync_rigid_body_set.get_sync(reference_body_handle).unwrap();

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
    pub camera_rect: &'a mut Rect,
    pub camera_offset: &'a mut Vec2,
    pub active_gamepad: &'a Option<GamepadId>,
    pub console: &'a mut Console,
    pub owned_rigid_bodies: &'a mut Vec<SyncRigidBodyHandle>,
    pub owned_colliders: &'a mut Vec<SyncColliderHandle>,
    pub owned_impulse_joints: &'a mut Vec<SyncImpulseJointHandle>,
    pub sounds: &'a mut dyn SoundManager,
    pub last_tick_mouse_world_pos: &'a mut Vec2,
    pub font_loader: &'a mut FontLoader,
    pub screen_shake: &'a mut ScreenShakeParameters,
    pub last_tick_duration: Duration
}

pub struct ScreenShakeParameters {
    pub x_intensity: f64,
    pub x_frequency: f64,
    pub x_offset: f64,
    pub x_noise: Perlin,
    pub x_intensity_decay: f64,
    pub x_frequency_decay: f64,

    pub y_intensity: f64,
    pub y_frequency: f64,
    pub y_offset: f64,
    pub y_noise: Perlin,
    pub y_intensity_decay: f64,
    pub y_frequency_decay: f64

}

impl ScreenShakeParameters {
    pub fn default(x_seed: Option<u32>, y_seed: Option<u32>) -> Self {

        let x_seed = match x_seed {
            Some(seed) => seed,
            None => 69420,
        };

        let y_seed = match y_seed {
            Some(seed) => seed,
            None => 42069,
        };


        Self {
            x_intensity: 0.,
            x_frequency: 0.,
            x_offset: 0.,
            x_noise: Perlin::new(x_seed),
            x_intensity_decay: 0.,
            x_frequency_decay: 0.,

            y_intensity: 0.,
            y_frequency: 0.,
            y_offset: 0.,
            y_noise: Perlin::new(y_seed),
            y_intensity_decay: 0.,
            y_frequency_decay: 0.,
        }
    }
}