use std::{collections::HashSet, time::{Duration, Instant}};

use diff::Diff;
use gamelibrary::{get_angle_to_mouse, mouse_world_pos, rapier_mouse_world_pos, rapier_to_macroquad, sound::soundmanager::SoundHandle, space::{Space, SyncColliderHandle, SyncImpulseJointHandle, SyncRigidBodyHandle}, sync_arena::{Index, SyncArena}, texture_loader::TextureLoader, time::Time, traits::{draw_texture_onto_physics_body, HasPhysics}};
use macroquad::{color::{RED, WHITE}, input::{is_key_released, is_mouse_button_released}, math::{vec2, Vec2}, miniquad::TextureParams, shapes::{draw_circle, draw_rectangle}, text::{draw_text_ex, TextParams}, texture::{draw_texture, draw_texture_ex, DrawTextureParams}, window::screen_height};
use nalgebra::{point, vector, Const, OPoint};
use parry2d::{math::{Translation, Vector}, query::Ray, shape::Shape};
use rapier2d::prelude::{ColliderHandle, InteractionGroups, QueryFilter, RevoluteJointBuilder, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};
use gamelibrary::sound::soundmanager::SoundManager;

use crate::{blood::Blood, bullet_casing::BulletCasing, bullet_trail::BulletTrail, collider_from_texture_size, damage_number::{self, DamageNumber}, enemy::Enemy, muzzle_flash::MuzzleFlash, player::{self, player::{Facing, Player, PlayerWeapon, WeaponTickParameters}}, structure::Structure, Grabbable, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct WeaponFireEvent {
    intersecting_colliders: Vec<SyncColliderHandle>,
    origin: Vec2,
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Weapon {
    pub player_rigid_body_handle: Option<SyncRigidBodyHandle>,
    pub collider: SyncColliderHandle,
    pub rigid_body: SyncRigidBodyHandle,
    pub sprite: String,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub grabbing: bool,
    pub owner: String,
    pub picked_up: bool,
    pub sounds: Vec<SoundHandle>, // is this the best way to do this?
    pub facing: Facing,
    pub muzzle_flash: MuzzleFlash,
    pub scale: f32,
    pub aim_angle_offset: f32,
    pub fire_sound_path: String,
    pub x_screen_shake_frequency: f64,
    pub x_screen_shake_intensity: f64,
    pub y_screen_shake_frequency: f64,
    pub y_screen_shake_intensity: f64,
    pub shell_sprite: Option<String>, // should this be generic i dont knowwwww
    pub bullet_casings: HashSet<BulletCasing>,
    pub player_joint_handle: Option<SyncImpulseJointHandle>,
    #[serde(default)]
    last_reload: Time,
    #[serde(default)]
    rounds: u32,
    #[serde(default)]
    capacity: u32,
    #[serde(default)]
    reserve_capacity: u32,
    #[serde(default)]
    reload_duration: u32, // reload duration in millis
}

impl Grabbable for Weapon {
    fn grabbing(&mut self) -> &mut bool {
        &mut self.grabbing
    }
}

impl Weapon {

    pub fn new(
        space: &mut Space, 
        pos: Vec2, 
        owner: String, 
        player_rigid_body_handle: Option<SyncRigidBodyHandle>, 
        textures: &mut TextureLoader,
        sprite_path: String,
        scale: f32,
        aim_angle_offset: Option<f32>,
        mass: Option<f32>,
        fire_sound_path: &str,
        x_screen_shake_frequency: f64,
        x_screen_shake_intensity: f64,
        y_screen_shake_frequency: f64,
        y_screen_shake_intensity: f64,
        shell_sprite_path: Option<String>,
        texture_size: Vec2,
        facing: Facing,
        reload_duration: u32,
        rounds: u32,
        capacity: u32,
        reserve_capacity: u32
        

    ) -> Self {

        let mass = mass.unwrap_or(1.);

        let texture_size = texture_size * scale ; // scale the size of the shotgun
        
        let rigid_body = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .ccd_enabled(true)
                .position(vector![pos.x, pos.y].into())
                .build()
        );

    

        let collider = space.sync_collider_set.insert_with_parent_sync(
            collider_from_texture_size(texture_size)
                .mass(mass)
                .build(), 
            rigid_body, 
            &mut space.sync_rigid_body_set
        );

        let aim_angle_offset = match aim_angle_offset {
            Some(aim_angle_offset) => aim_angle_offset,
            None => 0.,
        };

        // if we are attaching the weapon to the player we need to do some epic stuff!
        let player_joint_handle: Option<SyncImpulseJointHandle> = if let Some(player_rigid_body_handle) = player_rigid_body_handle {

            // make the shotgun not collide with anything
            space.sync_collider_set.get_sync_mut(collider).unwrap().set_collision_groups(InteractionGroups::none());

            let local_player_rigid_body_handle = space.sync_rigid_body_set.get_local_handle(player_rigid_body_handle);
            let local_weapon_rigid_body_handle = space.sync_rigid_body_set.get_local_handle(rigid_body);

            // joint the shotgun to the player
            Some(space.sync_impulse_joint_set.insert_sync(
                local_player_rigid_body_handle,
                local_weapon_rigid_body_handle,
                RevoluteJointBuilder::new()
                    .local_anchor1(vector![0., 0.].into())
                    .local_anchor2(vector![30., 0.].into())
                    .limits([-0.8, 0.8])
                    .contacts_enabled(false)
                .build(),
                true
            ))
            

        } 

        else {
            None
        };

        Self {
            player_rigid_body_handle,
            picked_up: false,
            collider,
            rigid_body,
            sprite: sprite_path,
            selected: false,
            dragging: false,
            drag_offset: None,
            grabbing: false,
            owner,
            sounds: vec![],
            facing,
            muzzle_flash: MuzzleFlash::new("assets/particles/shotgun_muzzle_flash.png".to_string(), web_time::Duration::from_millis(50)),
            scale,
            aim_angle_offset,
            fire_sound_path: fire_sound_path.to_string(),
            x_screen_shake_frequency,
            x_screen_shake_intensity,
            y_screen_shake_frequency,
            y_screen_shake_intensity,
            shell_sprite: shell_sprite_path,
            bullet_casings: HashSet::new(),
            player_joint_handle: player_joint_handle,
            last_reload: Time::new(0),
            rounds,
            capacity,
            reserve_capacity,
            reload_duration,
            
            
            
        }
    }

    pub fn reload(&mut self, ctx: &mut TickContext) {
        // dont reload while already reloading
        if self.last_reload.elapsed().num_milliseconds() < self.reload_duration.into() {
            
            return;
        }

        // dont reload if we already have a full mag
        if self.rounds == self.capacity {
            return;
        }

        let rounds_needed_to_fill = self.capacity - self.rounds;

        // dont use rounds than are available in reserve
        let actual_rounds_available = rounds_needed_to_fill.min(self.reserve_capacity);

        ctx.console.log(format!("actual rounds available: {:?}", actual_rounds_available));

        if actual_rounds_available == 0 {
            // play a sound here to indicate that we cant reload
    
            return;
        }

        self.reserve_capacity -= actual_rounds_available;

        self.rounds += actual_rounds_available;

        self.last_reload = Time::now();
    }

    pub fn owner_tick(
        &mut self, 
        hit_markers: &mut Vec<Vec2>, 
        space: &mut Space, 
        ctx: &mut TickContext,
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>,
        blood: &mut HashSet<Blood>, 
        weapon_tick_parameter: &mut WeaponTickParameters,
    ) {
        ctx.owned_rigid_bodies.push(self.rigid_body);
        ctx.owned_colliders.push(self.collider);

        if is_key_released(macroquad::input::KeyCode::R) {
            self.reload(ctx);
        }
        
    }

    pub fn all_tick(&mut self, space: &mut Space, ctx: &mut TickContext, weapon_tick_parameters: &mut WeaponTickParameters) {
        self.grab(weapon_tick_parameters.players, space, ctx);
    }

    pub fn tick(
        &mut self, 
        space: &mut Space, 
        hit_markers: &mut Vec<Vec2>, 
        ctx: &mut TickContext,
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>,
        blood: &mut HashSet<Blood>,
        weapon_tick_parameters: &mut WeaponTickParameters,
    ) {

        if *ctx.uuid == self.owner {
            self.owner_tick(hit_markers, space, ctx, damage_numbers, bullet_trails, blood, weapon_tick_parameters);
        }

        self.all_tick(space, ctx, weapon_tick_parameters);
        
        

        
    }   

    pub async fn sync_sound(&mut self, ctx: &mut TickContext<'_>) {
        for sound_handle in &mut self.sounds {
            ctx.sounds.sync_sound(sound_handle).await;
        }
    }

    pub fn get_weapon_tip(&self, space: &Space) -> OPoint<f32, Const<2>>{
        let shotgun_collider = space.sync_collider_set.get_sync(self.collider).unwrap();

        // get the tip of the gun
        let vertex_index = match self.facing {
            Facing::Right => 3,
            Facing::Left => 1,
        };

        //let vertex_index = match facing
        let shotgun_tip = shotgun_collider.shape().as_cuboid().unwrap().aabb(shotgun_collider.position()).vertices()[vertex_index];
        
        shotgun_tip
    }

    pub fn get_weapon_rear(&self, space: &Space) -> OPoint<f32, Const<2>> {
        let shotgun_collider = space.sync_collider_set.get_sync(self.collider).unwrap();

        let vertex_index = match self.facing {
            Facing::Right => 1,
            Facing::Left => 3,
        };

        let shotgun_rear = shotgun_collider.shape().as_cuboid().unwrap().aabb(shotgun_collider.position()).vertices()[vertex_index];
        
        shotgun_rear
    }

    pub fn shake_screen(&self, ctx: &mut TickContext) {
        ctx.screen_shake.x_frequency = self.x_screen_shake_frequency;
        ctx.screen_shake.x_intensity = self.x_screen_shake_intensity;

        ctx.screen_shake.x_frequency_decay = 10.;
        ctx.screen_shake.x_intensity_decay = 20.;
    }

    pub fn play_sound(&mut self) {
        let mut fire_sound = SoundHandle::new(&self.fire_sound_path, [0., 0., 0.]);

        fire_sound.play();

        self.sounds.push(fire_sound);
    }

    pub fn knockback_player(&mut self, space: &mut Space, bullet_vector: Vec2) {
        // knock the player back
        if let Some(player_rigid_body_handle) = self.player_rigid_body_handle {
            let player_rigid_body = space.sync_rigid_body_set.get_sync_mut(player_rigid_body_handle).unwrap();
            
            let new_player_rigid_body_velocity = {
                let mut new_player_rigid_body_velocity = player_rigid_body.linvel().clone();

                new_player_rigid_body_velocity.x -= bullet_vector.x * 500.;
                new_player_rigid_body_velocity.y -= bullet_vector.y * 500.; 

                new_player_rigid_body_velocity
            }; 

            

            player_rigid_body.set_linvel(
                new_player_rigid_body_velocity, 
                true
            );
        }
    }

    pub fn fire(
        &mut self, 
        space: &mut Space, 
        hit_markers: &mut Vec<Vec2>, 
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>,
        blood: &mut HashSet<Blood>,
        ctx: &mut TickContext,
        weapon_tick_parameters: &mut WeaponTickParameters
        
    ) {

        
        // dont shoot while reloading
        if self.last_reload.elapsed().num_milliseconds() < self.reload_duration.into() {

            let mut sound = SoundHandle::new("assets/sounds/pistol_dry_fire.wav", [0., 0., 0.]);
            sound.play();

            self.sounds.push(sound);

            
            return;
        }

        
        self.rounds -= 1;

        // automatically reload if zero bullets
        if self.rounds == 0 {
            self.reload(ctx);

            // also play dry fire sound if no bullets
            let mut sound = SoundHandle::new("assets/sounds/pistol_dry_fire.wav", [0., 0., 0.]);
            sound.play();

            self.sounds.push(sound);

            return;
        }

        self.muzzle_flash.flash();
        
        self.shake_screen(ctx);

        self.play_sound();

        let shotgun_body = space.sync_rigid_body_set.get_sync(self.rigid_body).unwrap().clone();

        let weapon_angle = shotgun_body.rotation().angle();

        let shotgun_pos = shotgun_body.position().translation;

        let shotgun_velocity = shotgun_body.linvel();

        // we use the angle of the gun to get the direction of the bullet
        let mut macroquad_angle_bullet_vector = Vec2 {
            x:  weapon_angle.cos(),
            y: weapon_angle.sin() * -1.,
        };
        
        match self.facing {
            Facing::Right => {},
            Facing::Left => {
                macroquad_angle_bullet_vector.x *= -1.;
                macroquad_angle_bullet_vector.y *= -1.;
            }
        }

        let rapier_angle_bullet_vector = Vec2 {
            x: macroquad_angle_bullet_vector.x,
            y: macroquad_angle_bullet_vector.y * -1.
        };


        self.knockback_player(space, rapier_angle_bullet_vector);

        bullet_trails.insert(
            BulletTrail::new(
                Vec2::new(
                    shotgun_pos.x, 
                    shotgun_pos.y + 10.
                ), 
                Vec2 {
                    x: shotgun_pos.x + (macroquad_angle_bullet_vector.x * 10000.),
                    y: shotgun_pos.y - (macroquad_angle_bullet_vector.y * 10000.),
                },
                None,
                self.owner.clone()
            )
        );
        
        match &self.shell_sprite {
            Some(shell_sprite) => {
                self.bullet_casings.insert(
                    BulletCasing::new(
                        Vec2::new(shotgun_pos.x, shotgun_pos.y), 
                        Vec2::new(5., 5.),
                        shell_sprite.clone(), 
                        space,
                        *shotgun_velocity
                    )
                );
            },
            None => {},
        }
        
        // from here on out needs to be cleaned up and put into seperate functions
        let ray = Ray::new(point![shotgun_pos.x, shotgun_pos.y], vector![rapier_angle_bullet_vector.x, rapier_angle_bullet_vector.y]);
        let max_toi = 5000.0;
        let solid = true;
        let filter = QueryFilter::default();

        

        let weapon_position = space.sync_rigid_body_set.get_sync(self.rigid_body).unwrap().translation().clone();

        let local_collider = space.sync_collider_set.get_local_handle(self.collider);

        let mut hit_rigid_bodies: Vec<RigidBodyHandle> = Vec::new();
        let mut intersections: Vec<ColliderHandle> = Vec::new();
        let mut sync_intersections: Vec<SyncColliderHandle> = Vec::new();
        
        // get a vector with all the intersections
        space.query_pipeline.intersections_with_ray(&space.sync_rigid_body_set.rigid_body_set, &space.sync_collider_set.collider_set, &ray, max_toi, solid, filter, 
        |handle, _intersection| {

            // dont want to intersect with the weapon itself
            if local_collider == handle {
                return true;
            };

            let sync_handle = space.sync_collider_set.get_sync_handle(handle);

            sync_intersections.push(sync_handle);

            intersections.push(handle);

            true

        });

        // everything from here on needs to be cleaned up we are iterating way too many times
        // we probably need to invert these loops

        
        // STRUCTURES
        for (_, structure) in &mut *weapon_tick_parameters.structures {

            if sync_intersections.contains(&structure.collider_handle) {

                let bullet_impact_data = BulletImpactData{ 
                    shooter_pos: shotgun_pos, 
                    impacted_collider: structure.collider_handle.clone() 
                };

                structure.handle_bullet(&bullet_impact_data, space);
                
                continue 
                
            }
                
        }

        // ENEMIES
        for (_, enemy) in &mut *weapon_tick_parameters.enemies {

            if sync_intersections.contains(&enemy.body.collider_handle) {
                let bullet_impact_data = BulletImpactData{ 
                    shooter_pos: shotgun_pos, 
                    impacted_collider: enemy.body.collider_handle.clone() 
                };

                enemy.handle_bullet_impact(space, bullet_impact_data);
            }
            
            else if sync_intersections.contains(&enemy.head.collider_handle)  {

                let bullet_impact_data = BulletImpactData{ 
                    shooter_pos: shotgun_pos, 
                    impacted_collider: enemy.head.collider_handle.clone() 
                };

                enemy.handle_bullet_impact(space, bullet_impact_data);
            }

        }
        
        // PLAYERS
        for (_, player) in &mut *weapon_tick_parameters.players {

            if sync_intersections.contains(&player.body.collider_handle) {
                let bullet_impact_data = BulletImpactData{ 
                    shooter_pos: shotgun_pos, 
                    impacted_collider: player.collider_handle().clone() 
                };

                player.handle_bullet_impact(space, bullet_impact_data);

            }

            if sync_intersections.contains(&player.head.collider_handle) {
                let bullet_impact_data = BulletImpactData{ 
                    shooter_pos: shotgun_pos, 
                    impacted_collider: player.collider_handle().clone() 
                };

                player.handle_bullet_impact(space, bullet_impact_data);

            }
            
        }

        for handle in intersections {
            let collider = space.sync_collider_set.get_local(handle).unwrap();

            let distance = collider.translation() - weapon_position;
            
            hit_rigid_bodies.push(collider.parent().unwrap());

        }
        

        // apply knockback to any rigid body hit
        self.knockback_generic_rigid_bodies(&mut hit_rigid_bodies, space, ctx, rapier_angle_bullet_vector);



    }

    pub fn knockback_generic_rigid_bodies(
        &self, 
        hit_rigid_bodies: &mut Vec<RigidBodyHandle>, 
        space: &mut Space, 
        ctx: &mut TickContext,
        bullet_vector: Vec2
    ) {
        for rigid_body_handle in hit_rigid_bodies {

            
            // own this rigid body for this frame so we can update its velocity
            ctx.owned_rigid_bodies.push(space.sync_rigid_body_set.get_sync_handle(*rigid_body_handle));

            let rigid_body = space.sync_rigid_body_set.get_local_mut(*rigid_body_handle).unwrap();

            let mut new_velocity = rigid_body.linvel().clone();

            
            new_velocity.x += bullet_vector.x * 500.;
            new_velocity.y += bullet_vector.y * 500.;
        
            rigid_body.set_linvel(
                new_velocity, 
                true
            );
        }
    }

    pub fn grab(&mut self, players: &mut SyncArena<Player>, space: &mut Space, ctx: &mut TickContext) {

        if self.picked_up {
            return;
        }

        // we need to have a more efficient way of finding the currently controlled player
        for (index, player) in players {
            if player.owner == *ctx.uuid {

                let reference_body = player.body.body_handle;

                self.update_grabbing(space, ctx.camera_rect, Vec2::new(250., 250.), reference_body);

                break;

            }
        }

        self.update_grab_velocity(space);

    }

    pub async fn draw_hud(&self, ctx: &mut TickContext<'_>) {

        let texture = ctx.textures.get(&self.sprite).await;

        let font = ctx.font_loader.get("assets/fonts/CutePixel.ttf").await;

        let mut draw_params = DrawTextureParams::default();

        draw_params.dest_size = Some(Vec2::new(texture.width() * 2., texture.height() * 2.));

        let sprite_draw_pos = Vec2::new(
            10., 
            (
                screen_height() - texture.height() * 2.
            ) - 10.
        );

        draw_texture_ex(texture, sprite_draw_pos.x, sprite_draw_pos.y, WHITE, draw_params);

        let mut text_params = TextParams::default();
        text_params.font = Some(font);
        text_params.font_size = 28;

        draw_text_ex(format!("{}/{}", self.rounds, self.capacity), sprite_draw_pos.x, sprite_draw_pos.y - 14., text_params);
    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, flip_x: bool, flip_y: bool) {

        draw_texture_onto_physics_body(
            self.rigid_body, 
            self.collider, 
            space, 
            &self.sprite, 
            textures, 
            flip_x, 
            flip_y, 
            0.
        ).await;

        let shotgun_collider = space.sync_collider_set.get_sync(self.collider).unwrap();

        let vertex_index = match self.facing {
            Facing::Right => 3,
            Facing::Left => 1,
        };

        let shotgun_end_point = shotgun_collider.shape().as_cuboid().unwrap().aabb(shotgun_collider.position()).vertices()[vertex_index];
            
        let shotgun_end_point_macroquad = rapier_to_macroquad(&Vec2::new(shotgun_end_point.x, shotgun_end_point.y));

        self.muzzle_flash.draw(shotgun_end_point_macroquad, textures).await;

        for casing in &self.bullet_casings {
            casing.draw(space, textures).await
        }

    }
}

impl HasPhysics for Weapon {
    fn collider_handle(&self) -> &SyncColliderHandle {
        &self.collider
    }

    fn rigid_body_handle(&self) -> &SyncRigidBodyHandle {
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

    fn drag_offset(&mut self) -> &mut Option<macroquad::prelude::Vec2> {
        &mut self.drag_offset
    }
}

#[derive(Clone)]
pub struct BulletImpactData {
    pub shooter_pos: Translation<f32>,
    pub impacted_collider: SyncColliderHandle
}