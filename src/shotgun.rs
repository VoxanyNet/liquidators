use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

use crate::{game_state::GameState, TickContext};

pub struct Shotgun {
    pub collider: ColliderHandle,
    pub rigid_body: RigidBodyHandle,
    pub sprite: String
}

impl Shotgun {
    async fn draw(&self, game_state: &mut GameState, ctx: &TickContext<'_>, ) {
        let rigid_body = game_state.level.space.rigid_body_set.get(self.rigid_body).unwrap();

    }
}