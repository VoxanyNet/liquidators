use gamelibrary::menu::Menu;
use gamelibrary::space::Space;
use gamelibrary::traits::HasPhysics;
use diff::Diff;
use macroquad::color::DARKGRAY;
use macroquad::input::{is_key_down, KeyCode};
use macroquad::math::{Rect, Vec2};
use macroquad::window;
use nalgebra::vector;
use rapier2d::dynamics::{RigidBodyBuilder, RigidBodyHandle, RigidBodyType};
use rapier2d::geometry::{ColliderBuilder, ColliderHandle};
use serde::{Deserialize, Serialize};

use crate::game_state::GameState;
use crate::TickContext;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct PhysicsSquare {
    pub scale: u32,
    pub color: macroquad::color::Color,
    pub owner: String,
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub controllable: bool,
    pub menu: Option<Menu>,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub resize_handles: [Rect; 4]
}

impl PhysicsSquare {
    pub fn new(space: &mut Space, position: Vec2, body_type: RigidBodyType, hx: f32, hy: f32, owner: &String, controllable: bool, color: macroquad::color::Color) -> Self {

        
        let rigid_body_handle = space.rigid_body_set.insert(
            RigidBodyBuilder::new(body_type)
                .translation(vector![position.x, position.y])
                .build()
        );
        
        
        
        let collider_handle = space.collider_set.insert_with_parent(
            ColliderBuilder::cuboid(hx, hy).build(),
            rigid_body_handle, 
            &mut space.rigid_body_set
        );

        Self {
            scale: 1,
            color: color,
            owner: owner.clone(),
            rigid_body_handle,
            collider_handle,
            controllable: controllable,
            menu: None,
            selected: false,
            dragging: false,
            drag_offset: None,
            resize_handles: [Rect::new(0., 0., 0., 0.); 4]
        }
        
    }

    pub fn get_menu(&mut self) -> &mut Option<Menu> {
        &mut self.menu
    }

    pub fn spawn_menu(&mut self, position: Vec2) {

        let mut menu = Menu::new(
            position,
            DARKGRAY
        );

        menu.add_button("Delete".to_string());

        self.menu = Some(menu);
    }

    pub fn handle_menu(self) -> Option<Self> {
        for menu_item in self.clone().menu.unwrap().get_menu_items() {

            if menu_item.clicked && menu_item.hovered {
                continue;
            }

            match menu_item.text.as_str() {
                "Delete" => return None,
                _ => return Some(self)
            };

        };  

        // this is the result if the menu doesnt have any items or none of the items are hovered and clicked
        Some(self)
    }

    pub fn tick(&mut self, game_state: &mut GameState, ctx: &mut TickContext) {

        let rigid_body = game_state.level.space.rigid_body_set.get_mut(self.rigid_body_handle).expect("shit");

        if rigid_body.position().translation.x >= window::screen_width() || rigid_body.position().translation.x <= 0. {
            rigid_body.set_linvel(
                vector![rigid_body.linvel().x * -1., rigid_body.linvel().y], 
                true
            )
        }

        if rigid_body.position().translation.y >= window::screen_height() || rigid_body.position().translation.y <= 0. {
            rigid_body.set_linvel(
                vector![rigid_body.linvel().x, rigid_body.linvel().y * -1.], 
                true
            )
        }

        if self.controllable {

            if is_key_down(KeyCode::W) {

                // stop any y axis movement if going down
                if rigid_body.linvel().y.is_sign_negative() {
                    rigid_body.set_linvel(
                        vector![rigid_body.linvel().x, 0.],
                        true
                    )
                }

                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x, rigid_body.linvel().y + 4.],
                    true
                )
            }

            if is_key_down(KeyCode::S) {

                if rigid_body.linvel().y.is_sign_positive() {
                    rigid_body.set_linvel(
                        vector![rigid_body.linvel().x, 0.],
                        true
                    )
                }

                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x, rigid_body.linvel().y - 4.],
                    true
                )
            }
            
            if is_key_down(KeyCode::A) {

                if rigid_body.linvel().x.is_sign_positive() {
                    rigid_body.set_linvel(
                        vector![0., rigid_body.linvel().y],
                        true
                    )
                }

                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x - 4., rigid_body.linvel().y],
                    true
                )
            }

            if is_key_down(KeyCode::D) {

                if rigid_body.linvel().x.is_sign_negative() {
                    rigid_body.set_linvel(
                        vector![0., rigid_body.linvel().y],
                        true
                    )
                }

                rigid_body.set_linvel(
                    vector![rigid_body.linvel().x + 4., rigid_body.linvel().y],
                    true
                )
            }

        }
    }
}

impl HasPhysics for PhysicsSquare {

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
