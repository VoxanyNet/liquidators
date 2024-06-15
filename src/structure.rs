use diff::Diff;
use gamelibrary::{menu::Menu, proxies::macroquad::{color::colors::{DARKGRAY, GREEN, RED}, math::vec2::Vec2}, space::RigidBodyHandle, traits::{Color, HasRigidBody}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, serde::Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Structure {
    pub rigid_body_handle: RigidBodyHandle,
    pub color: gamelibrary::proxies::macroquad::color::Color,
    pub menu: Option<Menu>
}

impl Structure {
    pub fn spawn_menu(&mut self, position: Vec2) {
        
        let mut menu = Menu::new(
            position,
            DARKGRAY
        );

        menu.add_button("Delete".to_string());
        menu.add_button("Green".to_string());
        menu.add_button("Red".to_string());

        self.menu = Some(menu);
    }

    pub fn handle_menu(mut self) -> Option<Self> {

        let menu = match self.menu.clone() {
            Some(menu) => menu,
            None => return Some(self),
        };

        for menu_item in menu.get_menu_items() {

            if !menu_item.clicked {
                continue;
            }

            match menu_item.text.as_str() {
                "Delete" => {
                    self.menu = None;
                    return None
                },
                "Green" => {
                    self.menu = None;
                    self.color = GREEN;
                    return Some(self);
                }
                "Red" => {
                    self.menu = None;
                    self.color = RED;
                    return Some(self);
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

impl Color for Structure {
    fn color(&self) -> gamelibrary::proxies::macroquad::color::Color {
        self.color
    }
}