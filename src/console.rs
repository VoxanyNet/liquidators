use std::fmt::Display;

use macroquad::{color::colors, input::{is_mouse_button_down, mouse_position}, math::{Rect, Vec2}, shapes::draw_rectangle, text::draw_text};

pub struct Console {
    pub enabled: bool,
    pub messages: Vec<String>,
    pub rect: Rect,
    move_offset: Option<Vec2>
}

impl Console {
    pub fn new() -> Self {
        Console {
            enabled: false,
            messages: vec![],
            rect: Rect::new(0., 0., 200., 150.),
            move_offset: None,
        }
    }

    pub fn tick(&mut self) {
        self.move_window();
    }

    pub fn log<T: Display>(&mut self, message: T) {
        self.messages.push(message.to_string());
    }

    pub fn move_window(&mut self) {
        if !self.rect.contains(mouse_position().into()) && self.move_offset.is_none() {
            return
        }

        if !self.enabled {
            return
        }

        if !is_mouse_button_down(macroquad::input::MouseButton::Left) {
            self.move_offset = None;
            return
        }  

        // only set move offset when first starting moving window
        let move_offset = match self.move_offset {
            Some(move_offset) => move_offset,
            None => {

                let new_move_offset = Vec2::new(
                    self.rect.x - mouse_position().0,
                    self.rect.y - mouse_position().1
                );

                self.move_offset = Some(
                    new_move_offset
                );

                new_move_offset
            }
        };

        self.rect.x = mouse_position().0 + move_offset.x;
        self.rect.y = mouse_position().1 + move_offset.y;
    
    }

    pub async fn draw(&mut self) {
        if !self.enabled {
            return
        }

        let mut console_color = colors::GRAY;

        console_color.a = 0.2;

        draw_rectangle(self.rect.x, self.rect.y, self.rect.w, self.rect.h, console_color);

        // origin probably isnt the best label for this
        let origin = Vec2::new(
            self.rect.x,
            self.rect.y + self.rect.h
        );

        // iterate through messages in reverse order
        for (index, message) in self.messages.iter().rev().enumerate() {
            draw_text(
                &message, 
                origin.x + 2., 
                origin.y - (index as f32 * 30.) - 2., // newest messages are draw at the bottom. we start at 10 pixels above bottom of screen
                30., 
                colors::WHITE
            );
        }
    }
}