
use diff::Diff;
use gamelibrary::{rapier_mouse_world_pos, space::{Space, SyncColliderHandle, SyncRigidBodyHandle}, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::{input::{self, is_mouse_button_pressed}, math::{Rect, Vec2}};
use nalgebra::vector;
use rapier2d::prelude::{ActiveEvents, ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::TickContext;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Brick {
    //sounds: Vec<ears::Sound>,
    collider: SyncColliderHandle,
    body: SyncRigidBodyHandle,
    selected: bool,
    dragging: bool,
    drag_offset: Option<Vec2>,
    texture_path: String,
    pub owner: Option<String>,
    editor_owner: Option<String>,
    previous_velocity: Vec2 // try to change this to the native rapier type
}

impl Brick {
    pub fn new(space: &mut Space, location: Vec2, owner: Option<String>) -> Self {

        let body_handle = space.sync_rigid_body_set.insert_sync( 
            RigidBodyBuilder::dynamic()
                .position(vector![location.x, location.y].into())
                .ccd_enabled(true)
                .build()
        );

        let collider_handle = space.sync_collider_set.insert_with_parent_sync(
            ColliderBuilder::cuboid(8., 3.)
            .active_events(ActiveEvents::COLLISION_EVENTS)
            .build(), 
            body_handle, 
            &mut space.sync_rigid_body_set
        );


        Self {
            collider: collider_handle,
            body: body_handle,
            selected: false,
            dragging: false,
            texture_path: "assets/structure/brick.png".to_string(),
            drag_offset: None,
            editor_owner: None,
            owner,
            previous_velocity: Vec2::ZERO,
            //sounds: vec![]
        }
    }

    pub fn tick(&mut self, ctx: &mut TickContext) {

        ctx.owned_rigid_bodies.push(*self.rigid_body_handle());
        ctx.owned_colliders.push(*self.collider_handle());
        
    }

    pub fn update_editor_owner(&mut self, editor_uuid: &String, space: &mut Space, camera_rect: &Rect) {
        // transfer ownership to whoever clicks this structure

        if !is_mouse_button_pressed(input::MouseButton::Left) {
            return;
        }

        if !self.contains_point(space, rapier_mouse_world_pos(camera_rect)) {
            return;
        }

        self.editor_owner = Some(editor_uuid.clone());
    }

    pub fn editor_tick(&mut self, editor_uuid: &String, space: &mut Space, camera_rect: &Rect) {
        self.update_editor_owner(editor_uuid, space, camera_rect);
        self.editor_rotate(space);
        self.editor_resize(space);
        self.update_selected(space, camera_rect);
        self.update_drag(space, camera_rect);
        self.update_is_dragging(space, camera_rect);

    }

    pub async fn editor_draw(&self, space: &Space, textures: &mut TextureLoader) {

        if self.selected {
            self.draw_outline(space, 3.);
        }
        
        self.draw_texture(space, &self.texture_path, textures, false, false, 0.).await;
        

    } 

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {
        self.draw_texture(space, &self.texture_path, textures, false, false, 0.).await;
    }
}

impl HasPhysics for Brick {
    fn collider_handle(&self) -> &SyncColliderHandle {
        &self.collider
    }

    fn rigid_body_handle(&self) -> &SyncRigidBodyHandle{
        &self.body
    }

    fn selected(&self) -> &bool {
        &self.selected
    }

    fn selected_mut(&mut self) -> &mut bool {
        &mut self.selected
    }

    fn dragging(&mut self) -> &mut bool {
        &mut self.dragging
    }

    fn drag_offset(&mut self) -> &mut Option<macroquad::prelude::Vec2> {
        &mut self.drag_offset
    }
}
