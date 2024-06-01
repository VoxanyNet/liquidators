use gamelibrary::proxies::macroquad::math::vec2::Vec2;
use liquidators_lib::level::Level;
use macroquad::{color::WHITE, input::{self, is_mouse_button_down, is_mouse_button_pressed}, miniquad::conf::Platform, window::Conf};

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

    let mut level = Level { 
        physics_squares: vec![], 
        structures: vec![] 
    };

    let mut starting_coords: Option<Vec2> = None;

    loop {

        match starting_coords {
            Some(coords) => {

                if input::is_mouse_button_down(input::MouseButton::Left) != true {
                    starting_coords = None
                }

                let mouse_pos = input::mouse_position();

                macroquad::shapes::draw_rectangle_lines(
                    coords.x, 
                    coords.y, 
                    mouse_pos.0 - coords.x, 
                    mouse_pos.1 - coords.y, 
                    5., 
                    WHITE
                )
            },
            None => {

                if input::is_mouse_button_down(input::MouseButton::Left) {
                    starting_coords = Some(
                        Vec2 { x: input::mouse_position().0, y: input::mouse_position().1 }
                    );
                }

            },
        }

        macroquad::window::next_frame().await;
        
        
    }
    
    
}