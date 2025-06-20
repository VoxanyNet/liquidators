use std::collections::HashMap;

use chrono::TimeDelta;
use diff::Diff;
use gamelibrary::{space::{Space, SyncImpulseJointHandle}, sync_arena::{Index, SyncArena}, texture_loader::TextureLoader, time::Time, traits::HasPhysics};
use macroquad::{input::{is_key_down, KeyCode}, math::Vec2};
use nalgebra::{vector};
use parry2d::math::Vector;
use rapier2d::prelude::RevoluteJointBuilder;
use serde::{Deserialize, Serialize};

use crate::{player::{self, body_part::BodyPart, player::{Facing, Player}}, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Enemy {
    head: BodyPart,
    body: BodyPart,
    health: u32,
    facing: Facing,
    pub owner: String,
    head_body_joint: SyncImpulseJointHandle,
    last_jump: Time,
    player_target: Option<Index>
}

impl Enemy {

    pub fn new(position: Vec2, owner: String, space: &mut Space, textures: &mut TextureLoader) -> Self {

        let head = BodyPart::new(
            "assets/cat/head.png".to_string(), 
            2, 
            10.,
            position, 
            space, 
            textures, 
            owner.clone()
        );

        let body = BodyPart::new(
            "assets/cat/body.png".to_string(), 
            2, 
            100.,
            position, 
            space, 
            textures, 
            owner.clone()
        );

        let head_body_joint = space.sync_impulse_joint_set.insert_sync(
            space.sync_rigid_body_set.get_local_handle(body.body_handle), 
            space.sync_rigid_body_set.get_local_handle(head.body_handle), 
            RevoluteJointBuilder::new()
                .local_anchor1(vector![0., 0.].into())
                .local_anchor2(vector![0., -30.].into())
                .limits([-0.4, 0.4])
                .contacts_enabled(false)
            .build(), 
            true
        );

        Self {
            head,
            body,
            health: 100,
            facing: Facing::Right,
            owner,
            head_body_joint,
            last_jump: Time::new(0),
            player_target: None
        }
    }


    pub fn tick(&mut self, space: &mut Space, ctx: &mut TickContext, players: &SyncArena<Player>) {

        self.head.tick(space, ctx);

        self.body.tick(space, ctx);

        self.change_facing_direction(space);

        self.upright(space, ctx);

        self.target_player(players, space);

        self.follow_target(space, players);

    }

    pub fn follow_target(&mut self, space: &mut Space, players: &SyncArena<Player>) {
        let target_player = match &mut self.player_target {
            Some(target_player_index) => {
                players.get(target_player_index).unwrap()
            },
            None => return,
        };

        let target_player_body_translation = space.sync_rigid_body_set.get_sync(*target_player.rigid_body_handle()).unwrap().translation().clone();

        let enemy_body = space.sync_rigid_body_set.get_sync_mut(self.body.body_handle).unwrap();

        let target_vector = (target_player_body_translation - enemy_body.translation()).normalize();

        enemy_body.set_linvel(
            vector![
                target_vector.x * 100., 
                enemy_body.linvel().y
            ], 
            true
        );





    }

    pub fn target_player(&mut self, players: &SyncArena<Player>, space: &Space) {

        let mut lowest_distance_player: Option<Index> = None;
        let mut lowest_distance: Option<f32> = None;

        for (player_index, player) in players {
            let player_body = space.sync_rigid_body_set.get_sync(*player.rigid_body_handle()).unwrap();

            let enemy_body = space.sync_rigid_body_set.get_sync(self.body.body_handle).unwrap();

            let distance = (player_body.translation() - enemy_body.translation()).magnitude();

            if let Some(mut lowest_distance) = lowest_distance {
                if distance < lowest_distance {
                    lowest_distance = distance;

                    lowest_distance_player = Some(player_index);
                }
            }
            else {
                lowest_distance = Some(distance);
                lowest_distance_player = Some(player_index);

            }
        }

        // dont target players that are over 200 units away
        if let Some(lowest_distance) = lowest_distance {
            if lowest_distance > 200. {
                self.player_target = None
            }
        }

        self.player_target = lowest_distance_player;

    }

    pub fn upright(&mut self, space: &mut Space, ctx: &mut TickContext) {

        let body = space.sync_rigid_body_set.get_sync_mut(self.body.body_handle).unwrap();

        // dont try to upright if we aren't knocked over
        if body.rotation().angle().abs() < 0.5 {
            return;
        }

        // only try to jump every 3 seconds
        if self.last_jump.elapsed() > TimeDelta::seconds(3) {
            
            let current_velocity = body.linvel();

            // dont allow if moving if falling or jumping
            if current_velocity.y.abs() > 0.5 {
                return
            }
            
            body.set_linvel(vector![current_velocity.x, current_velocity.y + 500.], true);

            self.last_jump = Time::now();
        }

        let joint = space.sync_impulse_joint_set.get_sync_mut(self.head_body_joint).unwrap();

        joint.data.as_revolute_mut().unwrap().set_motor_position(0., 1000., 2.);

        //println!("{:?}", joint.data.as_revolute().unwrap().motor())
    }

    pub fn change_facing_direction(&mut self, space: &Space) {
        let velocity = space.sync_rigid_body_set.get_sync(self.body.body_handle).unwrap().linvel();


        if velocity.x > 100. {

            self.facing = Facing::Right
        }

        if velocity.x < -100. {

            self.facing = Facing::Left
        }
    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {

        let flip_x = match self.facing {
            Facing::Right => false,
            Facing::Left => true,
        };

        self.body.draw(textures, space, flip_x).await;

        self.head.draw(textures, space, flip_x).await;
    }
}