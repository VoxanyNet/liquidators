use diff::Diff;
use gamelibrary::{animation::{Frames, TrackedFrames}, current_unix_millis, rapier_mouse_world_pos, space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::{ev::Code, Button, Gamepad};
use macroquad::{color::WHITE, input::{is_key_down, is_key_released, is_mouse_button_down, is_mouse_button_released, KeyCode}, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams}};
use nalgebra::{vector, Rotation, Rotation2};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBody, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{brick::Brick, level::Level, portal_bullet::PortalBullet, portal_gun::PortalGun, shotgun::Shotgun, structure, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum Facing {
    Right,
    Left
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Player {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
    pub sprite_path: String,
    pub owner: String,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub max_speed: Vec2,
    pub shotgun: Option<Shotgun>,
    pub portal_gun: PortalGun,
    pub animation_handler: PlayerAnimationHandler,
    pub walk_frame_progess: f32,
    pub idle_frame_progress: f32,
    pub facing: Facing
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum PlayerAnimationState {
    Walking,
    Idle,
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
// used to track the state of player animation
pub struct PlayerAnimationHandler {
    state: PlayerAnimationState,
    pub walking_frames: TrackedFrames,
    pub idle_frames: TrackedFrames
}

impl PlayerAnimationHandler {

    pub fn new(initial_state: PlayerAnimationState) -> Self {
        
        Self {
            state: initial_state,
            walking_frames: TrackedFrames::load_from_directory(&"assets/player/walk".to_string()),
            idle_frames: TrackedFrames::load_from_directory(&"assets/player/idle".to_string())
        }
    }

    pub fn get_current_frame(&self) -> &String {
        match self.state {
            PlayerAnimationState::Walking => {
                self.walking_frames.current_frame()
            },
            PlayerAnimationState::Idle => {
                self.idle_frames.current_frame()
            }
        }
    }

    pub fn set_frame(&mut self, state: PlayerAnimationState, new_frame_index: usize) {
        match state {
            PlayerAnimationState::Walking => {
                self.walking_frames.set_frame(new_frame_index);
            },
            PlayerAnimationState::Idle => {
                self.idle_frames.set_frame(new_frame_index);
            }
        }
    }

    pub fn set_animation_state(&mut self, new_state: PlayerAnimationState) {
        self.state = new_state;
    }

    pub fn next_frame(&mut self, state: PlayerAnimationState) {
        match state {
            PlayerAnimationState::Walking => {
                self.walking_frames.next_frame();
            },
            PlayerAnimationState::Idle => {
                self.idle_frames.next_frame();
            }
        }
    }
}

impl Player {

    pub fn spawn(players: &mut Vec<Player>, space: &mut Space, owner: String, position: &Vec2) {

        let rigid_body = RigidBodyBuilder::dynamic()
            .position(vector![position.x, position.y].into())
            .soft_ccd_prediction(20.)
            .ccd_enabled(true)
            .lock_rotations()
            .build();

        let collider = ColliderBuilder::cuboid(21., 30.)
            .mass(100.)
            .build();

        let rigid_body_handle = space.rigid_body_set.insert(rigid_body);
        let collider_handle = space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut space.rigid_body_set);

        players.push(
            Player {
                rigid_body: rigid_body_handle,
                collider: collider_handle,
                sprite_path: "assets/player/idle.png".to_string(),
                owner,
                selected: false,
                dragging: false,
                drag_offset: None,
                max_speed: vec2(350., 80.),
                shotgun: None,
                portal_gun: PortalGun::new(),
                animation_handler: PlayerAnimationHandler::new(PlayerAnimationState::Walking),
                walk_frame_progess: 0.,
                idle_frame_progress: 0.,
                facing: Facing::Right
                
            }
        )
    }

    pub fn tick(&mut self, level: &mut Level, ctx: &mut TickContext) {
        //self.launch_brick(level, ctx);
        self.control(level, ctx);
        self.update_selected(&mut level.space, &ctx.camera_rect);
        self.update_is_dragging(&mut level.space, &ctx.camera_rect);
        self.update_drag(&mut level.space, &ctx.camera_rect);
        self.own_nearby_structures(level, ctx);
        self.update_walk_animation(&mut level.space);
        self.update_portal_gun_pos(&level.space);
        self.fire_portal_gun(ctx.camera_rect, &mut level.portal_bullets);
        self.update_idle_animation(&mut level.space);
        self.change_facing_direction(&level.space);
        
    }

    pub fn change_facing_direction(&mut self, space: &Space) {
        let velocity = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap().linvel();

        if velocity.x > 0. {
            self.facing = Facing::Right
        }

        if velocity.x < 0. {
            self.facing = Facing::Left
        }
    }
    pub fn update_idle_animation(&mut self, space: &mut Space) {

        let velocity = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap().linvel();

        if velocity.x == 0. {

            self.idle_frame_progress += 0.1; 
            
            self.animation_handler.set_animation_state(PlayerAnimationState::Idle);

            if self.idle_frame_progress > 1. {
                self.animation_handler.next_frame(PlayerAnimationState::Idle);
                self.idle_frame_progress = 0.;
            }
            
        }

    }
    pub fn update_walk_animation(&mut self, space: &mut Space) {
        let velocity = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap().linvel();

        if velocity.x.abs() > 0. {
            self.walk_frame_progess += velocity.x.abs() / 2000.;

            self.animation_handler.set_animation_state(PlayerAnimationState::Walking);
        }

        if self.walk_frame_progess > 1. {
            self.animation_handler.next_frame(PlayerAnimationState::Walking);

            self.walk_frame_progess = 0.;
        }
        

        


    }
    pub fn own_nearby_structures(&mut self, level: &mut Level, ctx: &mut TickContext) {
        // take ownership of nearby structures to avoid network physics delay

        let our_body = level.space.rigid_body_set.get(self.rigid_body).unwrap();

        for structure in &mut level.structures {
            let structure_body = level.space.rigid_body_set.get(structure.rigid_body_handle).unwrap();

            let body_distance = structure_body.position().translation.vector - our_body.position().translation.vector;

            // if the other body is within 100 units, we take ownership of it
            if body_distance.abs().magnitude() > 100. {
                continue;
            }

            match &mut structure.owner {
                Some(owner) => {

                    // take ownership if we dont already own it
                    if owner == ctx.uuid {
                        continue;    
                    }

                    if current_unix_millis() - structure.last_ownership_change < 2 {
                        continue;
                    }

                    println!("taking ownership of structure!");
                        
                    *owner = ctx.uuid.clone();

                }
                None => {
                    structure.owner = Some(ctx.uuid.clone())
                },
            }

        }
    }
    pub fn fire_portal_gun(&mut self, camera_rect: &Rect, portal_bullets: &mut Vec<PortalBullet>) {
        if is_mouse_button_released(macroquad::input::MouseButton::Left) {
            //self.portal_gun.fire(camera_rect, portal_bullets);
        }
    }

    pub fn update_portal_gun_pos(&mut self, space: &Space) {

        let body = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap();

        self.portal_gun.position.x = body.position().translation.x;
        self.portal_gun.position.y = body.position().translation.y;
    }

    pub fn launch_brick(&mut self, level: &mut Level, ctx: &mut TickContext) {

        if !is_mouse_button_down(macroquad::input::MouseButton::Left) {
            return;
        }
        let (player_pos, player_rotation, player_velocity) = {
            let body = level.space.rigid_body_set.get(*self.rigid_body_handle()).unwrap();
            (body.position().clone(), body.rotation().clone(), body.linvel().clone())
        };

        let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

        let mouse_body_distance = mouse_pos - Vec2::new(player_pos.translation.x, player_pos.translation.y); 
        
        let brick_spawn_point = Vec2::new(
            player_pos.translation.x + mouse_body_distance.normalize().x * 40., 
            player_pos.translation.y + mouse_body_distance.normalize().y * 40.
        );

        let brick = Brick::new(&mut level.space, brick_spawn_point, Some(ctx.uuid.clone()));
        
        let brick_body = level.space.rigid_body_set.get_mut(*brick.rigid_body_handle()).unwrap();

        let mut brick_velocity = player_velocity;
        brick_velocity.x += 5000. * mouse_body_distance.normalize().x;
        
        brick_body.set_rotation(player_rotation, true);
        brick_body.set_linvel(brick_velocity, true);

        level.bricks.push(brick);

    }

    pub fn jump(&mut self, rigid_body: &mut RigidBody, gamepad: Option<Gamepad>) {
        if is_key_down(KeyCode::Space) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::South)}) {

            // dont allow if moving if falling or jumping

            if rigid_body.linvel().y.abs() > 0.5 {
                return
            }

            // stop any y axis movement if going down
            if rigid_body.linvel().y.is_sign_negative() {
                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x, 0.],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x, rigid_body.linvel().y + 700.],
                true
            );

            
        }
    }

    pub fn control(&mut self, level: &mut Level, ctx: &mut TickContext) {

        let rigid_body = level.space.rigid_body_set.get_mut(self.rigid_body).unwrap();

        let gamepad: Option<Gamepad<'_>> = None;

        self.jump(rigid_body, gamepad);

        if is_key_down(KeyCode::A) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::DPadLeft)}) {

            if rigid_body.linvel().x < -self.max_speed.x {
                return
            }

            if rigid_body.linvel().x.is_sign_positive() {
                rigid_body.set_linvel(
                    vector![0., rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x - 10., rigid_body.linvel().y],
                true
            );

        }

        if is_key_down(KeyCode::D) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::DPadRight)}) {

            if rigid_body.linvel().x > self.max_speed.x {
                return
            }

            if rigid_body.linvel().x.is_sign_negative() {
                rigid_body.set_linvel(
                    vector![0., rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x + 10., rigid_body.linvel().y],
                true
            )
        }

        // if gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::RightTrigger)}) {
        //     rigid_body.set_rotation(
        //         Rotation, wake_up
        //     );
        // }

    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {

        let mut flip_x = false;
        
        match self.facing {
            Facing::Left => {
                flip_x = true
            }
            _ => {},
        }

        self.draw_texture(&space, &self.animation_handler.get_current_frame(), textures, flip_x, false).await;
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