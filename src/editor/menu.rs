use gamelibrary::proxies::macroquad::math::{rect::Rect, vec2::Vec2};
use macroquad::input::{self, mouse_position};

pub struct Menu {
    pub items: Vec<MenuItem>,
    pub position: Vec2
}

pub struct MenuItem {
    rect: Rect,
    text: String,
    hovered: bool,
    clicked: bool
}

// impl MenuItem {
//     fn new() -> Self {
//         MenuItem { 
//             rect: Rect::new(x, y, w, h), 
//             text: (), 
//             hovered: (), 
//             clicked: () 
//         }
//     }
// }

impl Menu {
    fn update(&mut self) {

        let mouse_position = mouse_position();

        for item in &mut self.items {

            if item.rect.contains(
                Vec2::new(mouse_position.0, mouse_position.1)
            ) {

                item.hovered = true;

                if input::is_mouse_button_pressed(input::MouseButton::Left) != true {
                    item.clicked = true;
                }
            }
        };

    }

    fn draw(&self) {
        for item in &self.items {

        }
    }
}