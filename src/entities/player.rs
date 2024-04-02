use diff::Diff;
use macroquad::{color::RED, input::{self, MouseButton}};
use serde::{Deserialize, Serialize};

use crate::game::{Collidable, Controllable, Damagable, Draggable, Friction, TickContext, HasOwner, HasRect, Moveable, Scale, Sound, Texture, Tickable, Velocity};
use crate::time::Time;
use crate::proxies::macroquad::{input::KeyCode, math::{vec2::Vec2, rect::Rect}};
use crate::proxies::uuid::lib::Uuid;

use super::Entity;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Player {
    pub rect: Rect,
    pub velocity: Vec2,
    pub friction_coefficient: f32,
    pub health: i32,
    pub acceleration: f32,
    pub texture_path: String,
    pub attacking: bool,
    pub last_attack: Time,
    pub scale: u32,
    pub up_bind: KeyCode,
    pub down_bind: KeyCode,
    pub left_bind: KeyCode,
    pub right_bind: KeyCode,
    pub sound_path: String,
    pub dragging: bool,
    pub owner: Uuid
}

impl Player {
    pub fn new(owner: Uuid) -> Self {
        Self {
            rect: Rect {x: 30.0, y: 30.0, w: 85.0, h: 100.0},
            scale: 5,
            acceleration: 30.0,
            texture_path: "assets/player.png".to_string(),
            velocity: Vec2{x: 0.0, y: 0.0},
            health: 100,
            friction_coefficient: 20.,
            up_bind: KeyCode::W,
            down_bind: KeyCode::S,
            left_bind: KeyCode::A,
            right_bind: KeyCode::D,
            attacking: false,
            last_attack: Time::now(),
            sound_path: "assets/sounds/pickup.wav".to_string(),
            dragging: false,
            owner: owner
        }
    }

    fn sneak(&mut self) {
        if input::is_key_down(input::KeyCode::LeftShift) {
            self.set_acceleration(5.0)
        }
        
        else {
            self.set_acceleration(30.0)
        }

    }
    
    fn attack(&mut self, game: &mut TickContext) {

        if !input::is_mouse_button_down(MouseButton::Left) {

            self.set_texture_path("assets/player.png".to_string());

            return
        }

        if self.last_attack.elapsed().num_seconds() < 1 {
            
            return
        }
        
        self.last_attack = Time::now();

        let attack_hitbox = Rect::new(self.get_rect().right(), self.get_rect().y, 50.0, 100.0);

        macroquad::shapes::draw_rectangle(attack_hitbox.x, attack_hitbox.y, attack_hitbox.w, attack_hitbox.h, RED);

        self.attacking = true;
       

        self.set_texture_path("assets/player_attack.png".to_string());

        for entity in game.game_state.entities.iter_mut() {

            // only attack if entity is a player
            match entity {
                Entity::Player(player) => {
                    player.damage(10)
                },
                Entity::Tree(tree) => {

                    if self.get_rect().overlaps(&tree.get_rect()) {
                        tree.damage(10);

                        println!("{} tree health", tree.get_health())
                    }
                    
                },
                _ => {}
            }

        }
    }
}

impl HasOwner for Player {
    fn get_owner(&self) -> Uuid {
        self.owner
    }

    fn set_owner(&mut self, uuid: Uuid) {
        self.owner = uuid
    }
}

impl Tickable for Player {
    fn tick(&mut self, game: &mut crate::game::TickContext) {

        {
            self.control(game.last_tick.elapsed());

        }

        {
            for entity in game.game_state.entities.iter_mut() {

                match entity {
                    Entity::Player(player) => {
                        self.collide(player, game.last_tick.elapsed());
                    }
                    _ => {}
                }

            }
        }

        self.sneak();
        
        {
            self.attack(game)
        }

        {
            self.move_by_velocity(game.last_tick.elapsed());
        }

        {
            self.drag(); 
        }
        
        {
            self.apply_friction(game.last_tick.elapsed());
        }

    }
}


impl Draggable for Player {
    fn get_dragging(&self) -> bool {
        self.dragging
    }

    fn set_dragging(&mut self, dragging: bool) {
        self.dragging = dragging
    }
}

impl Sound for Player {
    fn get_sound_path(&self) -> String {
        self.sound_path.clone()
    }

    fn set_sound_path(&mut self, sound_path: String) {
       self.sound_path = sound_path; 
    }  
}

impl Damagable for Player {
    fn get_health(&self) -> i32 {
        self.health
    }

    fn set_health(&mut self, health: i32) {
        self.health = health
    }
}

impl HasRect for Player {
    fn get_rect(&self) -> Rect {
        self.rect
    }
    fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}

impl Velocity for Player {
    fn get_velocity(&self) -> Vec2 {
        self.velocity
    }

    fn set_velocity(&mut self, velocity: Vec2) {
        self.velocity = velocity
    }
}

impl Texture for Player {
    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }

    fn set_texture_path(&mut self, texture_path: String) {
        self.texture_path = texture_path;
    }
}

impl Scale for Player {
    fn get_scale(&self) -> u32 {
        self.scale
    }
}

impl Moveable for Player {}

impl Collidable for Player {}

impl Controllable for Player {
    fn get_acceleration(&self) -> f32 {
        self.acceleration
    }

    fn set_acceleration(&mut self, acceleration: f32) {
        self.acceleration = acceleration;
    }
    fn up_bind(&mut self) -> KeyCode {
        self.up_bind
    }
    fn down_bind(&mut self) -> KeyCode {
        self.down_bind
    }
    fn left_bind(&mut self) -> KeyCode {
        self.left_bind
    }
    fn right_bind(&mut self) -> KeyCode {
        self.right_bind
    }
}

impl Friction for Player {
    fn friction_coefficient(&self) -> f32 {
        self.friction_coefficient   
    }
}


