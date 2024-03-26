//! Color types and helpers.

use diff::Diff;
use serde::{Deserialize, Serialize};


/// A color represented by 4 floats: red, green, blue and alpha.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Serialize, Deserialize, Diff)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Color {
    /// Red channel value from 0.0 to 1.0
    pub r: f32,
    /// Blue channel value from 0.0 to 1.0
    pub g: f32,
    /// Green channel value from 0.0 to 1.0
    pub b: f32,
    /// Alpha channel value from 0.0 to 1.0
    pub a: f32,
}

impl From<macroquad::color::Color> for Color {
    fn from(value: macroquad::color::Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<&macroquad::color::Color> for Color {
    fn from(value: &macroquad::color::Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl From<&mut macroquad::color::Color> for Color {
    fn from(value: &mut macroquad::color::Color) -> Self {
        Self {
            r: value.r,
            g: value.g,
            b: value.b,
            a: value.a,
        }
    }
}

impl Into<macroquad::color::Color> for Color {
    fn into(self) -> macroquad::color::Color {
        macroquad::color::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl Into<macroquad::color::Color> for &Color {
    fn into(self) -> macroquad::color::Color {
        macroquad::color::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}


impl Into<macroquad::color::Color> for &mut Color {
    fn into(self) -> macroquad::color::Color {
        macroquad::color::Color {
            r: self.r,
            g: self.g,
            b: self.b,
            a: self.a,
        }
    }
}

impl Into<[u8; 4]> for Color {
    fn into(self) -> [u8; 4] {
        let parent: macroquad::color::Color = self.into();
        parent.into()
    }
}

impl Into<Color> for [u8; 4] {
    fn into(self) -> Color {
        let parent: macroquad::color::Color = self.into();
        parent.into()
    }
}

impl Into<[f32; 4]> for Color {
    fn into(self) -> [f32; 4] {
        let parent: macroquad::color::Color = self.into();
        parent.into()
    }
}

impl From<[f32; 4]> for Color {
    fn from(colors: [f32; 4]) -> Color {
        let parent: macroquad::color::Color = colors.into();
        parent.into()
    }
}

impl Color {
    /// Creates a new `Color` with the given red, green, blue, and alpha components.
    /// Values are expected to be between 0.0 and 1.0.
    ///
    /// # Example
    ///
    /// ```
    /// use macroquad::prelude::*;
    ///
    /// let pink = Color::new(1.00, 0.43, 0.76, 1.00);
    /// assert_eq!(pink.r, 1.00);
    /// assert_eq!(pink.g, 0.43);
    /// assert_eq!(pink.b, 0.76);
    /// assert_eq!(pink.a, 1.00);
    /// ```
    ///
    /// Note that values outside of this range are effectively clamped,
    /// and do not generate an error or warning.
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Color {
        Color { r, g, b, a }
    }

    /// Build a color from 4 components between 0 and 255.
    /// Unfortunately it can't be const fn due to [this issue](https://github.com/rust-lang/rust/issues/57241).
    /// When const version is needed "color_u8" macro may be a workaround.
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> Color {
        macroquad::color::Color::new(
            r as f32 / 255.,
            g as f32 / 255.,
            b as f32 / 255.,
            a as f32 / 255.,
        ).into()
    }

    /// Build a color from a hexadecimal u32
    ///
    /// # Example
    ///
    /// ```
    /// use macroquad::prelude::*;
    ///
    /// let light_blue = Color::from_hex(0x3CA7D5);
    /// assert_eq!(light_blue.r, 0.23529412);
    /// assert_eq!(light_blue.g, 0.654902);
    /// assert_eq!(light_blue.b, 0.8352941);
    /// assert_eq!(light_blue.a, 1.00);
    /// ```
    pub fn from_hex(hex: u32) -> Color {
        macroquad::color::Color::from_hex(hex).into()
    }

    /// Create a vec4 of red, green, blue, and alpha components.
    pub fn to_vec(&self) -> macroquad::math::Vec4 {
        macroquad::color::Color::to_vec(&self.into())
    }

    /// Create a color from a vec4 of red, green, blue, and alpha components.
    pub fn from_vec(vec: macroquad::math::Vec4) -> Self {
        macroquad::color::Color::from_vec(vec).into()
    }
}

pub mod colors {
    //! Constants for some common colors.

    use super::Color;

    pub const LIGHTGRAY: Color = Color::new(0.78, 0.78, 0.78, 1.00);
    pub const GRAY: Color = Color::new(0.51, 0.51, 0.51, 1.00);
    pub const DARKGRAY: Color = Color::new(0.31, 0.31, 0.31, 1.00);
    pub const YELLOW: Color = Color::new(0.99, 0.98, 0.00, 1.00);
    pub const GOLD: Color = Color::new(1.00, 0.80, 0.00, 1.00);
    pub const ORANGE: Color = Color::new(1.00, 0.63, 0.00, 1.00);
    pub const PINK: Color = Color::new(1.00, 0.43, 0.76, 1.00);
    pub const RED: Color = Color::new(0.90, 0.16, 0.22, 1.00);
    pub const MAROON: Color = Color::new(0.75, 0.13, 0.22, 1.00);
    pub const GREEN: Color = Color::new(0.00, 0.89, 0.19, 1.00);
    pub const LIME: Color = Color::new(0.00, 0.62, 0.18, 1.00);
    pub const DARKGREEN: Color = Color::new(0.00, 0.46, 0.17, 1.00);
    pub const SKYBLUE: Color = Color::new(0.40, 0.75, 1.00, 1.00);
    pub const BLUE: Color = Color::new(0.00, 0.47, 0.95, 1.00);
    pub const DARKBLUE: Color = Color::new(0.00, 0.32, 0.67, 1.00);
    pub const PURPLE: Color = Color::new(0.78, 0.48, 1.00, 1.00);
    pub const VIOLET: Color = Color::new(0.53, 0.24, 0.75, 1.00);
    pub const DARKPURPLE: Color = Color::new(0.44, 0.12, 0.49, 1.00);
    pub const BEIGE: Color = Color::new(0.83, 0.69, 0.51, 1.00);
    pub const BROWN: Color = Color::new(0.50, 0.42, 0.31, 1.00);
    pub const DARKBROWN: Color = Color::new(0.30, 0.25, 0.18, 1.00);
    pub const WHITE: Color = Color::new(1.00, 1.00, 1.00, 1.00);
    pub const BLACK: Color = Color::new(0.00, 0.00, 0.00, 1.00);
    pub const BLANK: Color = Color::new(0.00, 0.00, 0.00, 0.00);
    pub const MAGENTA: Color = Color::new(1.00, 0.00, 1.00, 1.00);
}

#[rustfmt::skip]
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Color {
    macroquad::color::hsl_to_rgb(h, s, l).into()
}

pub fn rgb_to_hsl(color: Color) -> (f32, f32, f32) {
    macroquad::color::rgb_to_hsl(color.into()).into()
}
