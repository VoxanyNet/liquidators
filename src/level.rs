use std::fs;

use diff::Diff;
use gamelibrary::{macroquad_to_rapier, mouse_world_pos, rapier_mouse_world_pos, space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{color::RED, input::{self, is_key_pressed, is_key_released}, math::Rect};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{brick::{Brick}, player::Player, radio::{Radio, RadioBuilder}, shotgun::Shotgun, structure::Structure, TickContext};

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
    pub shotguns: Vec<Shotgun>
}

impl Level {
    pub fn empty() -> Self {
        let mut level = Level { 
            bricks: vec![],
            structures: vec![],
            players: vec![],
            space: Space::new(),
            radios: vec![],
            shotguns: vec![]
        };
    
        level.space.gravity.y = -980.;
        
        level
    }

    pub fn from_save(path: String) -> Self {
        
        let bytes = fs::read(path).unwrap();

        let level: Self = bitcode::deserialize(&bytes).unwrap();
        
        level
    }

    pub fn tick(
        &mut self,
        ctx: &mut TickContext,
    ) {
        
        let mut owned_bodies: Vec<RigidBodyHandle> = vec![];
        let mut owned_colliders: Vec<ColliderHandle> = vec![];

        for player_index in 0..self.players.len() {

            let mut player = self.players.remove(player_index);

            if player.owner == *ctx.uuid {
                player.tick(self, ctx);

                owned_bodies.push(player.rigid_body);
                owned_colliders.push(player.collider);
            }            

            self.players.insert(player_index, player);
        }

        for structure_index in 0..self.structures.len() {
            let structure = self.structures.remove(structure_index);

            match &structure.owner {
                Some(owner) => {
                    if owner == ctx.uuid {
                        owned_bodies.push(structure.rigid_body_handle);
                        owned_colliders.push(structure.collider_handle);
                    }
                },
                None => {},
            }

            self.structures.insert(structure_index, structure);
        }  

        for brick_index in 0..self.bricks.len() {
            let mut brick = self.bricks.remove(brick_index);

            brick.tick(self, ctx);
            
            match &brick.owner {
                Some(owner) => {


                    if owner == ctx.uuid { 
                        
                        owned_bodies.push(*brick.rigid_body_handle());
                        owned_colliders.push(*brick.collider_handle());
                    }
                },
                None => {},
            }

            self.bricks.insert(brick_index, brick);
        }

        self.space.step(ctx.last_tick.elapsed(), &owned_bodies, &owned_colliders);
        
    }

    pub fn editor_tick(&mut self, camera_rect: &Rect, uuid: &String) {

        self.editor_spawn_structure(camera_rect, uuid);
        self.editor_spawn_brick(camera_rect, uuid);
        self.editor_spawn_radio(camera_rect, uuid);
        self.editor_spawn_shotgun(camera_rect);

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

    pub fn editor_spawn_shotgun(&mut self, camera_rect: &Rect) {

        if is_key_pressed(input::KeyCode::P) {
            let shotgun = Shotgun::new(&mut self.space, rapier_mouse_world_pos(camera_rect));

            self.shotguns.push(shotgun);

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

            let rigid_body_handle = self.space.rigid_body_set.insert(
                RigidBodyBuilder::fixed()
                    .position(
                        vector![rapier_mouse_world_pos.x, rapier_mouse_world_pos.y].into()
                    )
            );

            let collider = ColliderBuilder::cuboid(20., 20.)
                .mass(10.)
                .restitution(0.)
                .build();

            let collider_handle = self.space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut self.space.rigid_body_set);

            let new_structure = Structure { 
                editor_owner: uuid.clone(),
                rigid_body_handle: rigid_body_handle,
                collider_handle: collider_handle,
                color: RED,
                menu: None,
                selected: false,
                dragging: false,
                owner: None,
                drag_offset: None,
                sprite_path: "assets/structure/brick_block.png".to_string()
            };
            
            self.structures.push(new_structure);

        }
    }

    pub async fn editor_draw(&self, textures: &mut TextureLoader) {
        for structure in &self.structures {
            let texture_path = structure.sprite_path.clone() ;
            structure.debug_draw(&self.space, &texture_path, textures).await;
        }

        for radio in &self.radios {
            radio.draw(textures, &self.space).await;
        }

        for shotgun in &self.shotguns {
            shotgun.draw(&self.space, textures).await;
        }

        for brick in &self.bricks {
            brick.editor_draw(&self.space, textures).await
        }
    }
    pub async fn draw(&self, textures: &mut TextureLoader) {
        for structure in self.structures.iter() {

            let texture_path = structure.sprite_path.clone();

            structure.draw(&self.space, &texture_path, textures).await;
        }

        for player in self.players.iter() {
            player.draw(&self.space, textures).await;
        }

        for brick in &self.bricks {
            brick.draw(&self.space, textures).await;
        }
    }
}