use std::time::Duration;

use diff::Diff;
use gamelibrary::{texture_loader::TextureLoader, time::Time};
use macroquad::{color::WHITE, math::Vec2, texture::{draw_texture_ex, DrawTextureParams}};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Default)]
pub struct MuzzleFlash {
    last_flash: Time,
    sprite: String,
    duration: Duration
}

impl MuzzleFlash {

    pub fn flash(&mut self) {
        self.last_flash = Time::now()
    }

    // creating a new muzzle flash does not flash it by default
    pub fn new(sprite_path: String, duration: Duration) -> Self {
        Self {
            last_flash: Time::new(0),
            sprite: sprite_path,
            duration,
        }
    }
    pub async fn draw(&self, position: Vec2, textures: &mut TextureLoader) {


        if self.last_flash.elapsed().num_milliseconds() > self.duration.as_millis() as i64 {
            return;
        }

        let texture = textures.get(&self.sprite).await;
        
        draw_texture_ex(texture, position.x, position.y, WHITE, DrawTextureParams::default());
        
    }

}

#[derive(Serialize, Deserialize)]
pub struct MuzzleFlashDiff {
    flash: bool,
    sprite: Option<String>,
    duration: Option<Duration>
}

impl Diff for MuzzleFlash {
    type Repr = MuzzleFlashDiff;

    fn diff(&self, other: &Self) -> Self::Repr {
        // this is an example where we are implementing the diff function a lot differently
        // instead of diffing the muzzle flash in a way you would expect FUCK YOU

        let mut diff = MuzzleFlashDiff {
            
            flash: false,
            sprite: None,
            duration: None,
        };
        
        if other.last_flash != self.last_flash {
            diff.flash = true;
        };

        if other.sprite != self.sprite {
            diff.sprite = Some(other.sprite.clone());
        }

        if other.duration != self.duration {
            diff.duration = Some(other.duration);
        }

        diff
    }

    fn apply(&mut self, diff: &Self::Repr) {

        if diff.flash {

            self.last_flash = Time::now();
        }

        if let Some(duration) = diff.duration {
            self.duration = duration;
        }

        if let Some(sprite) = &diff.sprite {
            self.sprite = sprite.clone();
        }
    }

    fn identity() -> Self {
        Self { 
            last_flash: Time::new(0),
            sprite: String::identity(),
            duration: Duration::ZERO,
        }
    }
}