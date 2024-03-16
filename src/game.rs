use std::collections::HashMap;
use std::time::{Duration, Instant};

use macroquad::audio::{self, load_sound};
use macroquad::color::WHITE;
use macroquad::input::{self};
use macroquad::math::{self, Vec2};
use macroquad::texture::{self, load_texture, Texture2D};

use crate::entities::Entity;
use crate::game_state::GameState;

pub struct Game {
    pub game_state: GameState,
    pub textures: HashMap<String, Texture2D>,
    pub sounds: HashMap<String, macroquad::audio::Sound>,
    pub last_tick: Instant
}

impl Game {
    pub async fn draw(&mut self) {
        for entity in self.game_state.entities.iter_mut() {

            match entity {
                Entity::Player(player) => {player.draw(&mut self.textures).await}
                Entity::Zombie(zombie) => {zombie.draw(&mut self.textures).await}
                Entity::Bullet(bullet) => {bullet.draw()},
                Entity::Coin(coin) => {coin.draw()},
                Entity::Tree(tree) => {tree.draw(&mut self.textures).await},
                Entity::Wood(wood) => {wood.draw(&mut self.textures).await}
            };
        }
    }

    pub fn tick(&mut self) {

        for index in 0..self.game_state.entities.len() {

            // take the player out, tick it, then put it back in
            let mut entity = self.game_state.entities.swap_remove(index);

            match entity {
                Entity::Player(ref mut player) => {player.tick(self)}
                Entity::Zombie(ref mut _zombie) => {} // zombie doesnt have a tick method yet
                Entity::Bullet(ref mut bullet) => {bullet.tick(self)},
                Entity::Coin(ref mut coin) => {coin.tick(self)},
                Entity::Wood(ref mut _wood) => {},
                Entity::Tree(ref mut tree) => {tree.tick(self)},
                
            };
            
            // put the entity back
            self.game_state.entities.push(entity);

        }

        self.last_tick = Instant::now(); 

    }
}

pub trait Velocity {
    fn get_velocity(&self) -> macroquad::math::Vec2;
    fn set_velocity(&mut self, velocity: macroquad::math::Vec2);
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

pub trait Breakable: Damagable + Rect {
    fn highlight(&mut self) {
        let mouse_pos = Vec2::from(macroquad::input::mouse_position());

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

pub trait Collidable: Rect + Velocity {

    fn collide(&mut self, collider: &mut dyn Collidable, dt: Duration) {

        // check where our rect will be when it next moves
        let mut next_rect = self.get_rect().clone();

        next_rect.x += self.get_velocity().x * dt.as_millis() as f32;
        next_rect.y += self.get_velocity().y * dt.as_millis() as f32;

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
pub trait Friction: Rect + Velocity {
    fn apply_friction(&mut self, dt: Duration) {

        self.set_velocity(
            self.get_velocity() + ((-self.get_velocity() * self.friction_coefficient()) * dt.as_secs_f32())
        );
    }

    fn friction_coefficient(&self) -> f32;
}

pub trait Controllable: Rect + Velocity {
    fn control(&mut self, dt: Duration) {

        let mut velocity = self.get_velocity();
        let acceleration = self.get_acceleration();

        if macroquad::input::is_key_down(self.right_bind()) {
            velocity.x += acceleration * dt.as_millis() as f32;
        }

        if macroquad::input::is_key_down(self.left_bind()) {
            velocity.x -= acceleration * dt.as_millis() as f32

        }

        if macroquad::input::is_key_down(self.up_bind()) {
            velocity.y -= acceleration * dt.as_millis() as f32
        }

        if macroquad::input::is_key_down(self.down_bind()) {
            velocity.y += acceleration * dt.as_millis() as f32
        }

        // update to the final velocity
        self.set_velocity(
            velocity
        );

    }

    fn get_acceleration(&self) -> f32;
    fn set_acceleration(&mut self, acceleration: f32);

    fn up_bind(&mut self) -> macroquad::input::KeyCode;
    fn down_bind(&mut self) -> macroquad::input::KeyCode;
    fn left_bind(&mut self) -> macroquad::input::KeyCode;
    fn right_bind(&mut self) -> macroquad::input::KeyCode;
}

pub trait Moveable: Rect + Velocity {
    fn move_by_velocity(&mut self, dt: Duration) {

        let mut rect = self.get_rect();

        rect.x += self.get_velocity().x * dt.as_secs_f32();
        rect.y += self.get_velocity().y * dt.as_secs_f32();

        self.set_rect(rect);
    }
}
pub trait Rect {
    fn get_rect(&self) -> macroquad::math::Rect;
    fn set_rect(&mut self, rect: macroquad::math::Rect);
}

pub trait Color {
    fn color(&self) -> macroquad::color::Color;
}

pub trait Drawable: Rect + Color {
    fn draw(&mut self) {
        macroquad::shapes::draw_rectangle(self.get_rect().x, self.get_rect().y, self.get_rect().w, self.get_rect().h, self.color());
    }
}

pub trait Texture: Rect + Scale {
    async fn draw(&self, textures: &mut HashMap<String, Texture2D>) {
        
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
            self.get_rect().x,
            self.get_rect().y,
            WHITE,
            macroquad::texture::DrawTextureParams {
                dest_size: Some(scaled_texture_size),
                ..Default::default()
            },
         );

    }

    fn get_texture_path(&self) -> String;

    fn set_texture_path(&mut self, texture_path: String);
}

pub trait Tickable {
    fn tick(&mut self, game: &mut Game);
}

pub trait Scale {
    fn get_scale(&self) -> u32;
}

pub trait Draggable: Rect + Velocity {
    fn drag(&mut self) {

        if input::is_mouse_button_down(input::MouseButton::Left) & self.get_rect().contains(Vec2::from(input::mouse_position())) {
            self.set_dragging(true)
        }

        if input::is_mouse_button_released(input::MouseButton::Left) {
            self.set_dragging(false)
        }

        if !self.get_dragging() {
            return;
        }

        let mouse_pos = Vec2::from(macroquad::input::mouse_position());

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
