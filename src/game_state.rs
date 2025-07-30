
use std::collections::HashSet;

use diff::Diff;
use gamelibrary::{arenaiter::SyncArenaIterator, font_loader::FontLoader, log, rapier_mouse_world_pos, sync_arena::Index, texture_loader::TextureLoader, time::Time, traits::HasPhysics};
use macroquad::{camera::Camera2D, input::is_key_released, math::{Rect, Vec2}};
use serde::{Deserialize, Serialize};

use crate::{chat::Chat, enemy::Enemy, events::{self, Event}, level::Level, player::player::Player, structure::Structure, TickContext};

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub enum Mode {
    Deathmatch,
    Sandbox,
    WaveSurvival(WaveSurvivalData)
}

#[derive(Serialize, Deserialize, Diff, Clone, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct  WaveSurvivalData {
    
    pub wave: u32,
    pub last_wave_end: Time,
    pub ready: HashSet<Index>,
    pub wave_active: bool,
    pub enemy_reserve: u32, // the total number of remaining enemies that will spawn this wave
    pub batch_spawn_rate: u32, // the number of ms to wait between wave batches
    pub batch_size: u32, // the number of enemies that will spawn in each batch,
    pub last_batch_spawn: Time, 
}

impl WaveSurvivalData {

    /// Create new wave survival data starting at wave 1
    pub fn new() -> Self {
        Self {
            wave: 1,
            last_wave_end: Time::new(0),
            ready: HashSet::new(),
            wave_active: false,
            enemy_reserve: 10,
            batch_spawn_rate: 5000,
            batch_size: 1,
            last_batch_spawn: Time::new(0),
        }
    }
}

pub struct DeathmatchData {
    
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
            mode: Mode::Deathmatch,
        }
    }

    pub async fn sync_sounds(&mut self, ctx: &mut TickContext<'_>) {
        self.level.sync_sounds(ctx).await
    }

    pub fn spawn_brick(&mut self, ctx: &mut TickContext) {
        if is_key_released(macroquad::input::KeyCode::E) {

            let pos = rapier_mouse_world_pos(ctx.camera_rect);

            let new_structure = Structure::new(pos, &mut self.level.space, ctx.uuid.clone());
            
            self.level.structures.insert(new_structure);
        }
    }   

    pub fn deathmatch_tick(state: &mut GameState, ctx: &mut TickContext) {
        
        if !*ctx.is_host {
            return;
        }

        if let Mode::Deathmatch = state.mode {
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
        else {
            return;
        }

    }


    pub fn wave_survival_tick(&mut self, ctx: &mut TickContext) {

        if !*ctx.is_host {
            return;
        }
        if let Mode::WaveSurvival(data) = &mut self.mode {

            // start next wave logic
            if data.wave_active == false {
                let mut ready = true;

                for (player_index, _) in &self.level.players {
                    if !data.ready.contains(&player_index) {
                        ready = false
                    }
                }

                if ready {
                    data.wave += 1;

                    data.wave_active = true;
                }
            }

            if data.wave_active == false {
                return;
            }

            for i in 0..data.batch_size {

                let spawn_location = Vec2::new(-500. + (i as f32 * 60.) , 0.);

                self.level.enemies.insert(
                    Enemy::new(spawn_location, ctx.uuid.clone(), &mut self.level.space, ctx.textures)
                );
            }
        }
    }

    pub fn server_tick(&mut self) {
        self.level.server_tick();
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

    pub async fn draw(&self, textures: &mut TextureLoader, camera_rect: &Rect, fonts: &mut FontLoader, camera: &Camera2D) {

        self.level.draw(textures, camera_rect, fonts, camera).await;
    }

    pub async fn draw_hud(&self, ctx: &mut TickContext<'_>) {
        self.level.draw_hud(ctx).await;
    }

}