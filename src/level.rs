use std::{collections::{HashMap, HashSet}, fs};

use diff::Diff;
use gamelibrary::{arenaiter::SyncArenaIterator, font_loader::FontLoader, log, macroquad_to_rapier, mouse_world_pos, rapier_mouse_world_pos, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, swapiter::SwapIter, sync_arena::{Index, SyncArena}, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{camera::Camera2D, color::{RED, WHITE}, input::{self, is_key_down, is_key_pressed, is_key_released, KeyCode}, math::{Rect, Vec2}, prelude::camera::mouse::{self, Camera}, shapes::{draw_rectangle, draw_rectangle_ex, draw_rectangle_lines, DrawRectangleParams}, text::draw_text_ex, texture::{draw_texture_ex, DrawTextureParams}};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};
use serde::{Deserialize, Serialize};

use crate::{blood::Blood, brick::Brick, bullet_trail::BulletTrail, damage_number::DamageNumber, enemy::Enemy, grenade::Grenade, pixel::Pixel, player::{self, body_part::BodyPart, player::{Player, WeaponTickParameters}}, portal::Portal, portal_bullet::PortalBullet, radio::{Radio, RadioBuilder}, shotgun::{self, Shotgun}, sky::Sky, structure::Structure, teleporter::Teleporter, weapon::Weapon, TickContext};


#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Level {
    pub structures: SyncArena<Structure>,
    pub players: SyncArena<Player>,
    pub space: Space,
    pub bricks: Vec<Brick>,
    pub radios: Vec<Radio>,
    pub shotguns: Vec<Shotgun>,
    pub portal_bullets: Vec<PortalBullet>,
    pub portals: Vec<Portal>,
    pub sky: Sky,
    pub body_parts: Vec<BodyPart>,
    pub teleporters: Vec<Teleporter>,
    pub hit_markers: Vec<Vec2>,
    pub grenades: Vec<Grenade>,
    pub enemies: SyncArena<Enemy>,
    pub pixels: HashSet<Pixel>,
    pub damage_numbers: HashSet<DamageNumber>,
    #[serde(default)]
    pub bullet_trails: SyncArena<BulletTrail>,
    #[serde(default)]
    pub blood: HashSet<Blood>,
}

impl Level {
    pub fn empty() -> Self {   

        let mut space = Space::new();

        //let ground_rigid_body = 
        let mut level = Level { 
            bricks: vec![],
            structures: SyncArena::new(),
            players: SyncArena::new(),
            space: Space::new(),
            radios: vec![],
            shotguns: vec![],
            portal_bullets: vec![],
            portals: vec![],
            sky: Sky::new(),
            body_parts: vec![],
            teleporters: Vec::new(),
            hit_markers: Vec::new(),
            grenades: Vec::new(),
            enemies: SyncArena::new(),
            pixels: HashSet::new(),
            damage_numbers: HashSet::new(),
            bullet_trails: SyncArena::new(),
            blood: HashSet::new()
        };
    
        level.space.gravity.y = -980.;
        
        level
    }

    
    pub async fn sync_sounds(&mut self, ctx: &mut TickContext<'_>) {
        for (_, player) in &mut self.players {
            player.sync_sound(ctx).await;
        }
    }
    pub fn spawn_pixel(&mut self, pos: Vec2, ctx: &mut TickContext) {
        self.pixels.insert(
            Pixel::new(
                WHITE, 
                rapier_mouse_world_pos(ctx.camera_rect), 
                &mut self.space, 
                None, 
                None, 
                ctx.uuid.clone()
            )
        );
    }

    pub fn from_save(path: String) -> Self {
        
        
        let bytes = fs::read(path).unwrap();


        let level: Self = serde_yaml::from_slice(&bytes).unwrap();
        
        level
    }   

    pub fn server_tick(&mut self) {

    }

    pub fn tick(
        &mut self,
        ctx: &mut TickContext,
    ) {

        
        for (_, bullet_trail) in &mut self.bullet_trails {
            if bullet_trail.owner == *ctx.uuid {
                bullet_trail.tick(ctx);
            }
        };

        //self.spawn_fixed_structure(ctx.camera_rect, ctx.uuid);

        self.spawn_damage_number(ctx.camera_rect);
        
        if is_key_released(KeyCode::C) {

            let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

            self.enemies.insert(
                Enemy::new(mouse_pos, ctx.uuid.clone(), &mut self.space, ctx.textures)
            
            );
        }

        if is_key_released(KeyCode::H) {
            let mut players_iter = SyncArenaIterator::new(&mut self.players);
    
            while let Some((mut player, _)) = players_iter.next() {
                if player.contains_point(&mut self.space, rapier_mouse_world_pos(ctx.camera_rect)) {
                    player.despawn(&mut self.space);
                }

                else {
                    players_iter.restore(player);
                }
            }
        }

        if is_key_down(KeyCode::J) {
            self.spawn_pixel(rapier_mouse_world_pos(ctx.camera_rect), ctx);

        }

        if is_key_released(KeyCode::G) {

            let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

            self.grenades.push(
                Grenade::new(mouse_pos, &mut self.space)
            );


        }

        for (_, enemy) in &mut self.enemies {

            if enemy.owner != *ctx.uuid {
                continue;
            }

            enemy.tick(&mut self.space, ctx, &self.players);
        }

        for grenade in &mut self.grenades {
            grenade.tick(&mut self.space, ctx);
        }
        
        for portal_bullet in &mut self.portal_bullets {

            portal_bullet.tick();
            
        }
        
        for pixel in self.pixels.iter() {
            pixel.tick(ctx);
        }

        if is_key_released(KeyCode::Delete) {
            self.hit_markers = Vec::new();
        }

        for teleporter in &mut self.teleporters {
            teleporter.tick(ctx, &mut self.space, &mut self.players);
        }

        let mut weapon_tick_parameters = WeaponTickParameters {
            players: &mut self.players,
            enemies: &mut self.enemies,
            structures: &mut self.structures
            
        };

        for shotgun in &mut self.shotguns {

            shotgun.tick(&mut self.space, &mut self.hit_markers, ctx, &mut self.damage_numbers, &mut self.bullet_trails, &mut self.blood, &mut weapon_tick_parameters);

        }

        let mut players_iter = &mut SyncArenaIterator::new(&mut self.players);

        while let Some((mut player, players)) = players_iter.next() {


            player.tick(
                &mut self.space, 
                &mut self.structures, 
                &mut self.bricks, 
                &mut self.teleporters, 
                &mut self.hit_markers, 
                ctx, 
                players, 
                &mut self.enemies,
                &mut self.damage_numbers,
                &mut self.bullet_trails,
                &mut self.blood
            );
                

            players_iter.restore(player);   

        }

        let mut structure_iter = SyncArenaIterator::new(&mut self.structures);

        for brick in &mut self.bricks {


            match brick.owner.clone() {
                Some(owner) => {
                    if owner != *ctx.uuid {
                        continue;
                    }
                },
                None => continue,
            }

            brick.tick(ctx);
            
        }

        self.space.step(&ctx.owned_rigid_bodies, &ctx.owned_colliders, ctx.owned_impulse_joints, ctx.last_tick_duration);
        
    }

    pub fn editor_tick(&mut self, camera_rect: &Rect, uuid: &String, textures: &mut TextureLoader) {

        self.editor_spawn_structure(camera_rect, uuid);
        self.editor_spawn_brick(camera_rect, uuid);
        self.editor_spawn_radio(camera_rect, uuid);

        let mut structures_iter = SyncArenaIterator::new(&mut self.structures);

        while let Some((structure, structures)) = structures_iter.next() {

            let new_structure = structure.tick_editor(&mut self.space, camera_rect, uuid);

            if let Some(new_structure) = new_structure {
                structures_iter.restore(new_structure);
            }

        }

        for radio_index in 0..self.radios.len() {
            let mut radio = self.radios.remove(radio_index);

            radio.tick_editor(self, camera_rect, uuid);

            self.radios.insert(radio_index, radio);
        }

        for brick_index in 0..self.bricks.len() {
            let mut brick = self.bricks.remove(brick_index);

            brick.editor_tick(uuid, &mut self.space, camera_rect);

            self.bricks.insert(brick_index, brick);
        }
    }

    pub fn editor_spawn_radio(&mut self, camera_rect: &Rect, uuid: &String) {
        if !is_key_released(input::KeyCode::T) {
            return
        }

        let radio = RadioBuilder::new(&mut self.space, &rapier_mouse_world_pos(camera_rect))
            .editor_owner(uuid.clone())
            .build();

        self.radios.push(radio);

    }

    pub fn editor_spawn_brick(&mut self, camera_rect: &Rect, uuid: &String) {

        if !is_key_released(input::KeyCode::B) {
            return
        }

        let pos = rapier_mouse_world_pos(camera_rect);

        self.bricks.push(
            Brick::new(&mut self.space, pos, Some(uuid.clone()))
        );
    }

    pub fn spawn_damage_number(&mut self, camera_rect: &Rect) {

        if !is_key_released(KeyCode::L) {
            return;
        }
        let mouse_pos = rapier_mouse_world_pos(camera_rect);

        self.damage_numbers.insert(
            DamageNumber::new(&mut self.space, 5, mouse_pos, Some(24), Some(WHITE))
        );
    }

    pub fn spawn_fixed_structure(&mut self, camera_rect: &Rect, uuid: &String) {
        if is_key_released(input::KeyCode::Q) {

            let mouse_world_pos = mouse_world_pos(camera_rect);

            let rapier_mouse_world_pos = macroquad_to_rapier(&mouse_world_pos);

            let rigid_body_handle = self.space.sync_rigid_body_set.insert_sync(
                RigidBodyBuilder::fixed()
                    .position(
                        vector![rapier_mouse_world_pos.x, rapier_mouse_world_pos.y].into()
                    )
                    .ccd_enabled(true)
            );

            let collider = ColliderBuilder::cuboid(20., 20.)
                .mass(10.)
                .restitution(0.)
                .build();

            let collider_handle = self.space.sync_collider_set.insert_with_parent_sync(collider, rigid_body_handle, &mut self.space.sync_rigid_body_set);

            let new_structure = Structure { 
                grabbing: false,
                editor_owner: uuid.clone(),
                rigid_body_handle: rigid_body_handle,
                collider_handle: collider_handle,
                color: RED,
                menu: None,
                selected: false,
                dragging: false,
                owner: Some(uuid.to_string()),
                drag_offset: None,
                sprite_path: "assets/structure/brick_block.png".to_string(),
                last_ownership_change: 0,
                particles: vec![],
                joint_test: None.into(),
                stupid: Rect::new(0., 0., 50., 50.),
                joint_handle: None,
                ..Default::default()
            };
            
            self.structures.insert(new_structure);

        }
    }

    pub fn editor_spawn_structure(&mut self, camera_rect: &Rect, uuid: &String) {

        let mouse_world_pos = mouse_world_pos(camera_rect);

        let rapier_mouse_world_pos = macroquad_to_rapier(&mouse_world_pos);


        if is_key_released(input::KeyCode::E) {

            
            
            let new_structure = Structure::new(rapier_mouse_world_pos, &mut self.space, uuid.clone());
            
            self.structures.insert(new_structure);

        }

        if is_key_released(input::KeyCode::Q) {

            let rapier_mouse_world_pos = macroquad_to_rapier(&mouse_world_pos);

            let rigid_body_handle = self.space.sync_rigid_body_set.insert_sync(
                RigidBodyBuilder::fixed()
                    .position(
                        vector![rapier_mouse_world_pos.x, rapier_mouse_world_pos.y].into()
                    )
                    .ccd_enabled(true)
            );

            let collider = ColliderBuilder::cuboid(20., 20.)
                .mass(10.)
                .restitution(0.)
                .build();

            let collider_handle = self.space.sync_collider_set.insert_with_parent_sync(collider, rigid_body_handle, &mut self.space.sync_rigid_body_set);

            let new_structure = Structure { 
                grabbing: false,
                editor_owner: uuid.clone(),
                rigid_body_handle: rigid_body_handle,
                collider_handle: collider_handle,
                color: RED,
                menu: None,
                selected: false,
                dragging: false,
                owner: None,
                drag_offset: None,
                sprite_path: "assets/structure/brick_block.png".to_string(),
                last_ownership_change: 0,
                particles: vec![],
                joint_test: None.into(),
                stupid: Rect::new(0., 0., 50., 50.),
                joint_handle: None,
                ..Default::default()
            };
            
            self.structures.insert(new_structure);

        }
    }

    pub async fn editor_draw(&self, textures: &mut TextureLoader, camera_rect: &Rect) {
        for (_, structure) in &self.structures {

            let texture_path = structure.sprite_path.clone() ;
            structure.debug_draw(&self.space, &texture_path, textures).await;
        }

        for radio in &self.radios {
            radio.draw(textures, &self.space).await;
        }

        for shotgun in &self.shotguns {
            shotgun.draw(&self.space, textures, false, false).await;
        }

        for brick in &self.bricks {
            brick.editor_draw(&self.space, textures).await
        }

        let mouse_world_pos = mouse_world_pos(camera_rect);
        // draw structure cursor
        draw_rectangle_lines(mouse_world_pos.x - 20., mouse_world_pos.y - 20., 40., 40., 4., WHITE);
    }

    pub async fn draw_hud(&self, ctx: &mut TickContext<'_>) {
        for (_, player) in &self.players {
            player.draw_hud(ctx).await
        }
    }
    pub async fn draw(
        &self, 
        textures: &mut TextureLoader, 
        camera_rect: &Rect,
        fonts: &mut FontLoader,
        camera: &Camera2D
    ) {

        let background_texture = textures.get(&"assets/background_tile.png".to_string()).await;

        let tile_width = background_texture.width() * 8.;
        let tile_height = background_texture.height() * 8.;

        let mut params = DrawTextureParams::default();

        params.dest_size = Some(Vec2::new(tile_height, tile_width));
        for x in 0..40 {
            for y in 0..40 {
                draw_texture_ex(background_texture, (x as f32 * tile_width) - 1000., (y as f32 * tile_height) - 1000., WHITE, params.clone());
            }
        }
        //self.sky.draw();

        for damage_number in &self.damage_numbers {
            damage_number.draw(&self.space, fonts);
        }
        for teleporter in &self.teleporters {
            teleporter.draw(&self.space);
        }
        for shotgun in &self.shotguns {
            shotgun.draw(&self.space, textures, false, false).await;
        }

        for (_, bullet_trail) in &self.bullet_trails {
            bullet_trail.draw();
        }
        for (_, enemy) in &self.enemies {
            enemy.draw(&self.space, textures).await;
        }

        for pixel in &self.pixels {
            pixel.draw(&self.space).await;
        }

        for grenade in &self.grenades {
            grenade.draw(&self.space, textures).await
        }

        for (_, structure) in self.structures.iter() {

            let texture_path = structure.sprite_path.clone();

            structure.draw(&self.space, &texture_path, textures, camera).await;
        }

        for (index, player) in &self.players {
            player.draw(&self.space, textures, camera_rect).await;
        }

        for brick in &self.bricks {
            brick.draw(&self.space, textures).await;
        }

        for portal_bullet in &self.portal_bullets {
            portal_bullet.draw().await;
        }

        for portal in self.portals.iter() {
            portal.draw(&self.space).await
        }

        for hitmarker in self.hit_markers.iter() {
            draw_rectangle_ex(hitmarker.x, hitmarker.y, 20., 20., DrawRectangleParams {
                offset: Vec2::new(0.5, 0.5),
                rotation: 0.,
                color: WHITE,
            });
        }

        for blood in &self.blood {
            blood.draw(&self.space).await
        }
    }
}