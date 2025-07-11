use std::{collections::HashSet, f32::consts::PI, time::Instant};

use diff::Diff;
use gamelibrary::{animation::TrackedFrames, collider_top_left_pos, current_unix_millis, get_angle_to_mouse, rapier_mouse_world_pos, rapier_to_macroquad, sound::soundmanager::{SoundHandle, SoundManager}, space::{Space, SyncColliderHandle, SyncImpulseJointHandle, SyncRigidBodyHandle}, swapiter::SwapIter, sync_arena::{Index, SyncArena}, texture_loader::TextureLoader, traits::HasPhysics, uuid_u32};
use gilrs::Gamepad;
use macroquad::{color::{GREEN, WHITE}, input::{is_key_down, is_key_released, is_mouse_button_down, is_mouse_button_released, KeyCode}, math::{vec2, Rect, Vec2}, shapes::draw_rectangle, time::get_frame_time};
use nalgebra::vector;
use rapier2d::prelude::{ImpulseJointHandle, InteractionGroups, RevoluteJointBuilder, RigidBody};
use serde::{Deserialize, Serialize};

#[cfg(feature = "3d-audio")]
use gamelibrary::sound::backends::ears::EarsSoundManager as SelectedSoundManager; // this alias needs a better name

#[cfg(not(feature = "3d-audio"))]
use gamelibrary::sound::backends::macroquad::MacroquadSoundManager as SelectedSoundManager;

use crate::{blood::Blood, brick::Brick, bullet_trail::BulletTrail, damage_number::DamageNumber, enemy::Enemy, level::Level, pistol::Pistol, player, portal_bullet::PortalBullet, shotgun::Shotgun, structure::Structure, teleporter::Teleporter, TickContext};

use super::body_part::BodyPart;

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
pub enum PlayerWeapon {
    Shotgun(Shotgun),
    Pistol(Pistol)
}

impl PlayerWeapon {
    pub fn tick(
        &mut self, 
        players: &mut SyncArena<Player>, 
        space: &mut Space, 
        hit_markers: &mut Vec<Vec2>, 
        ctx: &mut TickContext,
        enemies: &mut SyncArena<Enemy>,
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>,
        blood: &mut HashSet<Blood>
    ) {
        match self {
            PlayerWeapon::Shotgun(shotgun) => shotgun.tick(players, space, hit_markers, ctx, enemies, damage_numbers, bullet_trails, blood),
            PlayerWeapon::Pistol(pistol) => pistol.tick(players, space, hit_markers, ctx, enemies, damage_numbers, bullet_trails, blood),
        }
    } 

    pub async fn sync_sound(&mut self, ctx: &mut TickContext<'_>) {
        match self {
            PlayerWeapon::Shotgun(shotgun) => shotgun.sync_sound(ctx).await,
            PlayerWeapon::Pistol(pistol) => pistol.sync_sound(ctx).await,
        }

    }

    pub fn player_joint_handle(&self) -> Option<SyncImpulseJointHandle> {
        match self {
            PlayerWeapon::Shotgun(shotgun) => shotgun.player_joint_handle(),
            PlayerWeapon::Pistol(pistol) => pistol.player_joint_handle(),
        }
    }
    pub fn aim_angle_offset(&self) -> f32 {
        match self {
            PlayerWeapon::Shotgun(shotgun) => shotgun.aim_angle_offset(),
            PlayerWeapon::Pistol(pistol) => pistol.aim_angle_offset(),
        }
    }
    pub fn rigid_body(&self) -> SyncRigidBodyHandle{
        match self {
            PlayerWeapon::Shotgun(shotgun) => shotgun.rigid_body(),
            PlayerWeapon::Pistol(pistol) => pistol.rigid_body(),
        }
    }

    pub fn set_facing(&mut self, facing: Facing) {
        match self {
            PlayerWeapon::Shotgun(shotgun) => shotgun.set_facing(facing),
            PlayerWeapon::Pistol(pistol) => pistol.set_facing(facing),
        }
    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, flip_x: bool, flip_y: bool) {
        match self {
            PlayerWeapon::Shotgun(shotgun) => shotgun.draw(space, textures, flip_x, flip_y).await,
            PlayerWeapon::Pistol(pistol) => pistol.draw(space, textures, flip_x, flip_y).await,
        }
    }
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Player {
    pub id: u64, // lame but required until i can find a better way
    pub health: u32,
    pub head: BodyPart,
    pub body: BodyPart,
    pub sprite_path: String,
    pub owner: String,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub max_speed: Vec2,
    weapon: Option<PlayerWeapon>,
    //pub animation_handler: PlayerAnimationHandler,
    pub walk_frame_progess: f32,
    pub idle_frame_progress: f32,
    pub facing: Facing,
    pub sound: SoundHandle,
    pub head_joint_handle: Option<SyncImpulseJointHandle>,
    pub teleporter_destination: Option<SyncRigidBodyHandle>,
    pub money: u32
}

impl Player {

    pub fn move_camera(&mut self, camera_rect: &mut Rect, space: &Space) {
        let position = space.sync_rigid_body_set.get_sync(self.body.body_handle).unwrap().translation();

        let macroquad_position = rapier_to_macroquad(&vec2(position.x, position.y));

        // if self.rect.right() > camera_rect.right() - 100.{
            
        //     camera_rect.x = (self.rect.right() - camera_rect.w) + 100.;
        // }
        
        if macroquad_position.x > camera_rect.right() - 200. {
            camera_rect.x = (macroquad_position.x - camera_rect.w) + 200.;
        }

        if macroquad_position.x < camera_rect.left() + 200. {
            
            camera_rect.x = macroquad_position.x - 200.
        }

        if macroquad_position.y > camera_rect.bottom() - 100. {
           

            camera_rect.y = (macroquad_position.y - camera_rect.h) + 100.;
        }

        if macroquad_position.y < camera_rect.top() + 100. {
        

            camera_rect.y = macroquad_position.y - 100.
        }


    }

    pub fn despawn(self, space: &mut Space) {

        // fortnite battle pass why do i suffer so much from these stupid small annoyances if i was just like that but not with all this shit then i would be so much more productive i am a slave to my physical body i wish for nothing more that to be free from the prison that is my body dear god save me from this existence i am more than this
        // removes the body AND the collider!
        space.sync_rigid_body_set.remove_sync(
            self.body.body_handle, 
            &mut space.island_manager, 
            &mut space.sync_collider_set, 
            &mut space.sync_impulse_joint_set, 
            &mut space.multibody_joint_set, 
            true
        );

        space.sync_rigid_body_set.remove_sync(
            self.head.body_handle, 
            &mut space.island_manager, 
            &mut space.sync_collider_set, 
            &mut space.sync_impulse_joint_set, 
            &mut space.multibody_joint_set, 
            true
        );

        match self.weapon {
            Some(weapon) => {
                space.sync_rigid_body_set.remove_sync(
                    weapon.rigid_body(), 
                    &mut space.island_manager, 
                    &mut space.sync_collider_set, 
                    &mut space.sync_impulse_joint_set, 
                    &mut space.multibody_joint_set, 
                    true
                );
            },
            None => {},
        }


    } 
    
    pub fn spawn(players: &mut SyncArena<Player>, space: &mut Space, owner: String, position: &Vec2, textures: &mut TextureLoader) {

        let cat_head = BodyPart::new(
            "assets/cat/head.png".to_string(), 
            2, 
            10.,
            *position, 
            space, 
            textures, 
            owner.clone(),
            Vec2::new(30., 28.)
        );

        let cat_body = BodyPart::new(
            "assets/cat/body.png".to_string(), 
            2, 
            100.,
            *position, 
            space, 
            textures, 
            owner.clone(),
            Vec2::new(22., 19.)
        );

        // lock the rotation of the cat body
        space.sync_rigid_body_set.get_sync_mut(cat_body.body_handle).unwrap().lock_rotations(true, true);

        let local_cat_head_body = space.sync_rigid_body_set.get_local_handle(cat_head.body_handle);
        let local_cat_body_body = space.sync_rigid_body_set.get_local_handle(cat_body.body_handle);

        // joint the head to the body
        let head_joint_handle = space.sync_impulse_joint_set.insert_sync(
            local_cat_body_body,
            local_cat_head_body,
            RevoluteJointBuilder::new()
                .local_anchor1(vector![0., 0.].into())
                .local_anchor2(vector![0., -30.].into())
                .limits([-0.4, 0.4])
                .contacts_enabled(false)
            .build(),
            true
        );

        let pistol = Pistol::new(space, *position, owner.clone(), Some(cat_body.body_handle), textures, Facing::Right);
        //let pistol = Shotgun::new(space, *position, owner.clone(), Some(cat_body.body_handle), textures);

        let sound = SoundHandle::new("assets/sounds/brick_land.wav", [0.,0.,0.]);

        players.insert(
            Player {
                id: uuid_u32() as u64,
                head: cat_head,
                body: cat_body,
                sprite_path: "assets/player/idle.png".to_string(),
                owner: owner.clone(),
                selected: false, 
                dragging: false,
                drag_offset: None,
                max_speed: vec2(350., 80.),
                weapon: Some(pistol.into()),
                //animation_handler: PlayerAnimationHandler::new(PlayerAnimationState::Walking),
                walk_frame_progess: 0.,
                idle_frame_progress: 0.,
                facing: Facing::Right,
                sound,
                head_joint_handle: Some(head_joint_handle),
                teleporter_destination: None,
                health: 100,
                money: 0
            }
        );
    }


    pub async fn sync_sound(&mut self, ctx: &mut TickContext<'_>) {
        if let Some(weapon) = &mut self.weapon {
            weapon.sync_sound(ctx).await
        }
    }

    pub fn change_weapon(
        &mut self, 
        space: &mut Space,
        textures: &mut TextureLoader
    ) {
        if !is_key_released(KeyCode::Q) {
            return;
        }

        let rigid_body_handle = self.rigid_body_handle().clone();
        let owner = self.owner.clone();

        if let Some(weapon) = &mut self.weapon {
            let new_weapon = match weapon {
                PlayerWeapon::Shotgun(_shotgun) => {
                    Pistol::new(space, vec2(0., 0.), self.owner.clone(), Some(rigid_body_handle.clone()), textures, self.facing.clone()).into()
                },
                PlayerWeapon::Pistol(_pistol) => {
                    Shotgun::new(space, vec2(0., 0.), self.owner.clone(), Some(rigid_body_handle.clone()), textures, self.facing.clone()).into()
                },  
            };

            *weapon = new_weapon;
        }
    }


    pub fn tick(
        &mut self, 
        space: &mut Space, 
        structures: &mut Vec<Structure>, 
        bricks: &mut Vec<Brick>, 
        teleporters: &mut Vec<Teleporter>, 
        hit_markers: &mut Vec<Vec2>, 
        ctx: &mut TickContext, 
        players: &mut SyncArena<Player>,
        enemies: &mut SyncArena<Enemy>,
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>,
        blood: &mut HashSet<Blood>
    ) {
        //self.launch_brick(level, ctx);
        self.change_weapon(space, ctx.textures);
        self.control(space, ctx);
        self.move_camera(ctx.camera_rect, space);
        self.update_selected(space, &ctx.camera_rect);
        self.update_is_dragging(space, &ctx.camera_rect);
        self.update_drag(space, &ctx.camera_rect);
        // this needs to be fixed so moving structures dont change owners, it causes it to glitch because conflicting updates
        
        let then =web_time::Instant::now();
        self.own_nearby_structures(space, structures, ctx, players);

        //self.update_walk_animation(space);
        //self.update_idle_animation(space);
        self.change_facing_direction(&space);
        self.delete_structure(structures, space, ctx);
        self.angle_head_to_mouse(space, ctx.camera_rect);
        self.place_teleporter(ctx, teleporters, space);
        self.angle_weapon_to_mouse(space, ctx.camera_rect);
        //self.launch_brick(bricks, space, ctx);
        
        self.detach_head_if_dead(space);

        if let Some(weapon) = &mut self.weapon {
            weapon.tick(
                players, 
                space, 
                hit_markers, 
                ctx,
                enemies,
                damage_numbers,
                bullet_trails,
                blood
            );
        }  

        

        if is_key_released(KeyCode::N) {

            self.sound = SoundHandle::new("assets/sounds/brick_land.wav", [0., 0., 0.]);
            
            self.sound.play();
        }
        //self.update_hitbox_size(space, ctx);

        self.head.tick(space, ctx);
        self.body.tick(space, ctx);
        
    }

    pub fn place_teleporter(&mut self, ctx: &TickContext, teleporters: &mut Vec<Teleporter>, space: &mut Space) {

        if !is_key_released(KeyCode::B) {
            return
        }

        let pos = rapier_mouse_world_pos(ctx.camera_rect);

        let mut teleporter = Teleporter::new(pos, space, &ctx.uuid);

        teleporter.set_destination(self.teleporter_destination);

        // the next teleporter we place will link to this one
        self.teleporter_destination = Some(teleporter.get_rigid_body_handle());

        teleporters.push(teleporter);
        
    }

    pub fn delete_structure(&mut self, structures: &mut Vec<Structure>, space: &mut Space, ctx: &mut TickContext) {
        if !is_key_released(KeyCode::Backspace) {
            return
        }

        let mut structures_iter = SwapIter::new(structures);

        let then =web_time::Instant::now();
        while structures_iter.not_done() {
            let (_structures, mut structure) = structures_iter.next();

            if structure.contains_point(space, rapier_mouse_world_pos(ctx.camera_rect)) {
                structure.despawn(space);

                continue;
            }

            structures_iter.restore(structure);
        }
        
    }

    pub fn angle_weapon_to_mouse(&mut self, space: &mut Space, camera_rect: &Rect) {

        let shotgun_joint_handle = match self.weapon.as_ref().unwrap().player_joint_handle() {
            Some(shotgun_joint_handle) => shotgun_joint_handle,
            None => return,
        };

        let aim_angle_offset = self.weapon.as_ref().unwrap().aim_angle_offset();

        // lol
        let body_body = space.sync_rigid_body_set.get_sync_mut(self.body.body_handle).unwrap();

        let body_body_pos = Vec2::new(body_body.translation().x, body_body.translation().y);

        let weapon_pos = space.sync_rigid_body_set.get_sync(self.weapon.as_ref().unwrap().rigid_body()).unwrap().translation();

        let angle_to_mouse = get_angle_to_mouse(Vec2::new(weapon_pos.x, weapon_pos.y), camera_rect);

        let shotgun_joint = space.sync_impulse_joint_set.get_sync_mut(shotgun_joint_handle).unwrap();

        let shotgun_joint_data = shotgun_joint.data.as_revolute_mut().unwrap();

        // anchor the shotgun in a different position if its supposed to be on our right side
        let shotgun_anchor_pos = match self.facing {
            Facing::Right => vector![-30., 0.].into(),
            Facing::Left => vector![30., 0.].into(),
        };

        shotgun_joint_data.set_local_anchor2(shotgun_anchor_pos);

        let target_angle = match self.facing {
            Facing::Right => {
                (-angle_to_mouse + (PI / 2.)) + aim_angle_offset
            },
            Facing::Left => {
                ((angle_to_mouse + (PI / 2.)) * -1.) - aim_angle_offset
            },
        };


        if target_angle.abs() > 0.799 {
            // dont try to set the angle if we know its beyond the limit
            return;
        }

        shotgun_joint_data.set_motor_position(target_angle, 3000., 50.);

        return;
    }

    pub fn angle_head_to_mouse(&mut self, space: &mut Space, camera_rect: &Rect) {

        let head_joint_handle = match self.head_joint_handle {
            Some(head_joint_handle) => head_joint_handle,
            None => return,
        };

        let head_body = space.sync_rigid_body_set.get_sync_mut(self.head.body_handle).unwrap();

        let head_body_pos = Vec2::new(head_body.translation().x, head_body.translation().y);

        let angle_to_mouse = get_angle_to_mouse(head_body_pos, camera_rect);

        let head_joint = space.sync_impulse_joint_set.get_sync_mut(head_joint_handle).unwrap();

        let target_angle = match self.facing {
            Facing::Right => {
                -angle_to_mouse + (PI / 2.)
            },
            Facing::Left => {
                (angle_to_mouse + (PI / 2.)) * -1.
            },
        };


        if target_angle.abs() > 0.399 {
            // dont try to set the angle if we know its beyond the limit
            return;
        }

        head_joint.data.as_revolute_mut().unwrap().set_motor_position(target_angle, 300., 0.);

        return;

    }

    pub fn detach_head_if_dead(&mut self, space: &mut Space) {

        let head_joint_handle = match self.head_joint_handle {
            Some(head_joint_handle) => {
                head_joint_handle
            },
            None => {
                return;
            },
        };

        if self.health <= 0 {
            let head_joint = space.sync_impulse_joint_set.remove_sync(head_joint_handle, true);

            self.head_joint_handle = None;
        }
    }

    // pub fn update_arm_angle(&mut self, space: &mut Space, camera_rect: &Rect) {


    //     let body_pos = space.rigid_body_set.get(self.rigid_body).unwrap().translation().clone();
    //     let angle_to_mouse = get_angle_to_mouse(Vec2::new(body_pos.x, body_pos.y), camera_rect);

    //     // left arm
    //     let left_arm_body = space.rigid_body_set.get_mut(self.left_arm.get_rigid_body_handle()).unwrap();
    //     left_arm_body.set_rotation(UnitComplex::new(-angle_to_mouse + 1.7), true);


    //     // right arm
    //     let right_arm_body = space.rigid_body_set.get_mut(self.right_arm.get_rigid_body_handle()).unwrap();
    //     right_arm_body.set_rotation(UnitComplex::new(-angle_to_mouse + 1.7), true);

    //     // need to restrict arm angle depending on facing
    // }

    pub fn change_facing_direction(&mut self, space: &Space) {
        let velocity = space.sync_rigid_body_set.get_sync(*self.rigid_body_handle()).unwrap().linvel();


        if velocity.x > 100. {

            if !is_key_down(KeyCode::D) {
                return;
            }

            self.facing = Facing::Right;

            if let Some(weapon) = &mut self.weapon {
                weapon.set_facing(self.facing.clone());
            };

        }

        if velocity.x < -100. {

            if !is_key_down(KeyCode::A) {
                return;
            }

            self.facing = Facing::Left;

            if let Some(weapon) = &mut self.weapon {
                weapon.set_facing(self.facing.clone());
            };
        }
    }
    // pub fn update_idle_animation(&mut self, space: &mut Space) {

    //     let velocity = space.sync_rigid_body_set.get_sync(*self.rigid_body_handle()).unwrap().linvel();

    //     if velocity.x == 0. {

    //         self.idle_frame_progress += 5. * get_frame_time(); 
            
    //         //self.animation_handler.set_animation_state(PlayerAnimationState::Idle);

    //         if self.idle_frame_progress > 1. {
    //             self.animation_handler.next_frame(PlayerAnimationState::Idle);
    //             self.idle_frame_progress = 0.;
    //         }
            
    //     }

    // }
    /// Update arm anchor pos depending on facing direction
    // pub fn update_arm_anchor_pos(&mut self, space: &mut Space) {
    //     let arm_anchor_pos = match self.facing {
    //         Facing::Right => Point::<Real>::new(0. * self.sprite_scale, 2. * self.sprite_scale),
    //         Facing::Left => Point::<Real>::new(3. * self.sprite_scale, 7. * self.sprite_scale),
    //     };

    //     let left_arm_joint = space.impulse_joint_set.get_mut(
    //         self.left_arm.get_joint_handle()
    //     )
    //         .unwrap()
    //         .data
    //         .as_revolute_mut()
    //         .unwrap();

    //     left_arm_joint.set_local_anchor1(arm_anchor_pos);

    //     let right_arm_joint = space.impulse_joint_set.get_mut(
    //         self.right_arm.get_joint_handle()
    //     )
    //         .unwrap()
    //         .data
    //         .as_revolute_mut()
    //         .unwrap();

    //     right_arm_joint.set_local_anchor1(arm_anchor_pos);
        
    // }
    // pub fn update_walk_animation(&mut self, space: &mut Space) {
    //     let velocity = space.sync_rigid_body_set.get_sync(*self.rigid_body_handle()).unwrap().linvel();

    //     if velocity.x.abs() > 0. {
    //         self.walk_frame_progess += (velocity.x.abs() * get_frame_time()) / 20.;

    //         self.animation_handler.set_animation_state(PlayerAnimationState::GunRun);
    //     }

    //     if self.walk_frame_progess > 1. {

    //         // this all seems pretty stupid
    //         let animation_state = self.animation_handler.get_animation_state();

    //         match animation_state {
    //             PlayerAnimationState::Walking => {},
    //             PlayerAnimationState::GunRun => {},
    //             _ => return
    //         }

    //         self.animation_handler.next_frame(animation_state);

    //         self.walk_frame_progess = 0.;
    //     }
        
    // }
    pub fn own_nearby_structures(&mut self, space: &mut Space, structures: &mut Vec<Structure>, ctx: &mut TickContext, other_players: &mut SyncArena<Player>) {
        // take ownership of nearby structures to avoid network physics delay


        for structure in structures {

            let structure_body = space.sync_rigid_body_set.get_sync(structure.rigid_body_handle).unwrap();

            if structure_body.linvel().magnitude() > 1. {
                continue;
            }

            let structure_pos = structure_body.position().translation.vector;
           

            // need to calculate our distance seperately because other_players does not contain us
            let our_pos = space.sync_rigid_body_set.get_sync(*self.rigid_body_handle()).unwrap().translation();
            let our_distance_to_structure = structure_body.position().translation.vector - our_pos;

            //println!("our player distance to structure: {:?}", our_distance_to_structure);

            let mut closest_owner = self.owner.clone();
            let mut closest_distance = our_distance_to_structure.clone();

            for (player_index, player) in other_players.iter() {
                let player_pos = space.sync_rigid_body_set.get_sync(*player.rigid_body_handle()).unwrap().translation();

                let body_distance = structure_pos - player_pos;

                if body_distance.magnitude() < closest_distance.magnitude() {
                    closest_owner = player.owner.clone();
                    closest_distance = body_distance.clone();
                }

                

                //println!("player: {:?} distance to structure: {:?}", player_index, body_distance);
            }

            

            if structure.owner.clone().unwrap() != closest_owner {
                
                println!("updating owner!");

                structure.owner = Some(closest_owner);

            }

            

            

            // // if the other body is within 100 units, we take ownership of it
            // if body_distance.abs().magnitude() > 100. {
            //     continue;
            // }

            // match &mut structure.owner {
            //     Some(owner) => {

            //         // take ownership if we dont already own it
            //         if owner == ctx.uuid {
            //             continue;    
            //         }
                        
            //         *owner = ctx.uuid.clone();

            //     }
            //     None => {
            //         structure.owner = Some(ctx.uuid.clone())
            //     },
            // }

        }
    }
    pub fn fire_portal_gun(&mut self, _camera_rect: &Rect, _portal_bullets: &mut Vec<PortalBullet>) {
        if is_mouse_button_released(macroquad::input::MouseButton::Left) {
            //self.portal_gun.fire(camera_rect, portal_bullets);
        }
    }
    
    pub fn launch_brick(&mut self, bricks: &mut Vec<Brick>, space: &mut Space, ctx: &mut TickContext) {

        if !is_mouse_button_down(macroquad::input::MouseButton::Left) {
            return;
        }
        let (player_pos, player_rotation, player_velocity) = {
            let body = space.sync_rigid_body_set.get_sync(*self.rigid_body_handle()).unwrap();
            (body.position().clone(), body.rotation().clone(), body.linvel().clone())
        };

        let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

        let mouse_body_distance = mouse_pos - Vec2::new(player_pos.translation.x, player_pos.translation.y); 
        
        let brick_spawn_point = Vec2::new(
            player_pos.translation.x + mouse_body_distance.normalize().x * 40., 
            player_pos.translation.y + mouse_body_distance.normalize().y * 40.
        );

        let brick = Brick::new(space, brick_spawn_point, Some(ctx.uuid.clone()));
        
        let brick_body = space.sync_rigid_body_set.get_sync_mut(*brick.rigid_body_handle()).unwrap();

        let mut brick_velocity = player_velocity;
        brick_velocity.x += 5000. * mouse_body_distance.normalize().x;
        
        brick_body.set_rotation(player_rotation, true);
        brick_body.set_linvel(brick_velocity, true);

        bricks.push(brick);

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

        let rigid_body = space.sync_rigid_body_set.get_sync_mut(self.body.body_handle).unwrap();

        let gamepad: Option<Gamepad<'_>> = None;

        // sprinting but a little dumb
        // let speed = match is_key_down(KeyCode::LeftShift) {
        //     true => 100.,
        //     false => 20.,
        // };

        let speed = 50.;

        self.jump(rigid_body, gamepad);

        if is_key_down(KeyCode::A) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::DPadLeft)}) {

            if rigid_body.linvel().x < -self.max_speed.x {
                return
            }

            // cancel out x movement (but not all the way)
            if rigid_body.linvel().x.is_sign_positive() {
                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x * 0.5, rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x - speed, rigid_body.linvel().y],
                true
            );

        }

        if is_key_down(KeyCode::D) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::DPadRight)}) {

            if rigid_body.linvel().x > self.max_speed.x {
                return
            }

            // cancel out movement
            if rigid_body.linvel().x.is_sign_negative() {
                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x * 0.5, rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x + speed, rigid_body.linvel().y],
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

        self.body.draw(textures, space, flip_x).await;
        self.head.draw(textures, space, flip_x).await;
       
        if let Some(weapon) = &self.weapon {

            weapon.draw(space, textures, flip_x, false).await
        }

        let head_pos = {
            
            let rapier_pos = space.sync_rigid_body_set.get_sync(self.head.body_handle).unwrap().position().translation;

            rapier_to_macroquad(&Vec2::new(rapier_pos.x, rapier_pos.y))
            
        };

        draw_rectangle(head_pos.x - 25., head_pos.y - 40., (self.health as f32 / 100 as f32) as f32 * 50., 10., GREEN);
        
        
    }
}

impl HasPhysics for Player {
    fn collider_handle(&self) -> &SyncColliderHandle {
        &self.body.collider_handle
    }

    fn rigid_body_handle(&self) -> &SyncRigidBodyHandle {
        &self.body.body_handle
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