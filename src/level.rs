use std::fs;

use diff::Diff;
use gamelibrary::{macroquad_to_rapier, mouse_world_pos, rapier_mouse_world_pos, space::Space, swapiter::SwapIter, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{color::RED, input::{self, is_key_down, is_key_pressed, is_key_released, KeyCode}, math::{Rect, Vec2}};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, RigidBodyBuilder};
use serde::{Deserialize, Serialize};

use crate::{boat::Boat, brick::Brick, player::{body_part::BodyPart, player::Player}, portal::Portal, portal_bullet::PortalBullet, radio::{Radio, RadioBuilder}, shotgun::Shotgun, sky::Sky, structure::Structure, teleporter::Teleporter, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Level {
    pub structures: Vec<Structure>,
    pub players: Vec<Player>,
    pub space: Space,
    pub bricks: Vec<Brick>,
    pub radios: Vec<Radio>,
    pub shotguns: Vec<Shotgun>,
    pub portal_bullets: Vec<PortalBullet>,
    pub portals: Vec<Portal>,
    pub sky: Sky,
    #[serde(default)]
    pub boats: Vec<Boat>,
    pub body_parts: Vec<BodyPart>,
    pub teleporters: Vec<Teleporter>
}

impl Level {
    pub fn empty() -> Self {
        let mut level = Level { 
            bricks: vec![],
            structures: vec![],
            players: vec![],
            space: Space::new(),
            radios: vec![],
            shotguns: vec![],
            portal_bullets: vec![],
            portals: vec![],
            sky: Sky::new(),
            boats: vec![],
            body_parts: vec![],
            teleporters: Vec::new()
        };
    
        level.space.gravity.y = -980.;
        
        level
    }

    pub fn from_save(path: String) -> Self {
        
        let bytes = fs::read(path).unwrap();


        let level: Self = serde_yaml::from_slice(&bytes).unwrap();
        
        level
    }

    pub fn tick(
        &mut self,
        ctx: &mut TickContext,
    ) {

        if is_key_down(KeyCode::C) {

            let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

            for (_, rigid_body) in self.space.rigid_body_set.iter_mut() {

                let distance_to_mouse = Vec2::new(mouse_pos.x - rigid_body.translation().x, mouse_pos.y - rigid_body.translation().y);

                rigid_body.apply_impulse(vector![distance_to_mouse.x * 5., distance_to_mouse.y * 5.].into(), true);

            }
        }
        
        for portal_bullet in &mut self.portal_bullets {

            portal_bullet.tick();
            
        }

        for teleporter in &mut self.teleporters {
            teleporter.tick(ctx, &mut self.space, &mut self.players);
        }
        for shotgun in &mut self.shotguns {

            shotgun.tick(&mut self.players, &mut self.space, ctx);

        }

        let mut players_iter = SwapIter::new(&mut self.players);

        while players_iter.not_done() {

            let (players, mut player) = players_iter.next();

            if player.owner != *ctx.uuid {
                players_iter.restore(player);
                
                continue;
            }

            player.tick(&mut self.space, &mut self.structures, &mut self.teleporters, ctx, players);
                

            players_iter.restore(player);   

        }

        for structure in &mut self.structures {

            structure.tick(ctx, &mut self.space, &self.players);
            
        }  

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

        self.space.step(&ctx.owned_rigid_bodies, &ctx.owned_colliders, ctx.last_tick);
        
    }

    pub fn editor_tick(&mut self, camera_rect: &Rect, uuid: &String, textures: &mut TextureLoader) {

        self.editor_spawn_structure(camera_rect, uuid);
        self.editor_spawn_brick(camera_rect, uuid);
        self.editor_spawn_radio(camera_rect, uuid);
        self.editor_spawn_shotgun(camera_rect, uuid, textures);

        for structure_index in 0..self.structures.len() {
            let mut structure = self.structures.remove(structure_index);

            structure.tick_editor(self, camera_rect, uuid);

            self.structures.insert(structure_index, structure);

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

    pub fn editor_spawn_shotgun(&mut self, camera_rect: &Rect, uuid: &String, textures: &mut TextureLoader) {

        if is_key_pressed(input::KeyCode::P) {
            Shotgun::spawn(&mut self.space, rapier_mouse_world_pos(camera_rect), &mut self.shotguns, uuid.clone(), textures);

        }
    }

    pub fn editor_spawn_structure(&mut self, camera_rect: &Rect, uuid: &String) {

        if is_key_released(input::KeyCode::E) {

            let mouse_world_pos = mouse_world_pos(camera_rect);

            let rapier_mouse_world_pos = macroquad_to_rapier(&mouse_world_pos);
            
            let new_structure = Structure::new(rapier_mouse_world_pos, &mut self.space, uuid.clone());
            
            self.structures.push(new_structure);

        }

        if is_key_released(input::KeyCode::Q) {

            let mouse_world_pos = mouse_world_pos(camera_rect);

            let rapier_mouse_world_pos = macroquad_to_rapier(&mouse_world_pos);

            let mut rigid_body_handle = self.space.rigid_body_set.insert(
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

            let collider_handle = self.space.collider_set.insert_with_parent(collider, &mut rigid_body_handle, &mut self.space.rigid_body_set);

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
                joint_test: None.into()
            };
            
            self.structures.push(new_structure);

        }
    }

    pub async fn editor_draw(&mut self, textures: &mut TextureLoader) {
        for structure in &mut self.structures {

            let texture_path = structure.sprite_path.clone() ;
            structure.debug_draw(&self.space, &texture_path, textures).await;
        }

        for radio in &mut self.radios {
            radio.draw(textures, &self.space).await;
        }

        for shotgun in &self.shotguns {
            shotgun.draw(&self.space, textures, false, false).await;
        }

        for brick in &mut self.bricks {
            brick.editor_draw(&self.space, textures).await
        }
    }
    pub async fn draw(&mut self, textures: &mut TextureLoader, camera_rect: &Rect) {

        //self.sky.draw();

        for teleporter in &self.teleporters {
            teleporter.draw(&self.space);
        }
        for shotgun in &self.shotguns {
            shotgun.draw(&self.space, textures, false, false).await;
        }

        for structure in self.structures.iter_mut() {

            let texture_path = structure.sprite_path.clone();

            structure.draw(&self.space, &texture_path, textures).await;
        }

        for boat in &mut self.boats {
            boat.draw_hitbox(&self.space)
        }

        for boat in &self.boats {
            boat.draw_top(&self.space, textures).await
        }

        for player in self.players.iter() {
            player.draw(&self.space, textures, camera_rect).await;
        }

        for boat in &self.boats {
            boat.draw_bottom(&self.space, textures).await
        }

        for brick in &mut self.bricks {
            brick.draw(&self.space, textures).await;
        }

        for portal_bullet in &self.portal_bullets {
            portal_bullet.draw().await;
        }

        for portal in self.portals.iter() {
            portal.draw(&self.space).await
        }
    }
}