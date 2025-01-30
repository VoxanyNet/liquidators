use diff::Diff;
use gamelibrary::{space::Space, texture_loader::TextureLoader, traits::{draw_texture_onto_physics_body, HasPhysics}};
use macroquad::math::Vec2;
use parry2d::math::{Isometry, Point};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, ImpulseJointHandle, RevoluteJointBuilder, RigidBodyBuilder, RigidBodyHandle};
use parry2d::math::Real;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Diff, PartialEq)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Arm {
    rigid_body_handle: RigidBodyHandle,
    collider_handle: ColliderHandle,
    joint_handle: ImpulseJointHandle,
    sprite_path: String,
    sprite_scale: f32
}

impl Arm {
    pub fn new(
        space: &mut Space, 
        body_rigid_body_handle: RigidBodyHandle,
        body_anchor_point: Point<Real>, 
        arm_anchor_point: Point<Real>,
        initial_pos: Isometry<Real>, 
        textures: &mut TextureLoader,
        sprite_path: String,
        sprite_scale: f32
    ) -> Self {

        let rigid_body = RigidBodyBuilder::dynamic()
            .position(initial_pos)
            .build();

        let texture = futures::executor::block_on(textures.get(&sprite_path));

        // use the texture height and width for the collider for now, might want to change this later
        let collider = ColliderBuilder::cuboid(
            texture.width() / 2., 
            texture.height() / 2.
        );

        let arm_rigid_body_handle = space.rigid_body_set.insert(rigid_body);
        let collider_handle = space.collider_set.insert_with_parent(collider, arm_rigid_body_handle, &mut space.rigid_body_set);

        let joint = RevoluteJointBuilder::new()
            .local_anchor1(body_anchor_point)
            .local_anchor2(arm_anchor_point)
            .build();

        let joint_handle = space.impulse_joint_set.insert(body_rigid_body_handle, arm_rigid_body_handle, joint, true);

        Self {
            rigid_body_handle: arm_rigid_body_handle,
            collider_handle,
            joint_handle,
            sprite_path,
            sprite_scale,
        }

    }


    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {
        draw_texture_onto_physics_body(
            self.rigid_body_handle, 
            self.collider_handle, 
            space, 
            &self.sprite_path, 
            textures, 
            false, 
            false, 
            0.
        ).await;
    }
}