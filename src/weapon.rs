use std::{collections::HashSet, time::{Duration, Instant}};

use diff::Diff;
use gamelibrary::{get_angle_to_mouse, mouse_world_pos, rapier_mouse_world_pos, rapier_to_macroquad, sound::soundmanager::SoundHandle, space::{Space, SyncColliderHandle, SyncImpulseJointHandle, SyncRigidBodyHandle}, sync_arena::{Index, SyncArena}, texture_loader::TextureLoader, time::Time, traits::{draw_texture_onto_physics_body, HasPhysics}};
use macroquad::{color::{RED, WHITE}, input::is_mouse_button_released, math::{vec2, Vec2}, miniquad::TextureParams, shapes::{draw_circle, draw_rectangle}, texture::{draw_texture_ex, DrawTextureParams}};
use nalgebra::{point, vector, Const, OPoint};
use parry2d::{query::Ray, shape::Shape};
use rapier2d::prelude::{ColliderHandle, InteractionGroups, QueryFilter, RevoluteJointBuilder, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};
use gamelibrary::sound::soundmanager::SoundManager;

use crate::{bullet_casing::BulletCasing, bullet_trail::BulletTrail, collider_from_texture_size, damage_number::{self, DamageNumber}, enemy::Enemy, muzzle_flash::MuzzleFlash, player::{self, player::{Facing, Player}}, Grabbable, TickContext};


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
    pub player_joint_handle: Option<SyncImpulseJointHandle>
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
        facing: Facing

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
            
        }
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

        self.sync_sound(ctx);
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
        
        self.shake_screen(ctx);

        self.play_sound();

        let shotgun_angle = space.sync_rigid_body_set.get_sync(self.rigid_body).unwrap().rotation().angle().clone();

        let shotgun_pos = space.sync_rigid_body_set.get_sync(self.rigid_body).unwrap().position().translation.clone();

        let shotgun_pos_macroquad = rapier_to_macroquad(&vec2(shotgun_pos.x , shotgun_pos.y));
        
        let rapier_mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

        let macroquad_mouse_pos = mouse_world_pos(&ctx.camera_rect);

        // the direction of the bullet
        let bullet_vector = Vec2::new(
            rapier_mouse_pos.x - shotgun_pos.x,
            rapier_mouse_pos.y - shotgun_pos.y 
        ).normalize();

        let mut macroquad_angle_bullet_vector = Vec2 {
            x:  shotgun_angle.cos(),
            y: shotgun_angle.sin() * -1.,
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


        println!("bullet vector: {:?}", macroquad_angle_bullet_vector);

        self.knockback_player(space, bullet_vector);

        let shotgun_tip = self.get_weapon_tip(space);
        let shotgun_tip_macroquad = rapier_to_macroquad(&vec2(shotgun_tip.x, shotgun_tip.y));

        bullet_trails.insert(
            BulletTrail::new(
                Vec2::new(
                    shotgun_pos_macroquad.x, 
                    shotgun_pos_macroquad.y - 10.
                ), 
                Vec2 {
                    x: shotgun_pos_macroquad.x + (macroquad_angle_bullet_vector.x * 10000.),
                    y: shotgun_pos_macroquad.y + (macroquad_angle_bullet_vector.y * 10000.),
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
                        space   
                    )
                );
            },
            None => {},
        }
        
        let ray = Ray::new(point![shotgun_pos.x, shotgun_pos.y], vector![rapier_angle_bullet_vector.x, rapier_angle_bullet_vector.y]);
        let max_toi = 5000.0;
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

            let rapier_pos = rapier_to_macroquad(&vec2(collider.translation().x, collider.translation().y));

            //hit_markers.push(rapier_pos);

            let distance = collider.translation() - shotgun_position;
            
            hit_rigid_bodies.push(collider.parent().unwrap());

            let fall_off_multiplier = (-0.01 * distance.norm()).exp();

            // ENEMIES
            for (_, enemy) in &mut *enemies {

                if enemy.health <= 0 {
                    continue;
                }

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

        self.knockback_intersections(hit_rigid_bodies, space, ctx, bullet_vector);



    }

    pub fn knockback_intersections(
        &self, 
        hit_rigid_bodies: Vec<RigidBodyHandle>, 
        space: &mut Space, 
        ctx: &mut TickContext,
        bullet_vector: Vec2
    ) {
        for rigid_body_handle in hit_rigid_bodies {

            
            // own this rigid body for this frame so we can update its velocity
            ctx.owned_rigid_bodies.push(space.sync_rigid_body_set.get_sync_handle(rigid_body_handle));

            let rigid_body = space.sync_rigid_body_set.get_local_mut(rigid_body_handle).unwrap();

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