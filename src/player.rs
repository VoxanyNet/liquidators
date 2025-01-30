use std::{clone, time::Instant};

use diff::Diff;
use futures::future;
use gamelibrary::{animation::{Frames, TrackedFrames}, current_unix_millis, draw_texture_rapier, rapier_mouse_world_pos, rapier_to_macroquad, rotate_point, space::Space, swapiter::SwapIter, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::{ev::Code, Button, Gamepad};
use macroquad::{camera::Camera2D, color::WHITE, input::{is_key_down, is_key_released, is_mouse_button_down, is_mouse_button_released, KeyCode}, math::{vec2, Rect, Vec2}, texture::{draw_texture_ex, DrawTextureParams}, time::get_frame_time};
use nalgebra::{vector, Rotation, Rotation2};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, InteractionGroups, QueryFilter, RigidBody, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{arm::Arm, brick::Brick, level::Level, portal_bullet::PortalBullet, portal_gun::PortalGun, shotgun::Shotgun, structure::{self, Structure}, TickContext};

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
    shotgun: Option<Shotgun>,
    pub portal_gun: PortalGun,
    pub animation_handler: PlayerAnimationHandler,
    pub walk_frame_progess: f32,
    pub idle_frame_progress: f32,
    pub facing: Facing,
    pub arm_angle: f32,
    // pub left_arm: Arm,
    // pub right_arm: Arm
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum PlayerAnimationState {
    Walking,
    Idle,
    GunRun
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
// used to track the state of player animation
pub struct PlayerAnimationHandler {
    state: PlayerAnimationState,
    walking_frames: TrackedFrames,
    idle_frames: TrackedFrames,
    gunrun_frames: TrackedFrames
}

impl PlayerAnimationHandler {

    pub fn new(initial_state: PlayerAnimationState) -> Self {
        
        Self {
            state: initial_state,
            walking_frames: TrackedFrames::load_from_directory(&"assets/player/walk".to_string()),
            idle_frames: TrackedFrames::load_from_directory(&"assets/player/idle".to_string()),
            gunrun_frames: TrackedFrames::load_from_directory(&"assets/player/gunrun".to_string())
        }
    }

    pub fn get_current_frame(&self) -> &String {
        match self.state {
            PlayerAnimationState::Walking => {
                self.walking_frames.current_frame()
            },
            PlayerAnimationState::Idle => {
                self.idle_frames.current_frame()
            },
            PlayerAnimationState::GunRun => {
                self.gunrun_frames.current_frame()
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
            },
            PlayerAnimationState::GunRun => {
                self.gunrun_frames.set_frame(new_frame_index);
            }
        }
    }

    pub fn set_animation_state(&mut self, new_state: PlayerAnimationState) {
        self.state = new_state;
    }

    pub fn get_animation_state(&self) -> PlayerAnimationState {
        self.state.clone()
    }

    pub fn next_frame(&mut self, state: PlayerAnimationState) {
        match state {
            PlayerAnimationState::Walking => {
                self.walking_frames.next_frame();
            },
            PlayerAnimationState::Idle => {
                self.idle_frames.next_frame();
            },
            PlayerAnimationState::GunRun => {
                self.gunrun_frames.next_frame();
            }
        }
    }
}

impl Player {

    pub fn despawn(self, space: &mut Space) {
        // removes the body AND the collider!
        space.rigid_body_set.remove(
            self.rigid_body, 
            &mut space.island_manager, 
            &mut space.collider_set, 
            &mut space.impulse_joint_set, 
            &mut space.multibody_joint_set, 
            true
        );
    } 

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

        let shotgun = Shotgun::new(space, *position, owner.clone());
        // we dont want the shotgun to collide with anything
        space.collider_set.get_mut(shotgun.collider).unwrap().set_collision_groups(
            InteractionGroups::none()
        );

        //let left_arm = Arm::new(space, rigid_body_handle, body_anchor_point, arm_anchor_point, initial_pos, textures, sprite_path, sprite_scale);

        players.push(
            Player {
                rigid_body: rigid_body_handle,
                collider: collider_handle,
                sprite_path: "assets/player/idle.png".to_string(),
                owner: owner.clone(),
                selected: false, 
                dragging: false,
                drag_offset: None,
                max_speed: vec2(350., 80.),
                shotgun: Some(shotgun),
                portal_gun: PortalGun::new(),
                animation_handler: PlayerAnimationHandler::new(PlayerAnimationState::Walking),
                walk_frame_progess: 0.,
                idle_frame_progress: 0.,
                facing: Facing::Right,
                arm_angle: 0.
            }
        )
    }

    pub fn pickup_shotgun(&mut self, level: &mut Level, ctx: &mut TickContext) {

        let player_shape = level.space.collider_set.get(self.collider).unwrap().shape();
        let player_pos = level.space.rigid_body_set.get(self.rigid_body).unwrap().position();

        let colliding_collider = match level.space.query_pipeline.intersection_with_shape(
            &level.space.rigid_body_set,
            &level.space.collider_set,
            player_pos, 
            player_shape, 
            QueryFilter::default()
        ) {
            Some(collider_collider) => collider_collider,
            None => return,
        };

        // for shotgun_index in &level.shotguns {
        //     if shotgun.collider == colliding_collider {
                
        //     }
        // }
    }

    pub fn update_shotgun_pos(&mut self, space: &mut Space) {

        let shotgun = match &self.shotgun {
            Some(shotgun) => shotgun,
            None => return,
        };
        
        let player_pos = {
            space.rigid_body_set.get(self.rigid_body).unwrap().translation().clone()
        };

        let shotgun_body = space.rigid_body_set.get_mut(shotgun.rigid_body).unwrap();

        let shotgun_collider = space.collider_set.get(shotgun.collider).unwrap(); 

        let player_collider = space.collider_set.get(self.collider).unwrap();

        

        let shotgun_length = shotgun_collider.shape().as_cuboid().unwrap().half_extents.x;
        let player_length = player_collider.shape().as_cuboid().unwrap().half_extents.x;
        
        let shotgun_offset = match self.facing {
            Facing::Right => shotgun_length + player_length,
            Facing::Left => -1. * (shotgun_length + player_length),
        };
        
        // shotgun_body.set_position(
        //     vector![player_pos.x + shotgun_offset, player_pos.y].into(), 
        //     true
        // );


        let new_pos = rotate_point(vec2(shotgun_body.translation().x, shotgun_body.translation().y), vec2(player_pos.x, player_pos.y), self.arm_angle);

        shotgun_body.set_position(
            vector![new_pos.x, new_pos.y].into(), true
        );

    }
    pub fn tick(&mut self, space: &mut Space, structures: &mut Vec<Structure>, ctx: &mut TickContext, players: &mut Vec<Player>) {
        //self.launch_brick(level, ctx);
        self.control(space, ctx);
        self.update_selected(space, &ctx.camera_rect);
        self.update_is_dragging(space, &ctx.camera_rect);
        self.update_drag(space, &ctx.camera_rect);
        self.own_nearby_structures(space, structures, ctx);
        self.update_walk_animation(space);
        self.update_portal_gun_pos(&space);
        //self.fire_portal_gun(ctx.camera_rect, &mut portal_bullets);
        self.update_idle_animation(space);
        self.change_facing_direction(&space);
        self.update_shotgun_pos(space);
        self.delete_structure(structures, space, ctx);
        //self.update_hitbox_size(space, ctx);

        self.update_arm_angle(space, ctx.camera_rect);
        
    }

    pub fn update_hitbox_size(&mut self, space: &mut Space, ctx: &mut TickContext) {
        let current_frame = self.animation_handler.get_current_frame();

        let texture = futures::executor::block_on(ctx.textures.get(current_frame));

        let collider = space.collider_set.get_mut(self.collider).unwrap();

        let collider_shape = collider.shape_mut().as_cuboid_mut().unwrap();
        
        if collider_shape.half_extents.x != texture.size().x {
            collider_shape.half_extents.x = texture.size().x;
        }

        if collider_shape.half_extents.y != texture.size().y {
            collider_shape.half_extents.y = texture.size().y;
        }   
        
    }

    pub fn delete_structure(&mut self, structures: &mut Vec<Structure>, space: &mut Space, ctx: &mut TickContext) {
        if !is_key_released(KeyCode::Backspace) {
            return
        }

        let mut structures_iter = SwapIter::new(structures);

        let then = Instant::now();
        while structures_iter.not_done() {
            let (structures, mut structure) = structures_iter.next();

            if structure.contains_point(space, rapier_mouse_world_pos(ctx.camera_rect)) {
                structure.despawn(space);

                continue;
            }

            structures_iter.restore(structure);
        }

        println!("iterate over structures: {:?}", then.elapsed());
        
    }

    pub fn angle_to_mouse(&self, space: &Space, camera_rect: &Rect) -> f32 {
        
        let player_body = space.rigid_body_set.get(self.rigid_body).unwrap();
        let mouse_pos = rapier_mouse_world_pos(camera_rect);

        let distance_to_mouse = Vec2::new(
            mouse_pos.x - player_body.translation().x,
            mouse_pos.y - player_body.translation().y 
        );

        distance_to_mouse.x.atan2(distance_to_mouse.y)

    }

    pub fn update_arm_angle(&mut self, space: &mut Space, camera_rect: &Rect) {
    
        self.arm_angle = self.angle_to_mouse(space, camera_rect) - 1.3;

        //println!("{}", self.angle_to_mouse(space, camera_rect));

        return;

        // need to restrict arm angle depending on facing
        match self.facing {
            Facing::Right => {
                self.arm_angle = self.arm_angle.clamp(0.2 - 1.3, 2.7 - 1.3);
            },
            Facing::Left => {
                self.arm_angle = self.arm_angle.clamp(-2.7 - 1.3, -0.8 - 1.3)
            },
        }

        println!("{}", self.arm_angle);
        
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

            self.idle_frame_progress += 5. * get_frame_time(); 
            
            //self.animation_handler.set_animation_state(PlayerAnimationState::Idle);

            if self.idle_frame_progress > 1. {
                self.animation_handler.next_frame(PlayerAnimationState::Idle);
                self.idle_frame_progress = 0.;
            }
            
        }

    }
    pub fn update_walk_animation(&mut self, space: &mut Space) {
        let velocity = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap().linvel();

        if velocity.x.abs() > 0. {
            self.walk_frame_progess += (velocity.x.abs() * get_frame_time()) / 20.;

            self.animation_handler.set_animation_state(PlayerAnimationState::GunRun);
        }

        if self.walk_frame_progess > 1. {

            // this all seems pretty stupid
            let animation_state = self.animation_handler.get_animation_state();

            match animation_state {
                PlayerAnimationState::Walking => {},
                PlayerAnimationState::GunRun => {},
                _ => return
            }

            self.animation_handler.next_frame(animation_state);

            self.walk_frame_progess = 0.;
        }
        

        


    }
    pub fn own_nearby_structures(&mut self, space: &mut Space, structures: &mut Vec<Structure>, ctx: &mut TickContext) {
        // take ownership of nearby structures to avoid network physics delay

        let our_body = space.rigid_body_set.get(self.rigid_body).unwrap();

        for structure in structures {

            if current_unix_millis() - structure.last_ownership_change < 2 {
                continue;
            }

            let structure_body = space.rigid_body_set.get(structure.rigid_body_handle).unwrap();

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

    pub fn control(&mut self, space: &mut Space, ctx: &mut TickContext) {

        let rigid_body = space.rigid_body_set.get_mut(self.rigid_body).unwrap();

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

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, camera_rect: &Rect) {

        // we use this to determine if we need to flip the texture for the body
        let mut flip_x = false;
        
        // flip texture if facing left
        match self.facing {
            Facing::Left => {
                flip_x = true
            }
            _ => {},
        }  

        //self.draw_collider(space).await;

        // draw shotgun (we update its position every tick)
        match &self.shotgun {
            Some(shotgun) => {
                shotgun.draw(space, textures, flip_x).await;
            },
            None => {},
        }

        // get the player position. all draws for the player are relative to this
        let player_position = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap().translation();

        // arms are drawn seperately from the body because they move to follow the camera
        
        // check if we need to draw bottom arm
        match self.animation_handler.get_animation_state() {
            PlayerAnimationState::GunRun => {
                
                let texture_scale = 1.75;

                // get the texture for the bottom arm
                let texture = textures.get(&"assets/player/gunrunarm_bottom.png".to_string()).await;

                // parameters for drawing arm
                let mut params = DrawTextureParams::default();

                // bottom arm draw pos starts at the center of the player's body, and then we modify it to be where we want it
                // this is in RAPIER COORDINATES, not macroquad
                // this value is where the TOP LEFT of the texture is
                let mut bottom_arm_draw_pos = Vec2::new(player_position.x, player_position.y);

                // set the size the bottom arm - just multiply the dimensions of the texture by a value so its not stretched / squashed
                params.dest_size = Some(
                    Vec2::new(texture.size().x * texture_scale, texture.size().y * texture_scale)
                );

                // set the angle of the arm to the arm angle set in the tick method
                params.rotation = self.arm_angle;

                // move the draw position up by 16 pixels, we want the arm to sit slightly higher than the center of the body
                bottom_arm_draw_pos.y += 16.;

                bottom_arm_draw_pos.x -= 7.;
                
                
                // calculate the position where we are going to actually draw the arm, we need this for the next step
                // this position is the top left of the texture
                let macroquad_draw_pos = rapier_to_macroquad(&bottom_arm_draw_pos);
                
                // set the pivot point of the arm to the shoulder instead of the center of arm
                // this pivot point is relative to the screen space, not the texture, thats why we needed the draw pos

                params.pivot = Some(
                    Vec2::new(
                        macroquad_draw_pos.x,
                        macroquad_draw_pos.y + (4.5 * texture_scale) // we want to pivot slightly lower than the top left of the arm, so we add to the y value because we are in macroquad coords. also we need to scale it by the number we used in the dest_size value
                    )
                );

                params.flip_y = match self.facing {
                    Facing::Right => false,
                    Facing::Left => true,
                };

                println!("bottom arm: {:?}", params.pivot);
                
                // actually draw the texture
                draw_texture_rapier(
                    texture, 
                    bottom_arm_draw_pos.x, 
                    bottom_arm_draw_pos.y, 
                    WHITE, 
                    params
                );
            },
            _ => {}
        }   

        
        let texture = textures.get(self.animation_handler.get_current_frame()).await;

        let mut draw_params = DrawTextureParams::default();

        draw_params.dest_size = Some(
            Vec2::new(texture.width() * 2.2, texture.height() * 2.2)
        );
        draw_params.flip_x = flip_x;
        
        self.draw_collider(space).await;

        let mut draw_offset_x = 0.;
        let mut draw_offset_y = 0.;

        match self.animation_handler.get_animation_state() {
            PlayerAnimationState::Walking => {
                draw_offset_x = -20.;
                draw_offset_y = 36.;
            },  
            PlayerAnimationState::Idle => {
                draw_offset_x = -20.;
                draw_offset_y = 36.;
            },
            PlayerAnimationState::GunRun => {
                draw_offset_x = -30.;
                draw_offset_y = 31.;
            },
        }

        draw_texture_rapier(
            texture, 
            player_position.x + draw_offset_x, 
            player_position.y + draw_offset_y, 
            WHITE, 
            draw_params
        );


        // draw top arm
        match self.animation_handler.get_animation_state() {

            PlayerAnimationState::GunRun => {

                let texture_scale = 2.5;

                // get the texture for the top arm
                let texture = textures.get(&"assets/player/gunrunarm_top.png".to_string()).await;

                // parameters for drawing arm
                let mut params = DrawTextureParams::default();

                // top arm draw pos starts at the center of the player's body, and then we modify it to be where we want it
                // this is in RAPIER COORDINATES, not macroquad
                // this value is where the TOP LEFT of the texture is
                let mut top_arm_draw_pos = Vec2::new(player_position.x, player_position.y);

                // set the size the bottom arm - just multiply the dimensions of the texture by a value so its not stretched / squashed
                params.dest_size = Some(
                    Vec2::new(texture.size().x * texture_scale, texture.size().y * texture_scale)
                );
                
                // set the angle of the arm to the arm angle set in the tick method
                params.rotation = self.arm_angle;

                top_arm_draw_pos.x -= 7.;

                // we want the top arm to sit above the center of the body
                top_arm_draw_pos.y += 16.;

                // calculate the position where we are going to actually draw the arm, we need this for the next step
                // this position is the top left of the texture
                let macroquad_draw_pos = rapier_to_macroquad(&top_arm_draw_pos);


                // set the pivot point of the arm to the shoulder instead of the center of arm
                // this pivot point is relative to the screen space, not the texture, thats why we needed the draw pos
                params.pivot = Some(
                    Vec2::new(
                        macroquad_draw_pos.x + (2.5 * texture_scale), 
                        macroquad_draw_pos.y + (3. * texture_scale) // also we need to scale it by the number we used in the dest_size value
                    )
                );

                println!("top arm: {:?}", params.pivot);
                

                params.flip_y = match self.facing {
                    Facing::Right => false,
                    Facing::Left => true,
                };
                
                draw_texture_rapier(
                    texture, 
                    top_arm_draw_pos.x, 
                    top_arm_draw_pos.y, 
                    WHITE, 
                    params
                );

            },
            _ => {}
        }

        
        
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