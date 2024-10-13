use diff::Diff;
use gamelibrary::{rapier_mouse_world_pos, space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use gilrs::{ev::Code, Button, Gamepad};
use macroquad::{input::{is_key_down, is_key_released, is_mouse_button_down, is_mouse_button_released, KeyCode}, math::{vec2, Rect, Vec2}};
use nalgebra::{vector, Rotation, Rotation2};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBody, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::{brick::Brick, level::Level, portal_bullet::PortalBullet, portal_gun::PortalGun, shotgun::Shotgun, TickContext};

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Player {
    pub rigid_body: RigidBodyHandle,
    pub collider: ColliderHandle,
    pub sprite_path: String,
    pub owner: String,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub max_speed: Vec2,
    pub shotgun: Option<Shotgun>,
    pub portal_gun: PortalGun
}

impl Player {

    pub fn spawn(players: &mut Vec<Player>, space: &mut Space, owner: String, position: &Vec2) {

        let rigid_body = RigidBodyBuilder::dynamic()
            .position(vector![position.x, position.y].into())
            .soft_ccd_prediction(20.)
            .build();

        let collider = ColliderBuilder::cuboid(18., 15.).mass(7000.).build();

        let rigid_body_handle = space.rigid_body_set.insert(rigid_body);
        let collider_handle = space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut space.rigid_body_set);

        players.push(
            Player {
                rigid_body: rigid_body_handle,
                collider: collider_handle,
                sprite_path: "assets/player/idle.png".to_string(),
                owner,
                selected: false,
                dragging: false,
                drag_offset: None,
                max_speed: vec2(350., 80.),
                shotgun: None,
                portal_gun: PortalGun::new()
                
            }
        )
    }

    pub fn tick(&mut self, level: &mut Level, ctx: &mut TickContext) {
        //self.launch_brick(level, ctx);
        self.control(level, ctx);
        self.update_portal_gun_pos(&level.space);
        self.fire_portal_gun(ctx.camera_rect, &mut level.portal_bullets);
        
    }

    pub fn fire_portal_gun(&mut self, camera_rect: &Rect, portal_bullets: &mut Vec<PortalBullet>) {
        if is_mouse_button_released(macroquad::input::MouseButton::Left) {
            self.portal_gun.fire(camera_rect, portal_bullets);
        }
    }

    pub fn update_portal_gun_pos(&mut self, space: &Space) {

        let body = space.rigid_body_set.get(*self.rigid_body_handle()).unwrap();

        self.portal_gun.position.x = body.position().translation.x;
        self.portal_gun.position.y = body.position().translation.y;
    }

    pub fn launch_brick(&mut self, level: &mut Level, ctx: &mut TickContext) {

        if !is_mouse_button_down(macroquad::input::MouseButton::Left) {
            return;
        }
        let (player_pos, player_rotation, player_velocity) = {
            let body = level.space.rigid_body_set.get(*self.rigid_body_handle()).unwrap();
            (body.position().clone(), body.rotation().clone(), body.linvel().clone())
        };

        let mouse_pos = rapier_mouse_world_pos(ctx.camera_rect);

        let mouse_body_distance = mouse_pos - Vec2::new(player_pos.translation.x, player_pos.translation.y); 
        
        let brick_spawn_point = Vec2::new(
            player_pos.translation.x + mouse_body_distance.normalize().x * 40., 
            player_pos.translation.y + mouse_body_distance.normalize().y * 40.
        );

        let brick = Brick::new(&mut level.space, brick_spawn_point, Some(ctx.uuid.clone()));
        
        let brick_body = level.space.rigid_body_set.get_mut(*brick.rigid_body_handle()).unwrap();

        let mut brick_velocity = player_velocity;
        brick_velocity.x += 5000. * mouse_body_distance.normalize().x;
        
        brick_body.set_rotation(player_rotation, true);
        brick_body.set_linvel(brick_velocity, true);

        level.bricks.push(brick);

    }

    pub fn jump(&mut self, rigid_body: &mut RigidBody, gamepad: Option<Gamepad>) {
        if is_key_down(KeyCode::Space) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::South)}) {

            // dont allow if moving if falling or jumping

            if rigid_body.linvel().y.abs() > 0.5 {
                return
            }

            // stop any y axis movement if going down
            if rigid_body.linvel().y.is_sign_negative() {
                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x, 0.],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x, rigid_body.linvel().y + 700.],
                true
            );

            
        }
    }

    pub fn control(&mut self, level: &mut Level, ctx: &mut TickContext) {

        let rigid_body = level.space.rigid_body_set.get_mut(self.rigid_body).unwrap();

        let gamepad: Option<Gamepad<'_>> = match ctx.active_gamepad {
            Some(active_gamepad) => {
                Some(ctx.gilrs.gamepad(*active_gamepad))
            },
            None => None,
        };

        self.jump(rigid_body, gamepad);

        if is_key_down(KeyCode::A) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::DPadLeft)}) {

            if rigid_body.linvel().x < -self.max_speed.x {
                return
            }

            if rigid_body.linvel().x.is_sign_positive() {
                rigid_body.set_linvel(
                    vector![0., rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x - 40., rigid_body.linvel().y],
                true
            );

        }

        if is_key_down(KeyCode::D) || gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::DPadRight)}) {

            if rigid_body.linvel().x > self.max_speed.x {
                return
            }

            if rigid_body.linvel().x.is_sign_negative() {
                rigid_body.set_linvel(
                    vector![0., rigid_body.linvel().y],
                    true
                )
            }

            rigid_body.set_linvel(
                vector![rigid_body.linvel().x + 40., rigid_body.linvel().y],
                true
            )
        }

        // if gamepad.map_or(false, |gamepad| {gamepad.is_pressed(gilrs::Button::RightTrigger)}) {
        //     rigid_body.set_rotation(
        //         Rotation, wake_up
        //     );
        // }

    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {

        self.draw_texture(&space, &self.sprite_path, textures).await;
    }
}

impl HasPhysics for Player {
    fn collider_handle(&self) -> &ColliderHandle {
        &self.collider
    }

    fn rigid_body_handle(&self) -> &RigidBodyHandle {
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

    fn drag_offset(&mut self) -> &mut Option<Vec2> {
        &mut self.drag_offset
    }
}