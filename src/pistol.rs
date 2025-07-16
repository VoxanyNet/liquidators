use std::{collections::HashSet, time::Duration};

use diff::Diff;
use gamelibrary::{sound::soundmanager::{SoundHandle, SoundManager}, space::{Space, SyncColliderHandle, SyncImpulseJointHandle, SyncRigidBodyHandle}, sync_arena::SyncArena, texture_loader::TextureLoader, time::Time};
use macroquad::{input::{is_key_released, is_mouse_button_released}, math::Vec2};
use serde::{Deserialize, Serialize};

use crate::{blood::Blood, bullet_trail::BulletTrail, damage_number::DamageNumber, enemy::Enemy, player::{self, player::{Facing, Player}}, weapon::Weapon, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Pistol {
    weapon: Weapon,
    last_reload: Time,
    rounds: u32,
    capacity: u32,
    reserve_capacity: u32,
    reload_duration: u32, // reload duration in millis
    sounds: Vec<SoundHandle>
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
                facing
            ),
            last_reload: Time::new(0),
            reload_duration: 2200,
            rounds: 12,
            capacity: 12,
            reserve_capacity: 24,
            sounds: Vec::new()
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

    pub async fn sync_sound(&mut self, ctx: &mut TickContext<'_>) {

        for sound in &mut self.sounds {
            ctx.sounds.sync_sound(sound).await
        }
        
        self.weapon.sync_sound(ctx).await
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
        blood: &mut HashSet<Blood>
    ) 
    {
        self.weapon.tick(players, space, hit_markers, ctx, enemies, damage_numbers, bullet_trails, blood);

        self.fire(space, players, enemies, hit_markers, damage_numbers, bullet_trails, blood, ctx);

        if is_key_released(macroquad::input::KeyCode::R) {
            self.reload(ctx);
        }
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
        ctx: &mut TickContext
    ) {

        if !is_mouse_button_released(macroquad::input::MouseButton::Left) {
            return;
        }

        ctx.console.log(format!("rounds: {}", self.rounds));
        ctx.console.log(format!("reserve: {}", self.reserve_capacity));
        
        // dont shoot while reloading
        if self.last_reload.elapsed().num_milliseconds() < self.reload_duration.into() {

            let mut sound = SoundHandle::new("assets/sounds/pistol_dry_fire.wav", [0., 0., 0.]);
            sound.play();

            self.sounds.push(sound);

            
            return;
        }

        // automatically reload if zero bullets
        if self.rounds == 0 {
            self.reload(ctx);

            return;
        }

        
        self.rounds -= 1;

        self.weapon.fire(space, players, enemies, hit_markers, damage_numbers, bullet_trails, blood, ctx);
    }

}

impl Into<player::player::PlayerWeapon> for Pistol {
    fn into(self) -> player::player::PlayerWeapon {
        player::player::PlayerWeapon::Pistol(self)
    }
}