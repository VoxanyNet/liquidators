use diff::Diff;
use gamelibrary::{menu::Menu, mouse_world_pos, rapier_mouse_world_pos, rapier_to_macroquad, space::Space, texture_loader::TextureLoader, traits::HasCollider};
use macroquad::{color::{DARKGRAY, WHITE}, input::{self, is_key_down, is_mouse_button_released}, math::{vec2, Rect, Vec2}, shapes::DrawRectangleParams, texture::{draw_texture_ex, DrawTextureParams}};
use nalgebra::vector;
use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, math::Rotation};
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
    pub sprite_path: String
}

impl Structure {

    pub async fn draw(&mut self, space: &Space, textures: &mut TextureLoader) {
        let rigid_body = space.rigid_body_set.get(self.rigid_body_handle).unwrap();
        let collider = space.collider_set.get(self.collider_handle).unwrap();

        // use the shape to define how large we should draw the texture
        // maybe we should change this
        let shape = collider.shape().as_cuboid().unwrap();

        let position = rigid_body.position().translation;
        let rotation = rigid_body.rotation().angle();

        let draw_pos = rapier_to_macroquad(&vec2(position.x, position.y));

        // draw the outline
        if *self.selected() {
            macroquad::shapes::draw_rectangle_ex(
                draw_pos.x,
                draw_pos.y, 
                (shape.half_extents.x * 2.) + 10., 
                (shape.half_extents.y * 2.) + 10., 
                DrawRectangleParams { offset: macroquad::math::Vec2::new(0.5, 0.5), rotation: rotation * -1., color: WHITE }
            );
        } 

        draw_texture_ex(
            textures.get(&self.sprite_path).await, 
            draw_pos.x - shape.half_extents.x, 
            draw_pos.y - shape.half_extents.y, 
            WHITE, 
            DrawTextureParams {
                dest_size: Some(vec2(shape.half_extents.x * 2., shape.half_extents.y * 2.)),
                source: None,
                rotation: rotation * -1.,
                flip_x: false,
                flip_y: false,
                pivot: None,
            }
        );

        
    }

    pub fn resize(&mut self, space: &mut Space) {

        if !*self.selected() {
            return;
        }
        let collider = space.collider_set.get_mut(self.collider_handle).unwrap();
        let rigid_body = space.rigid_body_set.get_mut(self.rigid_body_handle).unwrap();

        let shape = collider.shape_mut().as_cuboid_mut().unwrap();

        let increase_unit = 10.;

        if is_key_down(input::KeyCode::Right) {
            
            shape.half_extents.x += increase_unit;
            rigid_body.set_position(vector![rigid_body.position().translation.x + increase_unit, rigid_body.position().translation.y].into(), true)
        }

        if is_key_down(input::KeyCode::Up) {
            shape.half_extents.y += increase_unit;
            rigid_body.set_position(vector![rigid_body.position().translation.x, rigid_body.position().translation.y + increase_unit].into(), true)
        }

        if is_key_down(input::KeyCode::Down) {
            shape.half_extents.y -= increase_unit;
            rigid_body.set_position(vector![rigid_body.position().translation.x, rigid_body.position().translation.y - increase_unit].into(), true)
        }

        if is_key_down(input::KeyCode::Left) {
            shape.half_extents.x -= increase_unit;
            rigid_body.set_position(vector![rigid_body.position().translation.x - increase_unit, rigid_body.position().translation.y].into(), true)
        }

        if shape.half_extents.x <= 0. {
            shape.half_extents.x = 1.
        }

        if shape.half_extents.y <= 0. {
            shape.half_extents.y = 1.
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

    pub fn tick_editor(&mut self, level: &mut Level, camera_rect: &Rect, client_uuid: &String) {

        

        if self.editor_owner == *client_uuid {

            match &mut self.menu {
                Some(menu) => menu.update(camera_rect),
                None => {}
            }

            self.resize(&mut level.space);
            self.spawn_menu(&mut level.space, camera_rect);
            self.update_selected(&mut level.space, camera_rect);
            self.update_is_dragging(&mut level.space, camera_rect);
            self.update_drag(&mut level.space, camera_rect);
            self.rotate(&mut level.space);
        }


    }

    pub fn update_resize(&mut self) {
        if !*self.selected() {return}
    }

    pub fn rotate(&mut self, space: &mut Space) {
        if !*self.selected() {return}

        if !is_key_down(input::KeyCode::R) {return}

        let rigid_body = space.rigid_body_set.get_mut(self.rigid_body_handle).unwrap();
        
        rigid_body.set_rotation(Rotation::from_angle(rigid_body.rotation().angle() - 0.05), true);
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
}

impl HasCollider for Structure {

    fn collider_handle(&self) -> &ColliderHandle {
        &self.collider_handle
    }
    fn drag_offset(&mut self) -> &mut Option<Vec2> {
        &mut self.drag_offset
    }
    
    fn selected(&mut self) -> &mut bool {
        &mut self.selected
    }

    fn dragging(&mut self) -> &mut bool {
        &mut self.dragging
    }
}