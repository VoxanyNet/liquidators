use std::{collections::HashSet, time::{Duration, Instant}};

use diff::Diff;
use gamelibrary::{get_angle_to_mouse, mouse_world_pos, rapier_mouse_world_pos, rapier_to_macroquad, sound::soundmanager::SoundHandle, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, sync_arena::{Index, SyncArena}, texture_loader::TextureLoader, time::Time, traits::{draw_texture_onto_physics_body, HasPhysics}};
use macroquad::{color::{RED, WHITE}, input::is_mouse_button_released, math::{vec2, Vec2}, miniquad::TextureParams, shapes::{draw_circle, draw_rectangle}, texture::{draw_texture_ex, DrawTextureParams}};
use nalgebra::{point, vector};
use parry2d::{query::Ray, shape::Shape};
use rapier2d::prelude::{ColliderHandle, QueryFilter, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{bullet_trail::BulletTrail, collider_from_texture_size, damage_number::{self, DamageNumber}, enemy::Enemy, player::player::{Facing, Player}, Grabbable, TickContext};


#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct MuzzleFlash {
    last_flash: Time,
    sprite: String,
    duration: Duration
}

impl MuzzleFlash {

    pub fn flash(&mut self) {
        self.last_flash = Time::now()
    }

    // creating a new muzzle flash does not flash it by default
    pub fn new(sprite_path: String, duration: Duration) -> Self {
        Self {
            last_flash: Time::new(0),
            sprite: sprite_path,
            duration,
        }
    }
    pub async fn draw(&self, position: Vec2, textures: &mut TextureLoader) {


        if self.last_flash.elapsed().num_milliseconds() > self.duration.as_millis() as i64 {
            return;
        }

        let texture = textures.get(&self.sprite).await;
        
        draw_texture_ex(texture, position.x, position.y, WHITE, DrawTextureParams::default());
        
    }

}

#[derive(Serialize, Deserialize)]
pub struct MuzzleFlashDiff {
    flash: bool,
    sprite: Option<String>,
    duration: Option<Duration>
}

impl Diff for MuzzleFlash {
    type Repr = MuzzleFlashDiff;

    fn diff(&self, other: &Self) -> Self::Repr {
        // this is an example where we are implementing the diff function a lot differently
        // instead of diffing the muzzle flash in a way you would expect FUCK YOU

        let mut diff = MuzzleFlashDiff {
            
            flash: false,
            sprite: None,
            duration: None,
        };
        
        if other.last_flash != self.last_flash {
            diff.flash = true;
        };

        if other.sprite != self.sprite {
            diff.sprite = Some(other.sprite.clone());
        }

        if other.duration != self.duration {
            diff.duration = Some(other.duration);
        }

        diff
    }

    fn apply(&mut self, diff: &Self::Repr) {

        if diff.flash {

            self.last_flash = Time::now();
        }

        if let Some(duration) = diff.duration {
            self.duration = duration;
        }

        if let Some(sprite) = &diff.sprite {
            self.sprite = sprite.clone();
        }
    }

    fn identity() -> Self {
        Self { 
            last_flash: Time::new(0),
            sprite: String::identity(),
            duration: Duration::ZERO,
        }
    }
}

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Shotgun {
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
    pub muzzle_flash: MuzzleFlash
}

impl Grabbable for Shotgun {
    fn grabbing(&mut self) -> &mut bool {
        &mut self.grabbing
    }
}

impl Shotgun {

    pub fn new(space: &mut Space, pos: Vec2, owner: String, player_rigid_body_handle: Option<SyncRigidBodyHandle>, textures: &mut TextureLoader) -> Self {

        let sprite_path = "assets/shotgun.png".to_string();

        let texture_size = textures.get_blocking(&sprite_path).size() 
            * 2.; // scale the size of the shotgun

        dbg!(texture_size);
        
        let rigid_body = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .ccd_enabled(true)
                .position(vector![pos.x, pos.y].into())
                .build()
        );

        dbg!(collider_from_texture_size(texture_size).build().shape().as_cuboid().unwrap().half_extents);

        let collider = space.sync_collider_set.insert_with_parent_sync(
            collider_from_texture_size(texture_size)
                .mass(1.)
                .build(), 
            rigid_body, 
            &mut space.sync_rigid_body_set
        );

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
            facing: Facing::Left,
            muzzle_flash: MuzzleFlash::new("assets/particles/shotgun_muzzle_flash.png".to_string(), Duration::from_millis(50))
        }
    }

    pub fn spawn(space: &mut Space, pos: Vec2, shotguns: &mut Vec<Shotgun>, owner: String, player_rigid_body_handle: Option<SyncRigidBodyHandle>, textures: &mut TextureLoader) {

        shotguns.push(
            Self::new(space, pos, owner, player_rigid_body_handle, textures)
        );
    }

    pub fn owner_tick(
        &mut self, 
        players: &mut SyncArena<Player>, 
        hit_markers: &mut Vec<Vec2>, 
        space: &mut Space, 
        ctx: &mut TickContext,
        enemies: &mut SyncArena<Enemy>,
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>
    ) {
        ctx.owned_rigid_bodies.push(self.rigid_body);
        ctx.owned_colliders.push(self.collider);

        self.fire(space, players, enemies, hit_markers, damage_numbers, bullet_trails, ctx);
        
    }

    pub fn all_tick(&mut self, players: &mut SyncArena<Player>, space: &mut Space, ctx: &mut TickContext) {
        self.grab(players, space, ctx);

        //self.sync_sound(ctx);
    }

    pub fn tick(
        &mut self, 
        players: &mut SyncArena<Player>, 
        space: &mut Space, 
        hit_markers: &mut Vec<Vec2>, 
        ctx: &mut TickContext,
        enemies: &mut SyncArena<Enemy>,
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>
    ) {

        if *ctx.uuid == self.owner {
            self.owner_tick(players, hit_markers, space, ctx, enemies, damage_numbers, bullet_trails);
        }

        self.all_tick(players, space, ctx);
        
        

        
    }   

    pub fn sync_sound(&mut self, ctx: &mut TickContext) {
        for sound_handle in &mut self.sounds {
            ctx.sounds.sync_sound(sound_handle);
        }
    }

    pub fn fire(
        &mut self, 
        space: &mut Space, 
        players: &mut SyncArena<Player>,
        enemies: &mut SyncArena<Enemy>, 
        hit_markers: &mut Vec<Vec2>, 
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>,
        ctx: &mut TickContext
    ) {
        
        if !is_mouse_button_released(macroquad::input::MouseButton::Left) {
            return;
        }

        self.muzzle_flash.flash();


        ctx.screen_shake.x_frequency += 20.;
        ctx.screen_shake.x_intensity += 10.;

        ctx.screen_shake.x_frequency_decay = 10.;
        ctx.screen_shake.x_intensity_decay = 20.;
        

        let shotgun_pos = space.sync_rigid_body_set.get_sync(self.rigid_body).unwrap().position().translation.clone();

        let mut fire_sound = SoundHandle::new("assets/sounds/shotgun/fire.wav", [0., 0., 0.]);

        fire_sound.play();

        self.sounds.push(fire_sound);


        let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

        let mouse_vector = Vec2::new(
            mouse_pos.x - shotgun_pos.x,
            mouse_pos.y - shotgun_pos.y 
        ).normalize();

        // spawn bullet trail
        {
            let shotgun_collider = space.sync_collider_set.get_sync(self.collider).unwrap();

            let vertex_index = match self.facing {
                Facing::Right => 3,
                Facing::Left => 1,
            };

            //let vertex_index = match facing
            let bullet_trail_start = shotgun_collider.shape().as_cuboid().unwrap().aabb(shotgun_collider.position()).vertices()[vertex_index];
            
            let fortnite = rapier_to_macroquad(&Vec2::new(bullet_trail_start.x, bullet_trail_start.y));

            let mouse_pos = mouse_world_pos(&ctx.camera_rect);
            

            bullet_trails.insert(
                BulletTrail::new(
                    Vec2::new(fortnite.x, fortnite.y), 
                    mouse_pos
                )
            );
        }

        // knock the player back
        if let Some(player_rigid_body_handle) = self.player_rigid_body_handle {
            let player_rigid_body = space.sync_rigid_body_set.get_sync_mut(player_rigid_body_handle).unwrap();
            
            let new_player_rigid_body_velocity = {
                let mut new_player_rigid_body_velocity = player_rigid_body.linvel().clone();

                new_player_rigid_body_velocity.x -= mouse_vector.x * 500.;
                new_player_rigid_body_velocity.y -= mouse_vector.y * 500.; 

                new_player_rigid_body_velocity
            }; 

            

            player_rigid_body.set_linvel(
                new_player_rigid_body_velocity, 
                true
            );
        }

        let ray = Ray::new(point![shotgun_pos.x, shotgun_pos.y], vector![mouse_vector.x, mouse_vector.y]);
        let max_toi = 1000.0;
        let solid = true;
        let filter = QueryFilter::default();

        let mut hit_rigid_bodies: Vec<RigidBodyHandle> = Vec::new();

        let shotgun_position = space.sync_rigid_body_set.get_sync(self.rigid_body).unwrap().translation().clone();

        let local_collider = space.sync_collider_set.get_local_handle(self.collider);

        let mut intersections: Vec<ColliderHandle> = Vec::new();
        
        // get a vector with all the intersections
        space.query_pipeline.intersections_with_ray(&space.sync_rigid_body_set.rigid_body_set, &space.sync_collider_set.collider_set, &ray, max_toi, solid, filter, 
        |handle, _intersection| {

            // dont want to intersect with shotgun
            if local_collider == handle {
                return true;
            };

            intersections.push(handle);

            true

        });

        for handle in intersections {
            let collider = space.sync_collider_set.get_local(handle).unwrap();

            let distance = collider.translation() - shotgun_position;
            
            hit_rigid_bodies.push(collider.parent().unwrap());

            let fall_off_multiplier = (-0.01 * distance.norm()).exp();

            // ENEMIES
            for (_, enemy) in &mut *enemies {

                if enemy.health <= 0 {
                    continue;
                }

                // dont want to hit the enemy twice

                let mut enemy_hit = false;

                let enemy_position = space.sync_rigid_body_set.get_sync(enemy.head.body_handle).unwrap().translation();

                let enemy_position_macroquad = rapier_to_macroquad(&vec2(enemy_position.x, enemy_position.y));

                let damage_number_position = Vec2 {
                    x: enemy_position_macroquad.x,
                    y: enemy_position_macroquad.y - 120.,
                };

                if space.sync_collider_set.get_local_handle(enemy.body.collider_handle) == handle {

                    let damage = (50.0 * fall_off_multiplier).round() as i32;

                    enemy.health -= damage;

                    let damage_number = DamageNumber::new(space, damage, damage_number_position, Some(40), Some(RED));
                    
                    // temporary workaround to make damage number move with player when shot
                    hit_rigid_bodies.push(space.sync_rigid_body_set.get_local_handle(damage_number.rigid_body_handle));

                    damage_numbers.insert(
                        damage_number
                    );

                    enemy_hit = true;

                }


                if space.sync_collider_set.get_local_handle(enemy.head.collider_handle) == handle && enemy_hit == false {

                    
                    let damage = (100.0 * fall_off_multiplier).round() as i32;

                    enemy.health -= damage;

                    let damage_number = DamageNumber::new(space, damage, damage_number_position, Some(45), Some(RED));
                    
                    // temporary workaround to make damage number move with player when shot
                    hit_rigid_bodies.push(space.sync_rigid_body_set.get_local_handle(damage_number.rigid_body_handle));

                    damage_numbers.insert(
                        damage_number
                    );
                }
            }

            // PLAYERS
            for (_, player) in &mut *players {

                let local_player_head_collider_handle = space.sync_collider_set.get_local_handle(player.head.collider_handle);

                if local_player_head_collider_handle == handle {

            
                    if player.health <= 10 {
                        player.health = 0;

                        continue;
                    }

                    player.health -= 10;

                    continue;
                }

                if space.sync_collider_set.get_local_handle(player.body.collider_handle) == handle {


                    if player.health <= 5 {
                        player.health = 0;

                        continue;
                    }
                    
                    player.health -= 5;

                    
                }
            }

        }

        for rigid_body_handle in hit_rigid_bodies {

            
            // own this rigid body for this frame so we can update its velocity
            ctx.owned_rigid_bodies.push(space.sync_rigid_body_set.get_sync_handle(rigid_body_handle));

            let rigid_body = space.sync_rigid_body_set.get_local_mut(rigid_body_handle).unwrap();

            let mut new_velocity = rigid_body.linvel().clone();

            
            new_velocity.x += mouse_vector.x * 500.;
            new_velocity.y += mouse_vector.y * 500.;
        
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

    }
}

impl HasPhysics for Shotgun {
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