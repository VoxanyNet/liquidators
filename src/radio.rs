use diff::Diff;
use gamelibrary::{space::Space, texture_loader::TextureLoader, traits::HasPhysics};
use macroquad::math::{Rect, Vec2};
use nalgebra::vector;
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, RigidBodyBuilder, RigidBodyHandle};
use serde::{Deserialize, Serialize};

use crate::level::Level;

#[derive(Serialize, Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Radio {
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
    pub selected: bool,
    pub texture_path: String,
    pub dragging: bool,
    pub drag_offset: Option<Vec2>,
    pub owner: Option<String>,
    pub editor_owner: Option<String>
}

impl Radio {

    pub async fn draw(&self, textures: &mut TextureLoader, space: &Space) {
        self.draw_texture(space, &self.texture_path.clone(), textures, false, false).await;
    }

    pub fn tick_editor(&mut self, level: &mut Level, camera_rect: &Rect, client_uuid: &String) {


        if self.editor_owner.as_ref().unwrap() == client_uuid {
            self.update_selected(&mut level.space, camera_rect);

        }
        
    }
    
}

impl HasPhysics for Radio {
    fn collider_handle(&self) -> &ColliderHandle {
        &self.collider_handle
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

    fn drag_offset(&mut self) -> &mut Option<Vec2> {
        todo!()
    }

    fn rigid_body_handle(&self) -> &RigidBodyHandle {
        &self.rigid_body_handle
    }
}

pub struct RadioBuilder {
    rigid_body_handle: Option<RigidBodyHandle>,
    collider_handle: Option<ColliderHandle>,
    selected: Option<bool>,
    dragging: Option<bool>,
    drag_offset: Option<Option<Vec2>>,
    owner: Option<Option<String>>,
    editor_owner: Option<Option<String>>,
    texture_path: Option<String>
}

impl RadioBuilder {
    pub fn new(space: &mut Space, position: &Vec2) -> Self {

        let rigid_body = RigidBodyBuilder::dynamic()
            .position(vector![position.x, position.y].into())
            .build();

        let collider = ColliderBuilder::cuboid(8. * 2., 7. * 2.)
            .mass(10.)
            .restitution(0.)
            .build();

        let rigid_body_handle = space.rigid_body_set.insert(rigid_body);
        let collider_handle = space.collider_set.insert_with_parent(collider, rigid_body_handle, &mut space.rigid_body_set);

        Self {
            rigid_body_handle: Some(rigid_body_handle),
            collider_handle: Some(collider_handle),
            selected: None,
            dragging: None,
            drag_offset: None,
            owner: None,
            editor_owner: None,
            texture_path: None
        }
    }

    pub fn rigid_body_handle(mut self, rigid_body_handle: RigidBodyHandle) -> Self {
        self.rigid_body_handle = Some(rigid_body_handle);
        self
    }

    pub fn collider_handle(mut self, collider_handle: ColliderHandle) -> Self {
        self.collider_handle = Some(collider_handle);
        self
    }

    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = Some(selected);
        self
    }

    pub fn dragging(mut self, dragging: bool) -> Self {
        self.dragging = Some(dragging);
        self
    }
    
    pub fn drag_offset(mut self, drag_offset: Vec2) -> Self {
        self.drag_offset = Some(Some(drag_offset));
        self
    }

    pub fn owner(mut self, owner: String) -> Self {
        self.owner = Some(Some(owner));
        self
    }

    pub fn editor_owner(mut self, editor_owner: String) -> Self {
        self.editor_owner = Some(Some(editor_owner));
        self
    }

    pub fn texture_path(mut self, texture_path: String) -> Self {
        self.texture_path = Some(texture_path);
        self
    }

    pub fn build(self) -> Radio {

        let rigid_body_handle = match self.rigid_body_handle {
            Some(rigid_body_handle) => {
                rigid_body_handle
            },
            None => todo!()
        }; 

        let collider_handle = match self.collider_handle {
            Some(collider_handle) => {
                collider_handle
            },
            None => todo!()
        };

        let selected = match self.selected {
            Some(selected) => {
                selected
            },
            None => false
        };

        let dragging = match self.dragging {
            Some(dragging) => {
                dragging
            },
            None => false
        };

        let drag_offset = match self.drag_offset {
            Some(drag_offset) => {
                drag_offset
            },
            None => Some(Vec2::ZERO)
        };

        let owner: Option<String> = match self.owner {
            Some(owner) => owner,
            None => None,
        };

        let editor_owner = match self.editor_owner {
            Some(editor_owner) => editor_owner,
            None => None,
        };

        let texture_path = match self.texture_path {
            Some(texture_path) => texture_path,
            None => "assets/structure/radio_on.png".to_string()
        };


        Radio {
            rigid_body_handle,
            collider_handle,
            selected,
            dragging,
            drag_offset,
            owner,
            editor_owner,
            texture_path
        }
        
    }
}