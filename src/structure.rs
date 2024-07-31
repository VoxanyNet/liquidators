use diff::Diff;
use gamelibrary::{macroquad_to_rapier, menu::Menu, mouse_world_pos, rapier_mouse_world_pos, space::Space, traits::{Color, Drawable, HasCollider, HasRigidBody}};
use macroquad::{color::DARKGRAY, input::{self, is_key_down, is_mouse_button_released, mouse_position}, math::{Rect, Vec2}};
use nalgebra::{point, vector};
use rapier2d::{dynamics::RigidBodyHandle, geometry::ColliderHandle, math::Rotation, prelude::{Collider, ColliderBuilder}};
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
    pub drag_offset: Option<Vec2>
}

impl Structure {

    pub fn resize(&mut self, space: &mut Space) {

        if !*self.get_selected() {
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

    pub fn tick_editor(&mut self, level: &mut Level, camera_rect: &Rect) {

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

    pub fn update_resize(&mut self) {
        if !*self.get_selected() {return}
    }

    pub fn rotate(&mut self, space: &mut Space) {
        if !*self.get_selected() {return}

        if !is_key_down(input::KeyCode::R) {return}

        let rigid_body = space.rigid_body_set.get_mut(*self.get_rigid_body_handle()).unwrap();
        
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

                    let body = space.rigid_body_set.get_mut(*self.get_rigid_body_handle()).unwrap();
                    
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

impl HasRigidBody for Structure {
    fn get_rigid_body_handle(&self) -> &RigidBodyHandle {
        &self.rigid_body_handle
    }
}
impl HasCollider for Structure {

    fn get_collider_handle(&self) -> &ColliderHandle {
        &self.collider_handle
    }
    fn get_drag_offset(&mut self) -> &mut Option<Vec2> {
        &mut self.drag_offset
    }
    
    fn get_selected(&mut self) -> &mut bool {
        &mut self.selected
    }

    fn get_dragging(&mut self) -> &mut bool {
        &mut self.dragging
    }
}

impl Color for Structure {
    fn color(&mut self) -> &mut macroquad::color::Color {
        &mut self.color
    }
}