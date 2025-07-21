
use diff::Diff;
use gamelibrary::{arenaiter::SyncArenaIterator, font_loader::FontLoader, log, rapier_mouse_world_pos, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{camera::Camera2D, input::is_key_released, math::{Rect, Vec2}};
use serde::{Deserialize, Serialize};

use crate::{chat::Chat, level::Level, player::player::Player, structure::Structure, TickContext};


// THIS IS A GOOD IDEA, JUST FIGURE OUT THE TYPE STUFF
// pub trait Gamemode {
//     fn tick(state: &mut GameState, ctx: &mut TickContext);
// }

// #[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
// #[diff(attr(
//     #[derive(Serialize, Deserialize)]
// ))]
// pub struct Deathmatch {
//     game_start: bool,
//     living_players: u16
// }

// impl Deathmatch {
//     pub fn determine_living_players(&mut self, state: &mut GameState) {

//         let mut living_players: u32 = 0;
//         for (_, player) in &state.level.players {
//             if player.health > 0 {
//                 living_players += 1;
//             }
//         }
//     }
// }

// impl Gamemode for Deathmatch {
//     fn tick(state: &mut GameState, ctx: &mut TickContext) {
        
//     }
// }

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq, Debug)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum Mode {
    Deathmatch,
    Sandbox
}

impl Mode {

    // these methods dont really need to be in this enum
    pub fn tick(state: &mut GameState, ctx: &mut TickContext) {

        log(&format!("mode: {:?}", state.mode));
        match state.mode {
            Mode::Deathmatch => {

                log(&format!("matched deathmatch"));
                Mode::deathmatch_tick(state, ctx);
            },
            Mode::Sandbox => {

            },
        }
    }

    pub fn deathmatch_tick(state: &mut GameState, ctx: &mut TickContext) {
        
        // only the host should manage gamemode stuff
        if !*ctx.is_host {
            return;
        }

        log(&format!("DEATHMATCH TICKING: {}", ctx.is_host));

        // start the game if more than one player is connected
        if state.level.players.len() > 1 {
            state.game_started = true;
        }

        if state.level.players.len() < 2 {
            state.game_started = false
        }

        if !state.game_started {
            return;
        }

        let mut living_player_count: u32 = 0;

        for (_, player) in &state.level.players {
            if player.health > 0 {
                living_player_count += 1;

            }
        }

        if living_player_count < 2 {

            let mut players = SyncArenaIterator::new(&mut state.level.players);

            while let Some((player, arena)) = players.next() {

                let owner = player.owner.clone();
                let position = {
                    state.level.space.sync_rigid_body_set.get_sync(*player.rigid_body_handle()).unwrap().position().translation.clone()
                };

                // respawn the player with the same position and owner
                Player::spawn(arena, &mut state.level.space, owner, &Vec2::new(position.x, position.y), ctx.textures);

                player.despawn(&mut state.level.space);
            }
        }
    }
}

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct GameState {
    pub level: Level,
    pub game_started: bool,
    pub chat: Chat,
    pub living_players: u32,
    pub mode: Mode
}

impl GameState {

    pub fn empty() -> Self {
        Self {
            level: Level::empty(),
            game_started: false,
            chat: Chat::new(),
            living_players: 0,
            mode: Mode::Deathmatch
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

        log(&format!("is host: {}", ctx.is_host));

        Mode::tick(self, ctx);

        self.level.tick(ctx);

        self.spawn_brick(ctx);

        

        if is_key_released(macroquad::input::KeyCode::Backspace) {
            self.chat.add_message("Gamer".to_string(), "Test message".to_string());
        }

    }

    pub async fn draw(&self, textures: &mut TextureLoader, camera_rect: &Rect, fonts: &mut FontLoader, camera: &Camera2D) {

        self.level.draw(textures, camera_rect, fonts, camera).await;
    }

    pub async fn draw_hud(&self, ctx: &mut TickContext<'_>) {
        self.level.draw_hud(ctx).await;
    }

}