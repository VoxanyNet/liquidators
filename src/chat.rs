use diff::Diff;
use macroquad::{color::WHITE, math::Vec2, text::draw_text, window::screen_height};
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Chat {
    pub messages: Vec<Message>
}

impl Chat {
    pub fn add_message(&mut self, author: String, content: String) {
        self.messages.push(
            Message {
                author,
                content,
            }
        )
    }

    pub fn new() -> Self {
        Self { messages: Vec::new() }
    }

    pub async fn draw(&self) {

        // origin probably isnt the best label for this
        let origin = Vec2::new(
            0.,
            screen_height()
        );

        // iterate through messages in reverse order
        for (index, message) in self.messages.iter().rev().enumerate() {
            draw_text(
                &message.content, 
                origin.x + 30., 
                (origin.y - (index as f32 * 30.)) - 30., // newest messages are draw at the bottom. we start at 10 pixels above bottom of screen
                30., 
                WHITE
            )
        }
    }
}

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Message {
    pub author: String,
    pub content: String
}