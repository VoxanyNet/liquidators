use gamelibrary::traits::HasCollider;
use macroquad::math::Vec2;
use rapier2d::prelude::{ColliderHandle, RigidBodyHandle};

pub struct Radio {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub selected: bool,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub owner: Option<String>,
    pub editor_owner: String
}

impl HasCollider for Radio {
    fn get_collider_handle(&self) -> &ColliderHandle {
        &self.collider_handle
    }

    fn get_selected(&mut self) -> &mut bool {
        &mut self.selected
    }

    fn get_dragging(&mut self) -> &mut bool {
        &mut self.dragging
    }

    fn get_drag_offset(&mut self) -> &mut Option<Vec2> {
        todo!()
    }
}