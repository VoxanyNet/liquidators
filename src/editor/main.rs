use editor::Editor;
use gamelibrary::{menu::Button, space::Space};
use liquidators_lib::level::Level;
use macroquad::{color::{DARKGRAY, WHITE}, math::Rect, miniquad::conf::Platform, window::Conf};
use nalgebra::vector;

pub mod editor;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Liquidators Level Editor".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true, 
        platform: Platform::default(),
        ..Default::default()
    };
    conf
}

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut editor = Editor::new();

    editor.run().await;

    
    
}