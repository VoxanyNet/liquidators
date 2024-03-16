// use diff::Diff;
// use serde::{Deserialize, Serialize};

// use macroquad::math::{Rect as ParentRect, Vec2};

// #[derive(Serialize, Deserialize, Diff)]
// #[diff(attr(
//     #[derive(Serialize, Deserialize)]
// ))]
// pub struct Rect {
//     x: f32,
//     y: f32,
//     w: f32,
//     h: f32
// }



// impl From<macroquad::math::Rect> for Rect {
//     fn from(value: macroquad::math::Rect) -> Self {
//         Self {
//             x: value.x,
//             y: value.y,
//             w: value.w,
//             h: value.h
//         }
//     }
// }

// impl Into<macroquad::math::Rect> for Rect {
//     fn into(self) -> macroquad::math::Rect {
//         macroquad::math::Rect {
//             x: self.x,
//             y: self.y,
//             w: self.w,
//             h: self.h
//         }
//     }
// }

// impl<'a> Into<macroquad::math::Rect> for &'a Rect {
//     fn into(self) -> macroquad::math::Rect {
//         macroquad::math::Rect {
//             x: self.x,
//             y: self.y,
//             w: self.w,
//             h: self.h
//         }
//     }
// }

// impl<'a> Into<macroquad::math::Rect> for &'a mut Rect {
//     fn into(self) -> macroquad::math::Rect {
//         macroquad::math::Rect {
//             x: self.x,
//             y: self.y,
//             w: self.w,
//             h: self.h
//         }
//     }
// }


// impl Rect {
//     pub fn new(x: f32, y: f32, w: f32, h: f32) -> Rect {
//         Rect::from(ParentRect::new(x, y, w, h))
//     }

//     pub fn point(&self) -> Vec2 {
//         ParentRect::point(&self.into())
//     }

//     /// Returns the size (width and height) of the `Rect`.
//     pub fn size(&self) -> Vec2 {
//         ParentRect::size(&self.into())
//     }

//     /// Returns the center position of the `Rect`.
//     pub fn center(&self) -> Vec2 {
//         ParentRect::center(&self.into())
//     }

//     /// Returns the left edge of the `Rect`
//     pub fn left(&self) -> f32 {
//         ParentRect::left(&self.into())
//     }

//     /// Returns the right edge of the `Rect`
//     pub fn right(&self) -> f32 {
//         ParentRect::right(&self.into())
//     }

//     /// Returns the top edge of the `Rect`
//     pub fn top(&self) -> f32 {
//         ParentRect::top(&self.into())
//     }

//     /// Returns the bottom edge of the `Rect`
//     pub fn bottom(&self) -> f32 {
//         ParentRect::bottom(&self.into())
//     }

//     /// Moves the `Rect`'s origin to (x, y)
//     pub fn move_to(&mut self, destination: Vec2) {
//         ParentRect::move_to(&mut self.into(), destination)
//     }

//     /// Scales the `Rect` by a factor of (sx, sy),
//     /// growing towards the bottom-left
//     pub fn scale(&mut self, sx: f32, sy: f32) {
//         ParentRect::scale(&mut self.into(), sx, sy)
//     }

//     /// Checks whether the `Rect` contains a `Point`
//     pub fn contains(&self, point: Vec2) -> bool {
//         ParentRect::contains(&self.into(), point)
//     }

//     /// Checks whether the `Rect` overlaps another `Rect`
//     pub fn overlaps(&self, other: &Rect) -> bool {
//         ParentRect::overlaps(&self.into(), &other.into())
//     }

//     /// Returns a new `Rect` that includes all points of these two `Rect`s.
//     pub fn combine_with(self, other: Rect) -> Rect {
//         ParentRect::combine_with(self.into(), other.into()).into()
//     }

//     /// Returns an intersection rect there is any intersection
//     pub fn intersect(&self, other: Rect) -> Option<Rect> {
//         match ParentRect::intersect(&self.into(), other.into()) {
//             Some(rect) => rect.into()
//         }
//     }

//     /// Translate rect origin be `offset` vector
//     pub fn offset(self, offset: Vec2) -> Rect {
//         Rect::new(self.x + offset.x, self.y + offset.y, self.w, self.h)
//     }
// }