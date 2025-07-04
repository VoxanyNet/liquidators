use std::{f32::consts::PI, time::Instant};

use futures::executor::block_on;
use gamelibrary::{get_angle_to_mouse, macroquad_to_rapier, menu::Button, space::{Space, SyncImpulseJointHandle, SyncRigidBodyHandle}, texture_loader::TextureLoader};
use macroquad::{color::{Color, BLACK, DARKGRAY}, math::{Rect, Vec2}, miniquad::window::request_quit, text::{draw_text_ex, load_ttf_font, Font, TextParams}};
use nalgebra::vector;
use rapier2d::prelude::{RevoluteJointBuilder, RigidBodyBuilder};
use uuid::Uuid;

use crate::{player::body_part::BodyPart, TickContext};

// the main menu can be rendered on top of anything else
pub struct MainMenu {
    font: Font,
    head: BodyPart,
    space: Space,
    head_joint_base: SyncRigidBodyHandle,
    head_joint_base_joint: SyncImpulseJointHandle,
    new_game_button: Button,
    connect_game_button: Button,
    quit_button: Button,
    editor_button: Button,
    pub new_game: bool,
    pub connect: bool,
    pub quit: bool,
    pub launch_editor: bool
}

impl MainMenu {

    pub fn new(textures: &mut TextureLoader) -> Self {

        let mut clear_color = Color::default();

        clear_color.a = 0.;

        let new_game_button = Button::new("New Game".to_string(), Rect::new(50., 180., 150., 60.), clear_color, Some(clear_color), Some(clear_color), 50, "assets/fonts/CutePixel.ttf".to_string());

        let connect_game_button = Button::new("Connect".to_string(), Rect::new(50., 300., 150., 60.), clear_color, Some(clear_color), Some(clear_color), 50, "assets/fonts/CutePixel.ttf".to_string());

        let quit_button = Button::new("Quit".to_string(), Rect::new(50., 420., 150., 60.), clear_color, Some(clear_color), Some(clear_color), 50, "assets/fonts/CutePixel.ttf".to_string());

        let editor_button = Button::new("Editor".to_string(), Rect::new(50., 540., 150., 60.), clear_color, Some(clear_color), Some(clear_color), 50, "assets/fonts/CutePixel.ttf".to_string());

        let font = block_on(load_ttf_font("assets/fonts/CutePixel.ttf")).unwrap();
        
        let mut space = Space::new();

        let head_position_macroquad = Vec2 {
            x: 50.,
            y: 50.,
        };

        let head_position_rapier = macroquad_to_rapier(&head_position_macroquad);
        
        let head = BodyPart::new(
            "assets/cat/head.png".to_string(), 
            2, 
            10.,
            Vec2::ZERO, // this position doesnt matter   
            &mut space, 
            textures, 
            String::new()
        );

        let head_joint_base = space.sync_rigid_body_set.insert_sync(
            RigidBodyBuilder::fixed()
                .position(vector![head_position_rapier.x, head_position_rapier.y].into())
        );



        let head_joint_base_joint = space.sync_impulse_joint_set.insert_sync(
            space.sync_rigid_body_set.get_local_handle(head_joint_base),
            space.sync_rigid_body_set.get_local_handle(head.body_handle),
            RevoluteJointBuilder::new()
                .local_anchor1(vector![0., 0.].into())
                .local_anchor2(vector![0., -30.].into())
                .limits([-0.4, 0.4])
                .contacts_enabled(false)
            .build(),
            true
        );
        
        Self {
            font,
            space,
            head,
            head_joint_base,
            head_joint_base_joint,
            new_game_button: new_game_button,
            connect_game_button: connect_game_button,
            connect: false,
            quit_button,
            quit: false,
            editor_button,
            new_game: false,
            launch_editor: false
            
        }
    }

    pub fn angle_head_to_mouse(&mut self, camera_rect: &Rect) {

        let head_body = self.space.sync_rigid_body_set.get_sync_mut(self.head.body_handle).unwrap();

        let head_body_pos = Vec2::new(head_body.translation().x, head_body.translation().y);

        let angle_to_mouse = get_angle_to_mouse(head_body_pos, camera_rect);

        let head_joint = self.space.sync_impulse_joint_set.get_sync_mut(self.head_joint_base_joint).unwrap();

        let target_angle = -angle_to_mouse + (PI / 2.);


        if target_angle.abs() > 0.399 {
            // dont try to set the angle if we know its beyond the limit
            return;
        }

        head_joint.data.as_revolute_mut().unwrap().set_motor_position(target_angle, 300., 0.);

        return;

    }
    pub async fn draw(&self, textures: &mut TextureLoader) {

        let mut text_params = TextParams::default();

        text_params.font = Some(&self.font);
        text_params.font_size = 100;

        draw_text_ex(
            "Liquidators",
             50., 
             100., 
             text_params
        );

        //self.head.draw(textures, &self.space, false).await;
        
        self.new_game_button.draw().await;
        self.connect_game_button.draw().await;
        self.quit_button.draw().await;
        self.editor_button.draw().await;
    
    }

    pub fn tick(&mut self, ctx: &mut TickContext) {
        self.head.tick(&self.space, ctx);

        self.angle_head_to_mouse(ctx.camera_rect);

        self.space.step(&ctx.owned_rigid_bodies, &ctx.owned_colliders, &Vec::new(), ctx.last_tick_duration);

        self.new_game_button.update(Some(ctx.camera_rect));
        self.connect_game_button.update(Some(ctx.camera_rect));
        self.quit_button.update(Some(ctx.camera_rect));
        self.editor_button.update(Some(ctx.camera_rect));

        // this is a little dumb
        if self.new_game_button.clicked {
            self.new_game = true;
        }

        if self.connect_game_button.clicked {
            self.connect = true;
        };

        if self.quit_button.clicked {
            request_quit();
        }

        if self.editor_button.clicked {
            self.launch_editor = true;
        }
    }
}