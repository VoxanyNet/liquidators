use diff::Diff;
use serde::{Deserialize, Serialize};
use crate::proxies::macroquad::math::vec2::Vec2;

#[derive(Serialize, Deserialize, Diff, Clone, Copy)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32
}

impl From<macroquad::math::Rect> for Rect {
    fn from(value: macroquad::math::Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            w: value.w,
            h: value.h
        }
    }
}

impl From<&macroquad::math::Rect> for Rect {
    fn from(value: &macroquad::math::Rect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            w: value.w,
            h: value.h
        }
    }
}

// impl From<&macroquad::math::Rect> for &Rect {
//     fn from(value: &macroquad::math::Rect) -> Self {
//         &Rect {
//             x: value.x,
//             y: value.y,
//             w: value.w,
//             h: value.h
//         }
//     }
// }

// impl From<&mut macroquad::math::Rect> for &mut Rect {
//     fn from(value: &mut macroquad::math::Rect) -> Self {
//         &mut Rect {
//             x: value.x,
//             y: value.y,
//             w: value.w,
//             h: value.h
//         }
//     }
// }

impl From<&mut macroquad::math::Rect> for Rect {
    fn from(value: &mut macroquad::math::Rect) -> Self {
        Rect {
            x: value.x,
            y: value.y,
            w: value.w,
            h: value.h
        }
    }
}

impl Into<macroquad::math::Rect> for Rect {
    fn into(self) -> macroquad::math::Rect {
        macroquad::math::Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}

// impl<'a> Into<&'a macroquad::math::Rect> for Rect {
//     fn into(self) -> &'a macroquad::math::Rect {
//         &macroquad::math::Rect {
//             x: self.x,
//             y: self.y,
//             w: self.w,
//             h: self.h
//         }
//     }
// }

// impl<'a> Into<&'a macroquad::math::Rect> for &Rect {
//     fn into(self) -> &'a macroquad::math::Rect {
//         &macroquad::math::Rect {
//             x: self.x,
//             y: self.y,
//             w: self.w,
//             h: self.h
//         }
//     }
// }

// impl<'a> Into<&'a mut macroquad::math::Rect> for &mut Rect {
//     fn into(self) -> &'a mut macroquad::math::Rect {
//         &mut macroquad::math::Rect {
//             x: self.x,
//             y: self.y,
//             w: self.w,
//             h: self.h
//         }
//     }
// }

impl Into<macroquad::math::Rect> for &Rect {
    fn into(self) -> macroquad::math::Rect {
        macroquad::math::Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}

impl Into<macroquad::math::Rect> for &mut Rect {
    fn into(self) -> macroquad::math::Rect {
        macroquad::math::Rect {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h
        }
    }
}


impl Rect {

    pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
        Rect { x, y, w, h }
    }

    /// Returns the top-left corner of the `Rect`.
    pub fn point(&self) -> Vec2{
        let parent: macroquad::math::Rect = self.into();

        parent.point().into()
    }

    /// Returns the size (width and height) of the `Rect`.
    pub fn size(&self) -> Vec2 {
        let parent: macroquad::math::Rect = self.into();
        parent.size().into()
    }

    /// Returns the center position of the `Rect`.
    pub fn center(&self) -> Vec2 {
        let parent: macroquad::math::Rect = self.into();
        parent.center().into()
    }

    /// Returns the left edge of the `Rect`
    pub fn left(&self) -> f32 {
        let parent: macroquad::math::Rect = self.into();
        parent.left()
    }

    /// Returns the right edge of the `Rect`
    pub fn right(&self) -> f32 {
        let parent: macroquad::math::Rect = self.into();
        parent.right()
    }

    /// Returns the top edge of the `Rect`
    pub fn top(&self) -> f32 {
        let parent: macroquad::math::Rect = self.into();
        parent.top()
    }

    /// Returns the bottom edge of the `Rect`
    pub fn bottom(&self) -> f32 {
        let parent: macroquad::math::Rect = self.into();
        parent.bottom()
    }

    /// Moves the `Rect`'s origin to (x, y)
    pub fn move_to(&mut self, destination: Vec2) {
        let mut parent: macroquad::math::Rect = self.into();
        parent.move_to(destination.into());
        *self = parent.into();

    }

    /// Scales the `Rect` by a factor of (sx, sy),
    /// growing towards the bottom-left
    pub fn scale(&mut self, sx: f32, sy: f32) {
        let mut parent: macroquad::math::Rect = self.into();
        parent.scale(sx, sy);
        *self = parent.into();
    }

    /// Checks whether the `Rect` contains a `Point`
    pub fn contains(&self, point: Vec2) -> bool {
        let parent:  macroquad::math::Rect = self.into();
        parent.contains(point.into())
    }

    /// Checks whether the `Rect` overlaps another `Rect`
    pub fn overlaps(&self, other: &Rect) -> bool {
        let parent: macroquad::math::Rect = self.into();
        parent.overlaps(&other.into())
    }

    /// Returns a new `Rect` that includes all points of these two `Rect`s.
    pub fn combine_with(self, other: Rect) -> Rect {
        let parent: macroquad::math::Rect = self.into();
        parent.combine_with(other.into()).into()
    }

    /// Returns an intersection rect there is any intersection
    pub fn intersect(&self, other: Rect) -> Option<Rect> {
        let parent: macroquad::math::Rect = self.into();
        match parent.intersect(other.into()) {
            Some(rect) => Some(rect.into()),
            None => None,
        }
    }

    /// Translate rect origin be `offset` vector
    pub fn offset(self, offset: Vec2) -> Rect {
        let parent: macroquad::math::Rect = self.into();
        parent.offset(offset.into()).into()
    }
}
