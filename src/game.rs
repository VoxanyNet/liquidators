use std::collections::HashMap;

use chrono::TimeDelta;
use diff::Diff;
use macroquad::audio::{self, load_sound};
use macroquad::color::WHITE;
use macroquad::input::{self};
use macroquad::texture::{self, load_texture, Texture2D};
use macroquad::window::screen_height;
use serde::{Deserialize, Serialize};

use crate::collider::Collider;
use crate::game_state::GameState;
use crate::proxies::macroquad::{input::KeyCode, math::{vec2::Vec2, rect::Rect}};
use crate::rigid_body::RigidBody;
use crate::space::{RigidBodyHandle, Space};
use crate::time::Time;

pub struct TickContext<'a> {
    pub game_state: &'a mut GameState,
    pub is_host: &'a mut bool,
    pub textures: &'a mut HashMap<String, Texture2D>,
    pub sounds: &'a mut HashMap<String, macroquad::audio::Sound>,
    pub last_tick: &'a mut Time,
    pub uuid: &'a mut String
}

pub trait Velocity {
    fn get_velocity(&self) -> Vec2;
    fn set_velocity(&mut self, velocity: Vec2);
}

pub trait Damagable {
    fn damage(&mut self, damage: i32) {
        self.set_health(
            self.get_health() - damage
        )
    }

    fn get_health(&self) -> i32;
    fn set_health(&mut self, health: i32);
}

pub trait Breakable: Damagable + HasRect {
    fn highlight(&mut self) {

        let mouse_pos = Vec2::new(
            macroquad::input::mouse_position().0,
            macroquad::input::mouse_position().1
        );

        if self.get_rect().contains(mouse_pos) {
            self.set_highlighted(true);
        }

        else {
            self.set_highlighted(false);
        }
    }

    fn get_highlighted(&self) -> bool;
    fn set_highlighted(&mut self, highlighted: bool);
}

pub trait Collidable: HasRect + Velocity {

    fn collide(&mut self, collider: &mut dyn Collidable, dt: TimeDelta) {

        // check where our rect will be when it next moves
        let mut next_rect = self.get_rect().clone();

        next_rect.x += self.get_velocity().x * dt.num_milliseconds() as f32;
        next_rect.y += self.get_velocity().y * dt.num_milliseconds() as f32;

        if collider.get_rect().overlaps(&mut next_rect) {
            
            // add our velocity to the collider
            collider.set_velocity(
                collider.get_velocity() + self.get_velocity()
            );

            // invert current velocity
            self.set_velocity(
                self.get_velocity() * -0.05
            );

        }

    }
}

fn round(x: f32) -> f32 {
    f32::trunc(x  * 10.0) / 10.0 // or f32::trunc
}

pub trait Friction: HasRect + Velocity {
    fn apply_friction(&mut self, dt: TimeDelta) {

        self.set_velocity(
            self.get_velocity() + ((-self.get_velocity() * self.friction_coefficient()) * (dt.num_milliseconds() as f32 / 1000.))
        );

        self.set_velocity(
            Vec2::new(round(self.get_velocity().x), round(self.get_velocity().y))
        );
    }

    fn friction_coefficient(&self) -> f32;
}

pub trait Controllable: HasRect + Velocity {
    fn control(&mut self, dt: TimeDelta) {

        let mut velocity = self.get_velocity();
        let acceleration = self.get_acceleration();

        if macroquad::input::is_key_down(self.right_bind().into()) {
            velocity.x += acceleration * dt.num_milliseconds() as f32;
        }

        if macroquad::input::is_key_down(self.left_bind().into()) {
            velocity.x -= acceleration * dt.num_milliseconds() as f32

        }

        if macroquad::input::is_key_down(self.up_bind().into()) {
            velocity.y -= acceleration * dt.num_milliseconds() as f32
        }

        if macroquad::input::is_key_down(self.down_bind().into()) {
            velocity.y += acceleration * dt.num_milliseconds() as f32
        }

        // update to the final velocity
        self.set_velocity(
            velocity
        );

    }

    fn get_acceleration(&self) -> f32;
    fn set_acceleration(&mut self, acceleration: f32);

    fn up_bind(&mut self) -> KeyCode;
    fn down_bind(&mut self) -> KeyCode;
    fn left_bind(&mut self) -> KeyCode;
    fn right_bind(&mut self) -> KeyCode;
}

pub trait Moveable: HasRect + Velocity {
    fn move_by_velocity(&mut self, dt: TimeDelta) {

        let mut rect = self.get_rect();

        //println!("{}", self.get_velocity().x * (dt.num_milliseconds() as f32 / 1000.));

        rect.x += self.get_velocity().x * (dt.num_milliseconds() as f32 / 1000.);
        rect.y += self.get_velocity().y * (dt.num_milliseconds() as f32 / 1000.);

        self.set_rect(rect);
    }
}
pub trait HasRect {
    fn get_rect(&self) -> Rect;
    fn set_rect(&mut self, rect: Rect);
}

// pub trait HasCollider {
//     fn get_collider(&self) -> Collider;
//     fn set_collider(&mut self, collider: Collider);
// }



pub trait Color {
    fn color(&self) -> crate::proxies::macroquad::color::Color;
}

pub trait Drawable: HasRect + Color {
    fn draw(&mut self, camera_offset: &Vec2) {
        macroquad::shapes::draw_rectangle(self.get_rect().x, self.get_rect().y, self.get_rect().w, self.get_rect().h, self.color().into());
    }
}

pub trait HasRigidBody {
    fn get_rigid_body_handle(&self) -> &RigidBodyHandle;

    async fn draw(&mut self, camera_offset: &Vec2, space: &Space) {
        let rigid_body_handle = self.get_rigid_body_handle();
        let rigid_body = space.get_rigid_body(rigid_body_handle).expect("Invalid rigid body handle");

        macroquad::shapes::draw_rectangle(
            rigid_body.position.x + rigid_body.collider.hx, 
            ((rigid_body.position.y + rigid_body.collider.hy) * -1.) + screen_height(), 
            rigid_body.collider.hx * 2., 
            rigid_body.collider.hy * 2., 
            WHITE
        )
    }
}

pub trait Texture: HasRect + Scale {
    async fn draw(&self, textures: &mut HashMap<String, Texture2D>, camera_offset: &Vec2) {
        
        // load texture if not already
        if !textures.contains_key(&self.get_texture_path()) {
            let texture = load_texture(&self.get_texture_path()).await.unwrap();
            
            texture.set_filter(texture::FilterMode::Nearest);

            textures.insert(self.get_texture_path(), texture);
        }

        let texture = textures.get(&self.get_texture_path()).unwrap();

        let scaled_texture_size = Vec2 {
            x: texture.width() * self.get_scale() as f32,
            y: texture.height() * self.get_scale() as f32
        };

        // macroquad::shapes::draw_rectangle(
        //     self.get_rect().x,
        //     self.get_rect().y,
        //     self.get_rect().w, 
        //     self.get_rect().h,
        //     color::RED
        // );

        macroquad::texture::draw_texture_ex(
            texture,
            self.get_rect().x + camera_offset.x,
            self.get_rect().y + camera_offset.y,
            WHITE,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(scaled_texture_size.into()),
                ..Default::default()
            },
         );

    }

    fn get_texture_path(&self) -> String;

    fn set_texture_path(&mut self, texture_path: String);
}

pub trait Tickable: HasOwner {
    fn tick(&mut self, context: &mut TickContext);
}

pub trait HasOwner {
    fn get_owner(&self) -> String;

    fn set_owner(&mut self, uuid: String);
}

pub trait Scale {
    fn get_scale(&self) -> u32;
}

pub trait Draggable: HasRect + Velocity {
    fn drag(&mut self) {

        if input::is_mouse_button_down(input::MouseButton::Left) & self.get_rect().contains(Vec2{x: input::mouse_position().0, y: input::mouse_position().1}) {
            self.set_dragging(true)
        }

        if input::is_mouse_button_released(input::MouseButton::Left) {
            self.set_dragging(false)
        }

        if !self.get_dragging() {
            return;
        }

        let mouse_pos = Vec2{x: macroquad::input::mouse_position().0, y: macroquad::input::mouse_position().1};

        let rect = self.get_rect();

        let distance_to_mouse = Vec2::new(
            mouse_pos.x - rect.x,
            mouse_pos.y - rect.y
        );
        
        self.set_velocity(
            distance_to_mouse.normalize() * 1000.
        );

    }

    fn get_dragging(&self) -> bool;

    fn set_dragging(&mut self, dragging: bool);

}

pub trait Sound {
    async fn play_sound(&self, sounds: &mut HashMap<String, macroquad::audio::Sound>) {
        // load texture if not already
        if !sounds.contains_key(&self.get_sound_path()) {
            let sound = load_sound(&self.get_sound_path()).await.unwrap();

            sounds.insert(self.get_sound_path(), sound);
        }

        let sound = sounds.get(&self.get_sound_path()).unwrap();

        audio::play_sound_once(sound);
    }

    fn get_sound_path(&self) -> String;

    fn set_sound_path(&mut self, sound_path: String);

}

