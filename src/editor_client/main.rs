use editor_client::EditorClient;
use macroquad::{miniquad::conf::Platform, window::Conf};

pub mod editor_client;

fn window_conf() -> Conf {
    let mut conf = Conf {
        window_title: "Liquidators Level Editor".to_owned(),
        window_width: 1280,
        window_height: 720,
        window_resizable: true, 
        platform: Platform::default(),
        ..Default::default()
    };
    //conf.platform.swap_interval = Some(0); // disable vsync
    conf
}

#[macroquad::main(window_conf)]
async fn main() {
    
    let mut editor = EditorClient::connect("ws://voxany.net:5557");

    editor.run().await;

    
    
}