use std::time::Instant;

use parry2d::math::Real;
use diff::Diff;
use gamelibrary::{animation::TrackedFrames, current_unix_millis, draw_texture_rapier, get_angle_to_mouse, rapier_mouse_world_pos, space::Space, swapiter::SwapIter, syncsound::{SoundHandle, Sounds}, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::Gamepad;
use macroquad::{color::WHITE, input::{is_key_down, is_key_released, is_mouse_button_down, is_mouse_button_released, KeyCode}, math::{vec2, Rect, Vec2}, texture::DrawTextureParams, time::get_frame_time};
use nalgebra::{vector, UnitComplex};
use parry2d::math::Point;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, InteractionGroups, QueryFilter, RigidBody, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{arm::Arm, brick::Brick, level::Level, portal_bullet::PortalBullet, portal_gun::PortalGun, shotgun::Shotgun, structure::Structure, TickContext};

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
    pub left_arm: Arm,
    pub right_arm: Arm,
    pub sprite_scale: f32,
    pub sound: SoundHandle
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

        // fortnite battle pass why do i suffer so much from these stupid small annoyances if i was just like that but not with all this shit then i would be so much more productive i am a slave to my physical body i wish for nothing more that to be free from the prison that is my body dear god save me from this existence i am more than this
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

    pub fn spawn(players: &mut Vec<Player>, space: &mut Space, owner: String, position: &Vec2, textures: &mut TextureLoader) {

        let sprite_scale = 2.2;
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

        let left_arm = Arm::new(
            space, 
            rigid_body_handle, 
            Point::<Real>::new(0. * sprite_scale, 2. * sprite_scale), 
            Point::<Real>::new(0., 0.),
            textures, 
            "assets/player/gunrunarm_bottom.png".to_string(), 
            1.75
        );

        let right_arm = Arm::new(
            space,
            rigid_body_handle,
            Point::<Real>::new(0. * sprite_scale, 2. * sprite_scale),
            Point::<Real>::new(0., 0.),
            textures,
            "assets/player/gunrunarm_top.png".to_string(),
            1.75
        );

        let sound = SoundHandle::new("assets/sounds/brick_land.wav", [0.,0.,0.]);

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
                arm_angle: 0.,
                left_arm,
                right_arm,
                sprite_scale,
                sound
            }
        )
    }

    pub fn sync_sound(&mut self, sounds: &mut Sounds) {
        sounds.sync_sound(&mut self.sound);
    }

    pub fn pickup_shotgun(&mut self, level: &mut Level, _ctx: &mut TickContext) {

        let player_shape = level.space.collider_set.get(self.collider).unwrap().shape();
        let player_pos = level.space.rigid_body_set.get(self.rigid_body).unwrap().position();

        let _colliding_collider = match level.space.query_pipeline.intersection_with_shape(
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

    pub fn tick(&mut self, space: &mut Space, structures: &mut Vec<Structure>, ctx: &mut TickContext, _players: &mut Vec<Player>) {
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
        self.delete_structure(structures, space, ctx);
        self.update_arm_anchor_pos(space);
        self.sync_sound(ctx.sounds);

        if is_key_released(KeyCode::N) {

            self.sound = SoundHandle::new("assets/sounds/brick_land.wav", [0., 0., 0.]);
            
            self.sound.play();
        }
        //self.update_hitbox_size(space, ctx);

        self.update_arm_angle(space, ctx.camera_rect);
        
        self.add_owned_physics(ctx);

        self.left_arm.tick(ctx);
        self.right_arm.tick(ctx);
        
    }

    pub fn add_owned_physics(&self, ctx: &mut TickContext) {
        ctx.owned_rigid_bodies.push(self.rigid_body);
        ctx.owned_colliders.push(self.collider);
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
            let (_structures, mut structure) = structures_iter.next();

            if structure.contains_point(space, rapier_mouse_world_pos(ctx.camera_rect)) {
                structure.despawn(space);

                continue;
            }

            structures_iter.restore(structure);
        }

        println!("iterate over structures: {:?}", then.elapsed());
        
    }

    pub fn update_arm_angle(&mut self, space: &mut Space, camera_rect: &Rect) {


        let body_pos = space.rigid_body_set.get(self.rigid_body).unwrap().translation().clone();
        let angle_to_mouse = get_angle_to_mouse(Vec2::new(body_pos.x, body_pos.y), camera_rect);

        // left arm
        let left_arm_body = space.rigid_body_set.get_mut(self.left_arm.get_rigid_body_handle()).unwrap();
        left_arm_body.set_rotation(UnitComplex::new(-angle_to_mouse + 1.7), true);


        // right arm
        let right_arm_body = space.rigid_body_set.get_mut(self.right_arm.get_rigid_body_handle()).unwrap();
        right_arm_body.set_rotation(UnitComplex::new(-angle_to_mouse + 1.7), true);

        // need to restrict arm angle depending on facing
    }

    pub fn change_facing_direction(&mut self, space: &Space) {
        let velocity = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap().linvel();

        if velocity.x > 0.5 {
            self.facing = Facing::Right
        }

        if velocity.x < -0.5 {
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
    /// Update arm anchor pos depending on facing direction
    pub fn update_arm_anchor_pos(&mut self, space: &mut Space) {
        let arm_anchor_pos = match self.facing {
            Facing::Right => Point::<Real>::new(0. * self.sprite_scale, 2. * self.sprite_scale),
            Facing::Left => Point::<Real>::new(3. * self.sprite_scale, 7. * self.sprite_scale),
        };

        let left_arm_joint = space.impulse_joint_set.get_mut(
            self.left_arm.get_joint_handle()
        )
            .unwrap()
            .data
            .as_revolute_mut()
            .unwrap();

        left_arm_joint.set_local_anchor1(arm_anchor_pos);

        let right_arm_joint = space.impulse_joint_set.get_mut(
            self.right_arm.get_joint_handle()
        )
            .unwrap()
            .data
            .as_revolute_mut()
            .unwrap();

        right_arm_joint.set_local_anchor1(arm_anchor_pos);
        
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
    pub fn fire_portal_gun(&mut self, _camera_rect: &Rect, _portal_bullets: &mut Vec<PortalBullet>) {
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

    pub fn control(&mut self, space: &mut Space, _ctx: &mut TickContext) {

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

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, _camera_rect: &Rect) {

        //draw_hitbox(space, self.rigid_body, self.collider);
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
        // match &self.shotgun {
        //     Some(shotgun) => {
        //         shotgun.draw(space, textures, flip_x).await;
        //     },
        //     None => {},
        // }

        // get the player position. all draws for the player are relative to this
        let player_position = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap().translation();

        // arms are drawn seperately from the body because they move to follow the camera

        
        let texture = textures.get(self.animation_handler.get_current_frame()).await.weak_clone();

        let mut draw_params = DrawTextureParams::default();

        draw_params.dest_size = Some(
            Vec2::new(texture.width() * 2.2, texture.height() * 2.2)
        );
        draw_params.flip_x = flip_x;
        
        //self.draw_collider(space).await;

        let draw_offset_x: f32;
        let draw_offset_y: f32;

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
            &texture, 
            player_position.x + draw_offset_x, 
            player_position.y + draw_offset_y, 
            WHITE, 
            draw_params
        );

        self.left_arm.draw(space, textures, &self.facing).await;
        self.right_arm.draw(space, textures, &self.facing).await;


        // draw top arm
        match self.animation_handler.get_animation_state() {

            PlayerAnimationState::GunRun => {

                let _texture_scale = 2.5;

                

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