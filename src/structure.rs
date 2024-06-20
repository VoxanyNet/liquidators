use diff::Diff;
use gamelibrary::{menu::Menu, proxies::macroquad::{color::colors::{DARKGRAY, GREEN, RED}, math::vec2::Vec2}, space::{RigidBodyHandle, Space}, traits::{Color, HasRigidBody}, translate_coordinates};
use macroquad::input::{self, is_key_down, is_mouse_button_released, mouse_position};
use serde::{Serialize, Deserialize};

use crate::level::Level;

#[derive(Serialize, serde::Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Structure {
    pub rigid_body_handle: RigidBodyHandle,
    pub color: gamelibrary::proxies::macroquad::color::Color,
    pub menu: Option<Menu>,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>
}

impl Structure {

    pub fn spawn_menu(&mut self, space: &mut Space) {
        
        if !is_mouse_button_released(input::MouseButton::Right) {
            return;
        }

        let mouse_pos = Vec2::new(mouse_position().0, mouse_position().1);

        // this should probaby be cached somewhere
        let intersections = space.query_point(translate_coordinates(&mouse_pos));

        if !intersections.contains(self.get_rigid_body_handle()) {
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

    pub fn tick_editor(&mut self, level: &mut Level) {

        match &mut self.menu {
            Some(menu) => menu.update(),
            None => {}
        }

        self.spawn_menu(&mut level.space);

        self.update_selected(&mut level.space);

        self.update_is_dragging(&mut level.space);

        self.update_drag(&mut level.space);

        self.rotate(&mut level.space);


    }

    pub fn resize(&mut self) {
        if !*self.get_selected() {return}
    }

    pub fn rotate(&mut self, space: &mut Space) {
        if !*self.get_selected() {return}

        if !is_key_down(input::KeyCode::R) {return}

        space.get_rigid_body_mut(self.get_rigid_body_handle()).unwrap().rotation -= 0.05
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
                    return None
                },
                "Zero Velocity" => {

                    let body = space.get_rigid_body_mut(&self.get_rigid_body_handle()).unwrap();
                    
                    body.velocity = Vec2::ZERO;
                    body.angular_velocity = 0.;

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
    fn color(&mut self) -> &mut gamelibrary::proxies::macroquad::color::Color {
        &mut self.color
    }
}