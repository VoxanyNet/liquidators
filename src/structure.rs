use std::error::Error;

use diff::Diff;
use futures::executor::block_on;
use gamelibrary::{log, menu::Menu, mouse_world_pos, rapier_mouse_world_pos, rapier_to_macroquad, space::{Space, SyncColliderHandle, SyncImpulseJointHandle, SyncRigidBodyHandle}, sync_arena::SyncArena, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{camera::Camera2D, color::{Color, DARKGRAY, RED, WHITE}, input::{self, is_key_down, is_key_released, is_mouse_button_pressed, is_mouse_button_released, KeyCode}, math::{Rect, Vec2}, miniquad::{self, gl::GL_SCISSOR_TEST}, prelude::{gl_use_default_material, gl_use_material, load_material, Material, MaterialParams}, shapes::{draw_circle, draw_line, draw_rectangle, draw_rectangle_ex}, text::draw_text, texture::{draw_texture, draw_texture_ex, load_texture, DrawTextureParams}, window::get_internal_gl};
use nalgebra::vector;
use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, prelude::{ColliderBuilder, RigidBodyBuilder}};
use serde::{Serialize, Deserialize};

use crate::{level::Level, player::{self, player::Player}, weapon::BulletImpactData, Grabbable, TickContext};

#[derive(Serialize, serde::Deserialize, Diff, PartialEq, Clone, Debug, Default)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Structure {
    pub rigid_body_handle: SyncRigidBodyHandle,
    pub collider_handle: SyncColliderHandle,
    pub color: macroquad::color::Color,
    pub menu: Option<Menu>,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub owner: Option<String>,
    pub editor_owner: String,
    pub sprite_path: String,
    pub last_ownership_change: u64,
    pub grabbing: bool,
    pub particles: Vec<Vec2>,
    pub joint_test: Box<Option<Structure>>,
    #[serde(default)]
    pub stupid: Rect,
    #[serde(default)]
    pub joint_handle: Option<SyncImpulseJointHandle>,
    #[serde(default)]
    pub max_impulse: f32
}

impl Grabbable for Structure {
    fn grabbing(&mut self) -> &mut bool {
        &mut self.grabbing
    }
}
impl Structure {

    pub fn break_joint(&mut self, space: &mut Space) {
        let joint = match &self.joint_handle {
            Some(joint_handle) => {
                space.sync_impulse_joint_set.get_sync_mut(*joint_handle).unwrap()
            },
            None => return,
        };

        if joint.impulses.magnitude() > self.max_impulse {
            self.max_impulse = joint.impulses.magnitude();
        }

        dbg!(self.max_impulse);


    }


    pub fn handle_bullet(&mut self, impact_data: &BulletImpactData, space: &mut Space) {

        if let Some(joint_handle) = self.joint_handle {
            space.sync_impulse_joint_set.remove(joint_handle);

            self.joint_handle = None;
        }

    }

    pub fn despawn(self, space: &mut Space) {

        // removes the body AND the collider!
        space.sync_rigid_body_set.remove_sync(
            self.rigid_body_handle, 
            &mut space.island_manager, 
            &mut space.sync_collider_set, 
            &mut space.sync_impulse_joint_set, 
            &mut space.multibody_joint_set, 
            true
        );
    } 

    pub fn new(pos: Vec2, space: &mut Space, owner: String) -> Self {


        let rigid_body_handle = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::dynamic()
                .position(
                    vector![pos.x, pos.y].into()
                )
                .soft_ccd_prediction(20.)
                .ccd_enabled(true)
        );

        let collider = ColliderBuilder::cuboid(20., 20.)
            .mass(100.)
            .build();
        

        let collider_handle = space.sync_collider_set.insert_with_parent_sync(collider, rigid_body_handle, &mut space.sync_rigid_body_set);

        Structure { 
            editor_owner: owner.clone(),
            rigid_body_handle: rigid_body_handle,
            collider_handle: collider_handle,
            color: RED,
            menu: None,
            selected: false,
            dragging: false,
            owner: Some(owner),
            drag_offset: None,
            sprite_path: "assets/structure/brick_block.png".to_string(),
            last_ownership_change: 0,
            grabbing: false,
            particles: vec![],
            joint_test: None.into(),
            stupid: Rect::new(0., 0., 50., 50.),
            joint_handle: None,
            ..Default::default()
        }
    }

    
    pub fn spawn_menu(&mut self, space: &mut Space, camera_rect: &Rect) {
        
        if !is_mouse_button_released(input::MouseButton::Right) {
            return;
        }

        let mouse_pos = mouse_world_pos(camera_rect);
        let mouse_rapier_coords = rapier_mouse_world_pos(camera_rect);

        if !self.contains_point(space, mouse_rapier_coords) {
            return
        }

        let mut menu = Menu::new(
            mouse_pos,
            DARKGRAY,
            "assets/fonts/CutePixel.ttf".to_string(), 
            None,
            None
        );

        
        // we can block here because we arent going to run this on wasm
        block_on(menu.add_button("Delete".to_string()));
        block_on(menu.add_button("Zero Velocity".to_string()));

        self.menu = Some(menu);
    }

    pub fn tick(&mut self, ctx: &mut TickContext, space: &mut Space, players: &SyncArena<Player>) {
        
        // let collider = space.collider_set.get(self.collider_handle).unwrap();
        // let body_transform = space.rigid_body_set.get(self.rigid_body_handle).unwrap().position();

        // let body = space.rigid_body_set.get(self.rigid_body_handle).unwrap();

        if is_key_down(KeyCode::Down) {

            if is_key_down(KeyCode::LeftShift) {
                self.stupid.h += 1.;
            }

            self.stupid.y += 1.;

        }

        if is_key_down(KeyCode::Up) {

            if is_key_down(KeyCode::LeftShift) {
                self.stupid.h -= 1.;
            }

            self.stupid.y -= 1.;

        }

        if is_key_down(KeyCode::Left) {

            if is_key_down(KeyCode::LeftShift) {
                self.stupid.w -= 1.;
            }

            self.stupid.x -= 1.;

        }

        if is_key_down(KeyCode::Right) {

            if is_key_down(KeyCode::LeftShift) {
                self.stupid.w += 1.;
            }

            self.stupid.x += 1.;

        }

        self.click_to_own(ctx, space);
        
        //self.update_selected(space, ctx.camera_rect);
        //self.update_is_dragging(space, ctx.camera_rect);
        //self.update_drag(space, ctx.camera_rect);

        match &mut *self.joint_test {
            Some(joint_test) => joint_test.tick(ctx, space, players),
            None => {},
        }
        if self.owner.is_none() {
            return;
        }

        if *ctx.uuid == self.owner.clone().unwrap() {

            self.break_joint(space);

            // we need to have a more efficient way of finding the currently controlled player
            for (_, player) in players {
                if player.owner == *ctx.uuid {

                    let reference_body = player.body.body_handle;

                    self.update_grabbing(space, ctx.camera_rect, Vec2::new(250., 250.), reference_body);

                    break;

                }
            }

            //self.update_grab_velocity(space);
        }

        match &self.owner {
            Some(owner) => {
                if owner == ctx.uuid {
                    ctx.owned_rigid_bodies.push(self.rigid_body_handle);
                    ctx.owned_colliders.push(self.collider_handle);
                }
            },
            None => {},
        }
        
    }

    pub fn click_to_own(&mut self, ctx: &mut TickContext, space: &mut Space) {
        if !is_mouse_button_released(input::MouseButton::Left) {
            return;
        }

        if !self.contains_point(space, rapier_mouse_world_pos(ctx.camera_rect)) {
            return;
        }

        self.owner = Some(ctx.uuid.clone());

    }


    pub fn update_owner(&mut self, ctx: &mut TickContext, space: &mut Space, players: &SyncArena<Player>) {

        for (_, player) in players {
            
        }
    }

    pub fn update_editor_owner(&mut self, editor_uuid: &String, space: &mut Space, camera_rect: &Rect) {
        // transfer ownership to whoever clicks this structure

        if !is_mouse_button_pressed(input::MouseButton::Left) {
            return;
        }

        if !self.contains_point(space, rapier_mouse_world_pos(camera_rect)) {
            return;
        }

        self.editor_owner = editor_uuid.clone();
    }

    pub fn tick_editor(mut self, space: &mut Space, camera_rect: &Rect, editor_uuid: &String) -> Option<Self> {

        self.update_editor_owner(editor_uuid, space, camera_rect);


        if self.editor_owner == *editor_uuid {

            match &mut self.menu {
                Some(menu) => menu.update(Some(camera_rect)),
                None => {}
            }

            self.editor_resize(space);
            self.spawn_menu(space, camera_rect);
            self.update_selected(space, camera_rect);
            self.update_is_dragging(space, camera_rect);
            self.update_drag(space, camera_rect);
            self.editor_rotate(space);

            if self.selected && is_key_released(KeyCode::Delete) {
                return None
            }
        }

        Some(self)


    }

    pub fn handle_menu(mut self, space: &mut Space) -> Option<Self> {

        // we probably shouldnt clone the menu but ehhhhh
        let menu = match self.menu.clone() {
            Some(menu) => menu,
            None => return Some(self),
        };

        for menu_item in menu.get_menu_items().clone() {

            if !menu_item.clicked {
                continue;
            }

            match menu_item.text.as_str() {
                "Delete" => {
                    self.menu = None;
                    space.sync_rigid_body_set.remove_sync(self.rigid_body_handle, &mut space.island_manager, &mut space.sync_collider_set, &mut space.sync_impulse_joint_set, &mut space.multibody_joint_set, true);
                    return None
                },
                "Zero Velocity" => {

                    let body = space.sync_rigid_body_set.get_sync_mut(self.rigid_body_handle).unwrap();
                    
                    body.set_linvel(vector![0., 0.], true);
                    //body.set_rotation(Rotation::from_angle(0.), true);

                    self.menu = None;   

                }
                _ => return Some(self)
            };

        };  

        // this is the result if the menu doesnt have any items or none of the items are hovered and clicked
        Some(self)
    }

    pub async fn debug_draw(&self, space: &Space, texture_path: &String, textures: &mut TextureLoader) {

        if self.selected {
            self.draw_outline(space, 10.);
        }
        
        self.draw_texture(space, texture_path, textures, false, false, 0.).await;

        match &self.menu {
            Some(menu) => menu.draw().await,
            None => {},
        }

        if let Some(joint_handle) = self.joint_handle {
            let joint = space.sync_impulse_joint_set.get_sync(joint_handle).unwrap();

            let pos_1 = space.sync_rigid_body_set.get_local(joint.body1).unwrap().translation();
            let pos_2 = space.sync_rigid_body_set.get_local(joint.body2).unwrap().translation();

            let pos_1_macroquad = rapier_to_macroquad(&Vec2::new(pos_1.x, pos_1.y));
            let pos_2_macroquad = rapier_to_macroquad(&Vec2::new(pos_2.x, pos_2.y));

            draw_line(pos_1_macroquad.x, pos_1_macroquad.y, pos_2_macroquad.x, pos_2_macroquad.y, 4., WHITE);


        }
    }

    pub async fn draw(&self, space: &Space, texture_path: &String, textures: &mut TextureLoader, camera: &Camera2D) {

        self.draw_texture(space, texture_path, textures, false, false, 0.).await;


        let texture = textures.get(&"assets/structure/brick_block.png".to_string()).await;

        let gl = unsafe {
            get_internal_gl()
        };

        gl.quad_context.texture_set_wrap(texture.raw_miniquad_id(), miniquad::TextureWrap::Repeat, miniquad::TextureWrap::Repeat);

        // let rapier_position = space.sync_rigid_body_set.get_sync(self.rigid_body_handle).unwrap().translation();

        // let macroquad_position = rapier_to_macroquad(&Vec2::new(rapier_position.x, rapier_position.y));

        // let half_extents = space.sync_collider_set.get_sync(self.collider_handle).unwrap().shape().as_cuboid().unwrap().half_extents;
        
        // let quad_gl = unsafe { &mut get_internal_gl() };

        // let camera_macroquad_position = camera.world_to_screen(macroquad_position);

        // log(&camera_macroquad_position.to_string());
        // log(&format!("macroquad pos: {}", macroquad_position));

        // log(&format!("{:?}", self.stupid));

        // let stupid_camera_pos = camera.world_to_screen(Vec2::new(self.stupid.x, self.stupid.y));

        // quad_gl.quad_gl.scissor(Some(
        //     (
        //         (camera_macroquad_position.x - half_extents.x) as i32,
        //         (camera_macroquad_position.y - half_extents.y) as i32,
        //         (half_extents.x * 2.) as i32,
        //         (half_extents.y * 2.) as i32
        //     )
        // ));
        
        // let texture = textures.get(&"assets/structure_tile.png".to_string()).await;

        // let mut params = DrawTextureParams::default();
        // params.dest_size = Some(Vec2::new(half_extents.x * 5., half_extents.y * 5.));

        // draw_texture_ex(texture, macroquad_position.x, macroquad_position.y , WHITE, params);

        // quad_gl.quad_gl.scissor(None);
        // if is_key_down(input::KeyCode::Down) {
        //     draw_rectangle(0., 0., 500., 500., RED);
        // }
        


        for particle in &self.particles {
            draw_circle(particle.x, particle.y, 20., WHITE);
        }

        //let pos = space.sync_rigid_body_set.get_sync(self.rigid_body_handle).unwrap().position().translation;

        //let pos = rapier_to_macroquad(&Vec2::new(pos.x, pos.y));

        //draw_text(self.owner.clone().unwrap().as_str(), pos.x, pos.y, 20., WHITE);


    }

}

impl HasPhysics for Structure {

    fn collider_handle(&self) -> &SyncColliderHandle {
        &self.collider_handle
    }
    fn drag_offset(&mut self) -> &mut Option<Vec2> {
        &mut self.drag_offset
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

    fn rigid_body_handle(&self) -> &SyncRigidBodyHandle {
        &self.rigid_body_handle
    }
}