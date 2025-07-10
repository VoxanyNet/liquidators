
use diff::Diff;
use gamelibrary::{font_loader::FontLoader, rapier_mouse_world_pos, texture_loader::TextureLoader};
use macroquad::{input::is_key_released, math::Rect};
use serde::{Deserialize, Serialize};

use crate::{chat::Chat, level::Level, structure::Structure, TickContext};

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub level: Level,
    pub game_started: bool,
    pub chat: Chat,
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            level: Level::empty(),
            game_started: false,
            chat: Chat::new()
        }
    }

    pub async fn sync_sounds(&mut self, ctx: &mut TickContext<'_>) {
        self.level.sync_sounds(ctx).await
    }

    pub fn spawn_brick(&mut self, ctx: &mut TickContext) {
        if is_key_released(macroquad::input::KeyCode::E) {

            let pos = rapier_mouse_world_pos(ctx.camera_rect);

            let new_structure = Structure::new(pos, &mut self.level.space, ctx.uuid.clone());
            
            self.level.structures.push(new_structure);
        }
    }

    pub fn tick(
        &mut self,
        ctx: &mut TickContext
    ) { 

        self.level.tick(ctx);

        self.spawn_brick(ctx);

        if is_key_released(macroquad::input::KeyCode::Backspace) {
            self.chat.add_message("Gamer".to_string(), "Test message".to_string());
        }

    }

    pub async fn draw(&self, textures: &mut TextureLoader, camera_rect: &Rect, fonts: &mut FontLoader) {

        self.level.draw(textures, camera_rect, fonts).await;
    }

}