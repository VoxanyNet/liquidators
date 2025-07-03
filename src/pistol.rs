use std::collections::HashSet;

use diff::Diff;
use gamelibrary::{space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, sync_arena::SyncArena, texture_loader::TextureLoader};
use macroquad::math::Vec2;
use serde::{Deserialize, Serialize};

use crate::{bullet_trail::BulletTrail, damage_number::DamageNumber, enemy::Enemy, player::{self, player::{Facing, Player}}, weapon::Weapon, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Pistol {
    weapon: Weapon
}

impl Pistol {
    pub fn new(
        space: &mut Space, 
        pos: Vec2, 
        owner: String, 
        player_rigid_body_handle: Option<SyncRigidBodyHandle>, 
        textures: &mut TextureLoader,
    ) -> Self {
        Self {
            weapon: Weapon::new(
                space, 
                pos, 
                owner, 
                player_rigid_body_handle, 
                textures, "assets/weapons/pistol.png".to_string(),
                1.3, 
                Some(0.),
                Some(1.),
                "assets/sounds/pistol.wav",
                50.,
                5.,
                0.,
                0.
            )
        }
    }

    pub fn aim_angle_offset(&self) -> f32 {
        self.weapon.aim_angle_offset
    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, flip_x: bool, flip_y: bool) {
        self.weapon.draw(space, textures, flip_x, flip_y).await
    }

    pub fn collider(&self) -> SyncColliderHandle {
        self.weapon.collider
    }

    pub fn rigid_body(&self) -> SyncRigidBodyHandle {
        self.weapon.rigid_body
    }

    pub fn tick(
        &mut self, 
        players: &mut SyncArena<Player>, 
        space: &mut Space, 
        hit_markers: &mut Vec<Vec2>, 
        ctx: &mut TickContext, 
        enemies: &mut SyncArena<Enemy>, 
        damage_numbers: &mut HashSet<DamageNumber>, 
        bullet_trails: &mut SyncArena<BulletTrail>) 
    {
        self.weapon.tick(players, space, hit_markers, ctx, enemies, damage_numbers, bullet_trails);
    }

    pub fn set_facing(&mut self, facing: Facing) {
        self.weapon.facing = facing
    }

}

impl Into<player::player::PlayerWeapon> for Pistol {
    fn into(self) -> player::player::PlayerWeapon {
        player::player::PlayerWeapon::Pistol(self)
    }
}