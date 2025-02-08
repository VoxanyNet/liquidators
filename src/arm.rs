use diff::Diff;
use gamelibrary::{collider_top_left_pos, draw_texture_rapier, rapier_to_macroquad, space::Space, texture_loader::TextureLoader, traits::{draw_hitbox, draw_texture_onto_physics_body, HasPhysics}};
use macroquad::{color::{GREEN, RED, WHITE}, math::Vec2, shapes::draw_circle, texture::DrawTextureParams};
use nalgebra::vector;
use parry2d::math::{Isometry, Point};
use rapier2d::prelude::{ColliderBuilder, ColliderHandle, ImpulseJointHandle, InteractionGroups, RevoluteJointBuilder, RigidBodyBuilder, RigidBodyHandle};
use parry2d::math::Real;
use serde::{Deserialize, Serialize};

use crate::TickContext;

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
        mut arm_anchor_point: Point<Real>,
        textures: &mut TextureLoader,
        sprite_path: String,
        sprite_scale: f32
    ) -> Self {

        // doesnt need an initial position because the position is determined by the joint
        let rigid_body = RigidBodyBuilder::dynamic()
            .build();

        arm_anchor_point.x *= sprite_scale;
        arm_anchor_point.y *= sprite_scale;

        let texture = futures::executor::block_on(textures.get(&sprite_path));

        // use the texture height and width for the collider for now, might want to change this later
        let collider = ColliderBuilder::cuboid(
            (texture.width() / 2.) * sprite_scale, 
            (texture.height() / 2.) * sprite_scale
        )
            .mass(1.)
            .collision_groups(InteractionGroups::none())
            .build();

        let arm_rigid_body_handle = space.rigid_body_set.insert(rigid_body);
        let collider_handle = space.collider_set.insert_with_parent(collider, arm_rigid_body_handle, &mut space.rigid_body_set);

        let arm_anchor_point = {
            let mut top_left = collider_top_left_pos(space, collider_handle);

            top_left.x -= 8.;
            

            top_left
            
        };
        let joint = RevoluteJointBuilder::new()
            .local_anchor1(body_anchor_point)
            .local_anchor2(vector![arm_anchor_point.x, arm_anchor_point.y].into())
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

    pub fn get_joint_handle(&mut self) -> ImpulseJointHandle {
        self.joint_handle
    }
    pub fn tick(&mut self, ctx: &mut TickContext) {
        ctx.owned_rigid_bodies.push(self.rigid_body_handle);
        ctx.owned_colliders.push(self.collider_handle);
    }

    pub fn get_rigid_body_handle(&self) -> RigidBodyHandle {
        self.rigid_body_handle
    }

    pub async fn draw(&self, space: &Space, textures: &mut TextureLoader) {

        let body = space.rigid_body_set.get(self.rigid_body_handle).unwrap();
        let collider = space.collider_set.get(self.collider_handle).unwrap();
        let shape = collider.shape().as_cuboid().unwrap();

        let mut draw_params = DrawTextureParams::default();
        
        // this is dumb because we are getting the texture from the cache twice here but here we are
        let texture = futures::executor::block_on(textures.get(&self.sprite_path));

        draw_params.dest_size = Some(
            Vec2::new(texture.width() * self.sprite_scale, texture.height() * self.sprite_scale)
        );

        draw_params.rotation = body.rotation().angle() * -1.;

        let pos = body.translation();
        
        //draw_hitbox(space, self.rigid_body_handle, self.collider_handle);

        draw_texture_rapier(
            texture, 
            pos.x - shape.half_extents.x, 
            pos.y + shape.half_extents.y, 
            WHITE, 
            draw_params
        );

        let joint = space.impulse_joint_set.get(self.joint_handle).unwrap();

        let body_pos = space.rigid_body_set.get(joint.body1).unwrap().translation();
        let arm_pos = space.rigid_body_set.get(self.get_rigid_body_handle()).unwrap().translation();

        let body_pos_macroquad = rapier_to_macroquad(&Vec2::new(body_pos.x, body_pos.y));
        let arm_pos_macroquad = rapier_to_macroquad(&Vec2::new(arm_pos.x, arm_pos.y));

        let body_anchor = joint.data.local_anchor1();
        let arm_anchor = joint.data.local_anchor2();

        //draw_circle(body_anchor.x + body_pos_macroquad.x, body_anchor.y + body_pos_macroquad.y, 10., RED);
        //draw_circle(arm_anchor.x + arm_pos_macroquad.x, arm_anchor.y + arm_pos_macroquad.y, 5., GREEN);
    }
}