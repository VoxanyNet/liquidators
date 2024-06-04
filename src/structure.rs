use diff::Diff;
use gamelibrary::{space::RigidBodyHandle, traits::{Color, HasRigidBody}};
use serde::{Serialize, Deserialize};

#[derive(Serialize, serde::Deserialize, Diff, PartialEq, Clone)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Structure {
    pub rigid_body_handle: RigidBodyHandle,
    pub color: gamelibrary::proxies::macroquad::color::Color
}

impl HasRigidBody for Structure {
    fn get_rigid_body_handle(&self) -> &RigidBodyHandle {
        &self.rigid_body_handle
    }
}

impl Color for Structure {
    fn color(&self) -> gamelibrary::proxies::macroquad::color::Color {
        self.color
    }
}