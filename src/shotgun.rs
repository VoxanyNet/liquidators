use std::{collections::HashSet, time::{Duration, Instant}};

use diff::Diff;
use gamelibrary::{get_angle_to_mouse, mouse_world_pos, rapier_mouse_world_pos, rapier_to_macroquad, sound::soundmanager::SoundHandle, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, sync_arena::{Index, SyncArena}, texture_loader::TextureLoader, time::Time, traits::{draw_texture_onto_physics_body, HasPhysics}};
use macroquad::{color::{RED, WHITE}, input::is_mouse_button_released, math::{vec2, Vec2}, miniquad::TextureParams, shapes::{draw_circle, draw_rectangle}, texture::{draw_texture_ex, DrawTextureParams}};
use nalgebra::{point, vector, Const, OPoint};
use parry2d::{query::Ray, shape::Shape};
use rapier2d::prelude::{ColliderHandle, QueryFilter, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{bullet_trail::BulletTrail, collider_from_texture_size, damage_number::{self, DamageNumber}, enemy::Enemy, muzzle_flash::MuzzleFlash, player::player::{Facing, Player, PlayerWeapon}, weapon::Weapon, Grabbable, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Shotgun {
    pub weapon: Weapon
}

impl Shotgun {

    pub fn rigid_body(&self) -> SyncRigidBodyHandle {
        self.weapon.rigid_body
    }

    pub fn aim_angle_offset(&self) -> f32 {
        self.weapon.aim_angle_offset
    }

    pub fn collider(&self) -> SyncColliderHandle {
        self.weapon.collider
    }

    pub fn facing(&self) -> &Facing {
        &self.weapon.facing
    }

    pub fn set_facing(&mut self, facing: Facing) {
        self.weapon.facing = facing
    }

    pub fn owner(&self) -> &String {
        &self.weapon.owner
    }

    pub fn set_owner(&mut self, owner: String) {
        self.weapon.owner = owner
    }
    
    pub fn new(space: &mut Space, pos: Vec2, owner: String, player_rigid_body_handle: Option<SyncRigidBodyHandle>, textures: &mut TextureLoader) -> Self {

        Self {
            weapon: Weapon::new(
                space, 
                pos, 
                owner, 
                player_rigid_body_handle, 
                textures, "assets/shotgun.png".to_string(), 
                2., 
                Some(0.),
                Some(1.),
                "assets/sounds/shotgun/fire.wav",
                20.,
                10.,
                0.,
                0.
            ),
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
        self.weapon.owner_tick(players, hit_markers, space, ctx, enemies, damage_numbers, bullet_trails);
        
    }

    pub fn all_tick(&mut self, players: &mut SyncArena<Player>, space: &mut Space, ctx: &mut TickContext) {
        self.weapon.all_tick(players, space, ctx);
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

        self.weapon.tick(players, space, hit_markers, ctx, enemies, damage_numbers, bullet_trails);
        
        

        
    }   

    pub fn sync_sound(&mut self, ctx: &mut TickContext) {
        self.weapon.sync_sound(ctx);
    }

    pub fn get_weapon_tip(&self, space: &Space) -> OPoint<f32, Const<2>>{
        self.weapon.get_weapon_tip(space)
    }

    pub fn get_weapon_rear(&self, space: &Space) -> OPoint<f32, Const<2>> {
        self.weapon.get_weapon_rear(space)
    }

    pub fn shake_screen(&self, ctx: &mut TickContext) {
        self.weapon.shake_screen(ctx);
    }

    pub fn play_sound(&mut self) {
        self.weapon.play_sound();
    }

    pub fn knockback_player(&mut self, space: &mut Space, bullet_vector: Vec2) {
        self.weapon.knockback_player(space, bullet_vector);
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
        
        self.weapon.fire(space, players, enemies, hit_markers, damage_numbers, bullet_trails, ctx);

    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, flip_x: bool, flip_y: bool) {

        self.weapon.draw(space, textures, flip_x, flip_y).await

    }
}

impl Into<PlayerWeapon> for Shotgun {
    fn into(self) -> PlayerWeapon {
        PlayerWeapon::Shotgun(self)
    }
}

