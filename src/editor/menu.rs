use gamelibrary::proxies::macroquad::{color::Color, math::{rect::Rect, vec2::Vec2}};
use macroquad::{color::{BLACK, WHITE}, input::{self, mouse_position}};

pub struct Menu {
    items: Vec<MenuItem>,
    position: Vec2,
    color: Color
}

impl Menu {

    pub fn new(position: Vec2, color: Color) -> Self {
        Self {
            items: vec![],
            position: position,
            color: color,
        }
    }

    pub fn update(&mut self) {
        for menu_item in &mut self.items {
            menu_item.update()
        }
    }

    pub fn add_button(&mut self, text: String) {

        self.items.push(
            MenuItem { 
                rect: Rect { 
                    x: self.position.x, 
                    y: self.position.y + (20. * self.items.len() as f32), 
                    w: 100., 
                    h: 20. 
                }, 
                text: text, 
                hovered: false, 
                clicked: false, 
                color: self.color
            }
        )
    }

    pub async fn draw(&self) {
        for item in &self.items {
            item.draw().await;
        }
    }
}

struct MenuItem {
    rect: Rect,
    text: String,
    hovered: bool,
    clicked: bool,
    color: Color
}

impl MenuItem {
    async fn draw(&self) {

        let color = match self.hovered {
            true => WHITE,
            false => self.color.into()
        };
        
        macroquad::shapes::draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, color);
        macroquad::shapes::draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 3., BLACK);
        macroquad::text::draw_text(&self.text, self.rect.x, self.rect.y + self.rect.h / 2., 14., WHITE);
    }

    fn update(&mut self) {

        let mouse_position = mouse_position();

        self.hovered = false;
        self.clicked = false;

        if self.rect.contains(
            Vec2::new(mouse_position.0, mouse_position.1)
        ) {

            self.hovered = true;

            if input::is_mouse_button_pressed(input::MouseButton::Left) != true {
                self.clicked = true;
            }
        }
    }
}


