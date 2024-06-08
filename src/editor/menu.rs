use gamelibrary::proxies::macroquad::{color::Color, math::{rect::Rect, vec2::Vec2}};
use macroquad::{color::{BLACK, RED, WHITE}, input::{self, is_mouse_button_down, is_mouse_button_pressed, mouse_position}, shapes::draw_rectangle_lines};

pub struct Menu {
    items: Vec<MenuItem>,
    position: Vec2,
    pub color: Color,
    pub containing_rect: Rect
}

impl Menu {

    pub fn new(position: Vec2, color: Color) -> Self {
        Self {
            items: vec![],
            position: position,
            color: color,
            containing_rect: Rect::new(position.x, position.y, 0., 0.)
        }
    }

    pub fn update(&mut self) {

        // reset containing rect because the menu items can change
        self.containing_rect = Rect::new(self.position.x, self.position.y, 0., 0.);

        for menu_item in &mut self.items {
            menu_item.update();

            self.containing_rect = self.containing_rect.combine_with(menu_item.rect);
        }

        let mouse_position = mouse_position();

    }

    pub fn add_button(&mut self, text: String) {

        self.items.push(
            MenuItem { 
                rect: Rect { 
                    x: self.position.x, 
                    y: self.position.y + (30. * self.items.len() as f32), 
                    w: 150., 
                    h: 30. 
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

        draw_rectangle_lines(self.containing_rect.x, self.containing_rect.y, self.containing_rect.w, self.containing_rect.h, 3., RED);

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

        let (rect_color, font_color) = match self.hovered {
            true => (WHITE, BLACK),
            false => (self.color.into(), WHITE)
        };

        
        macroquad::shapes::draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, rect_color);
        macroquad::shapes::draw_rectangle_lines(self.rect.x, self.rect.y, self.rect.w, self.rect.h, 3., BLACK);
        macroquad::text::draw_text(&self.text, self.rect.x + 3., self.rect.y + self.rect.h / 2., 20., font_color);
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


