use chrono::{DateTime, Utc};
use diff::Diff;
use gamelibrary::{menu::Menu, mouse_world_pos, rapier_mouse_world_pos, space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{color::{DARKGRAY, RED}, input::{self, is_mouse_button_pressed, is_mouse_button_released}, math::{Rect, Vec2}, miniquad::date::now};
use nalgebra::vector;
use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, prelude::{ColliderBuilder, RigidBodyBuilder}};
use serde::{Serialize, Deserialize};

use crate::level::Level;

#[derive(Serialize, serde::Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Structure {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub color: macroquad::color::Color,
    pub menu: Option<Menu>,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub owner: Option<String>,
    pub editor_owner: String,
    pub sprite_path: String,
    pub last_ownership_change: u64
}

impl Structure {

    pub fn new(pos: Vec2, space: &mut Space, owner: String) -> Self {


        let rigid_body_handle = space.rigid_body_set.insert(
            RigidBodyBuilder::dynamic()
                .position(
                    vector![pos.x, pos.y].into()
                )
                .ccd_enabled(true)
                .soft_ccd_prediction(20.)
        );

        let collider = ColliderBuilder::cuboid(20., 20.)
            .mass(100.)
            .build();
        

        let collider_handle = space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut space.rigid_body_set);

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
            last_ownership_change: 0
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
            DARKGRAY
        );

        menu.add_button("Delete".to_string());
        menu.add_button("Zero Velocity".to_string());

        self.menu = Some(menu);
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

    pub fn tick_editor(&mut self, level: &mut Level, camera_rect: &Rect, editor_uuid: &String) {

        self.update_editor_owner(editor_uuid, &mut level.space, camera_rect);

        if self.editor_owner == *editor_uuid {

            match &mut self.menu {
                Some(menu) => menu.update(Some(camera_rect)),
                None => {}
            }

            self.editor_resize(&mut level.space);
            self.spawn_menu(&mut level.space, camera_rect);
            self.update_selected(&mut level.space, camera_rect);
            self.update_is_dragging(&mut level.space, camera_rect);
            self.update_drag(&mut level.space, camera_rect);
            self.editor_rotate(&mut level.space);
        }


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
                    space.rigid_body_set.remove(self.rigid_body_handle, &mut space.island_manager, &mut space.collider_set, &mut space.impulse_joint_set, &mut space.multibody_joint_set, true);
                    return None
                },
                "Zero Velocity" => {

                    let body = space.rigid_body_set.get_mut(self.rigid_body_handle).unwrap();
                    
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
        self.draw_outline(space, 10.).await;
        self.draw_texture(space, texture_path, textures, false, false).await;

        match &self.menu {
            Some(menu) => menu.draw().await,
            None => {},
        }
    }

    pub async fn draw(&self, space: &Space, texture_path: &String, textures: &mut TextureLoader) {
        self.draw_texture(space, texture_path, textures, false, false).await;
    }

}

impl HasPhysics for Structure {

    fn collider_handle(&self) -> &ColliderHandle {
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

    fn rigid_body_handle(&self) -> &RigidBodyHandle {
        &self.rigid_body_handle
    }
}