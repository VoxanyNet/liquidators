
use diff::Diff;
use gamelibrary::texture_loader::TextureLoader;
use macroquad::input::is_key_released;
use serde::{Deserialize, Serialize};

use crate::{chat::Chat, level::Level, TickContext};

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub level: Level,
    pub game_started: bool,
    pub chat: Chat
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            level: Level::empty(),
            game_started: false,
            chat: Chat::new()
        }
    }

    pub fn tick(
        &mut self,
        ctx: &mut TickContext
    ) { 

        self.level.tick(ctx);

        if is_key_released(macroquad::input::KeyCode::Backspace) {
            self.chat.add_message("Test".to_string(), "Super cool!".to_string())
        }

    }

    pub async fn draw(&self, textures: &mut TextureLoader) {

        self.level.draw(textures).await;

        self.chat.draw().await;
    }

}