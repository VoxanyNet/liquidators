use editor::Editor;
use gamelibrary::space::Space;
use liquidators_lib::level::Level;
use macroquad::{miniquad::conf::Platform, window::Conf};

pub mod menu;
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
    conf.platform.swap_interval = Some(0); // disable vsync
    conf
}

#[macroquad::main(window_conf)]
async fn main() {

    let level = Level { 
        physics_squares: vec![], 
        structures: vec![],
        space: Space::new(-980.)
    };

    
    let mut editor = Editor { 
        level,
        menu: None
    };

    editor.run().await;
    
    
}