use std::{collections::HashSet, time::Duration};

use diff::Diff;
use gamelibrary::{sound::soundmanager::{SoundHandle, SoundManager}, space::{Space, SyncColliderHandle, SyncImpulseJointHandle, SyncRigidBodyHandle}, sync_arena::SyncArena, texture_loader::TextureLoader, time::Time};
use macroquad::{input::{is_key_released, is_mouse_button_released}, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::{blood::Blood, bullet_trail::BulletTrail, damage_number::DamageNumber, enemy::Enemy, player::{self, player::{Facing, Player}}, weapon::{Hitscan, Weapon}, TickContext};

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
        facing: Facing
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
                8.,
                0.,
                0.,
                Some("assets/weapons/pistol/casing.png".to_string()),
                Vec2::new(32., 22.),
                facing,
                2200,
                12,
                12,
                24,
            )
        }
    }


    pub async fn sync_sound(&mut self, ctx: &mut TickContext<'_>) {
        
        self.weapon.sync_sound(ctx).await
    }

    pub fn aim_angle_offset(&self) -> f32 {
        self.weapon.aim_angle_offset
    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader, flip_x: bool, flip_y: bool) {
        self.weapon.draw(space, textures, flip_x, flip_y).await
    }

    pub async fn draw_hud(&self, ctx: &mut TickContext<'_>) {
        self.weapon.draw_hud(ctx).await
    }

    pub fn collider(&self) -> SyncColliderHandle {
        self.weapon.collider
    }

    pub fn player_joint_handle(&self) -> Option<SyncImpulseJointHandle> {
        self.weapon.player_joint_handle
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
        bullet_trails: &mut SyncArena<BulletTrail>,
        blood: &mut HashSet<Blood>,
        hitscans: &mut SyncArena<Hitscan>
    ) 
    {
        self.weapon.tick(players, space, hit_markers, ctx, enemies, damage_numbers, bullet_trails, blood);

        self.fire(space, players, enemies, hit_markers, damage_numbers, bullet_trails, blood, ctx, hitscans);
    }

    pub fn set_facing(&mut self, facing: Facing) {
        self.weapon.facing = facing
    }

    pub fn fire(
        &mut self, 
        space: &mut Space, 
        players: &mut SyncArena<Player>,
        enemies: &mut SyncArena<Enemy>, 
        hit_markers: &mut Vec<Vec2>, 
        damage_numbers: &mut HashSet<DamageNumber>,
        bullet_trails: &mut SyncArena<BulletTrail>,
        blood: &mut HashSet<Blood>,
        ctx: &mut TickContext,
        hitscans: &mut SyncArena<Hitscan>
    ) {

        if !is_mouse_button_released(macroquad::input::MouseButton::Left) {
            return;
        }
        

        self.weapon.fire(space, players, enemies, hit_markers, damage_numbers, bullet_trails, blood, ctx, hitscans);
    }

}

impl Into<player::player::PlayerWeapon> for Pistol {
    fn into(self) -> player::player::PlayerWeapon {
        player::player::PlayerWeapon::Pistol(self)
    }
}