use std::ops::{Add, AddAssign, Mul, Neg};

use diff::Diff;
use serde::{Deserialize, Serialize};
use macroquad::math::{BVec2, Vec3};

#[derive(Serialize, Deserialize, Diff, Clone, Copy)]
#[diff(attr(
    #[derive(Serialize, Deserialize)]
))]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl From<macroquad::math::Vec2> for Vec2 {
    fn from(value: macroquad::math::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}

impl From<&mut macroquad::math::Vec2> for Vec2 {
    fn from(value: &mut macroquad::math::Vec2) -> Self {
        Self {
            x: value.x,
            y: value.y
        }
    }
}


impl Into<macroquad::math::Vec2> for Vec2 {
    fn into(self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<macroquad::math::Vec2> for &Vec2 {
    fn into(self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

impl Into<macroquad::math::Vec2> for &mut Vec2 {
    fn into(self) -> macroquad::math::Vec2 {
        macroquad::math::Vec2 {
            x: self.x,
            y: self.y,
        }
    }
}

// impl<'a> Into<&'a macroquad::math::Vec2> for &Vec2 {
//     fn into(self) -> &'a macroquad::math::Vec2 {
//         &macroquad::math::Vec2 {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

// impl<'a> Into<&'a mut macroquad::math::Vec2> for &mut Vec2 {
//     fn into(self) -> &'a mut macroquad::math::Vec2 {
//         &mut macroquad::math::Vec2 {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

impl Mul<f32> for Vec2 {

    type Output = Vec2;

    fn mul(self, rhs: f32) -> Self::Output {
        macroquad::math::Vec2::mul(self.into(), rhs).into()
    }
    
}

impl Neg for Vec2 {

    type Output = Vec2;

    fn neg(self) -> Self::Output {
        macroquad::math::Vec2::neg(self.into()).into()
    }
    
    
}

impl AddAssign<Vec2> for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        let mut parent: macroquad::math::Vec2 = self.into();

        let rhs_parent: macroquad::math::Vec2 = rhs.into();
        
        parent.add_assign(rhs_parent);

        *self = parent.into();
    }
}

impl Add<Vec2> for Vec2 {
    fn add(self, rhs: Vec2) -> Self::Output {
        let parent: macroquad::math::Vec2 = self.into();
        let rhs_parent: macroquad::math::Vec2 = rhs.into();

        let parent_result = parent.add(rhs_parent);

        parent_result.into()
    }
    
    type Output = Vec2;
}

impl Vec2 {
    /// All zeroes.
    pub const ZERO: Self = Self::splat(0.0);

    /// All ones.
    pub const ONE: Self = Self::splat(1.0);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::splat(-1.0);

    /// All NAN.
    pub const NAN: Self = Self::splat(f32::NAN);

    /// A unit-length vector pointing along the positive X axis.
    pub const X: Self = Self::new(1.0, 0.0);

    /// A unit-length vector pointing along the positive Y axis.
    pub const Y: Self = Self::new(0.0, 1.0);

    /// A unit-length vector pointing along the negative X axis.
    pub const NEG_X: Self = Self::new(-1.0, 0.0);

    /// A unit-length vector pointing along the negative Y axis.
    pub const NEG_Y: Self = Self::new(0.0, -1.0);

    /// The unit axes.
    pub const AXES: [Self; 2] = [Self::X, Self::Y];

    /// Creates a new vector.
    #[inline(always)]
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    /// Creates a vector with all elements set to `v`.
    #[inline]
    pub const fn splat(v: f32) -> Self {
        Self { x: v, y: v }
    }

    /// Creates a vector from the elements in `if_true` and `if_false`, selecting which to use
    /// for each element of `self`.
    ///
    /// A true element in the mask uses the corresponding element from `if_true`, and false
    /// uses the element from `if_false`.
    #[inline]
    pub fn select(mask: macroquad::math::BVec2, if_true: Self, if_false: Self) -> Self {

        macroquad::math::Vec2::select(mask, if_true.into(), if_false.into()).into()
    }

    /// Creates a new vector from an array.
    #[inline]
    pub const fn from_array(a: [f32; 2]) -> Self {
        Self::new(a[0], a[1])
    }

    /// `[x, y]`
    pub const fn to_array(&self) -> [f32; 2] {
        [self.x, self.y]
    }

    /// Creates a vector from the first 2 values in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub const fn from_slice(slice: &[f32]) -> Self {
        Self::new(slice[0], slice[1])
    }

    /// Writes the elements of `self` to the first 2 elements in `slice`.
    ///
    /// # Panics
    ///
    /// Panics if `slice` is less than 2 elements long.
    #[inline]
    pub fn write_to_slice(self, slice: &mut [f32]) {
        macroquad::math::Vec2::write_to_slice(self.into(), slice)
    }

    /// Creates a 3D vector from `self` and the given `z` value.
    #[inline]
    pub const fn extend(self, z: f32) -> Vec3 {
        macroquad::math::Vec3::new(self.x, self.y, z)
    }

    /// Computes the dot product of `self` and `rhs`.
    #[inline]
    pub fn dot(self, rhs: Self) -> f32 {
        macroquad::math::Vec2::dot(self.into(), rhs.into())
    }

    /// Returns a vector containing the minimum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.min(rhs.x), self.y.min(rhs.y), ..]`.
    #[inline]
    pub fn min(self, rhs: Self) -> Self {
        macroquad::math::Vec2::min(self.into(), rhs.into()).into()
    }

    /// Returns a vector containing the maximum values for each element of `self` and `rhs`.
    ///
    /// In other words this computes `[self.x.max(rhs.x), self.y.max(rhs.y), ..]`.
    #[inline]
    pub fn max(self, rhs: Self) -> Self {
        macroquad::math::Vec2::max(self.into(), rhs.into()).into()
    }

    /// Component-wise clamping of values, similar to [`f32::clamp`].
    ///
    /// Each element in `min` must be less-or-equal to the corresponding element in `max`.
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        macroquad::math::Vec2::clamp(self.into(), min.into(), max.into()).into()
    }

    /// Returns the horizontal minimum of `self`.
    ///
    /// In other words this computes `min(x, y, ..)`.
    #[inline]
    pub fn min_element(self) -> f32 {
        macroquad::math::Vec2::min_element(self.into())
    }

    /// Returns the horizontal maximum of `self`.
    ///
    /// In other words this computes `max(x, y, ..)`.
    #[inline]
    pub fn max_element(self) -> f32 {
        macroquad::math::Vec2::max_element(self.into())
    }

    /// Returns a vector mask containing the result of a `==` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words, this computes `[self.x == rhs.x, self.y == rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpeq(self, rhs: Self) -> BVec2 {
        macroquad::math::Vec2::cmpeq(self.into(), rhs.into())
    }

    /// Returns a vector mask containing the result of a `!=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x != rhs.x, self.y != rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpne(self, rhs: Self) -> BVec2 {
        macroquad::math::Vec2::cmpne(self.into(), rhs.into())
    }

    /// Returns a vector mask containing the result of a `>=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x >= rhs.x, self.y >= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpge(self, rhs: Self) -> BVec2 {
        macroquad::math::Vec2::cmpge(self.into(), rhs.into())
    }

    /// Returns a vector mask containing the result of a `>` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x > rhs.x, self.y > rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmpgt(self, rhs: Self) -> BVec2 {
        macroquad::math::Vec2::cmpgt(self.into(), rhs.into())
    }

    /// Returns a vector mask containing the result of a `<=` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x <= rhs.x, self.y <= rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmple(self, rhs: Self) -> BVec2 {
        macroquad::math::Vec2::cmple(self.into(), rhs.into())
    }

    /// Returns a vector mask containing the result of a `<` comparison for each element of
    /// `self` and `rhs`.
    ///
    /// In other words this computes `[self.x < rhs.x, self.y < rhs.y, ..]` for all
    /// elements.
    #[inline]
    pub fn cmplt(self, rhs: Self) -> BVec2 {
        macroquad::math::Vec2::cmplt(self.into(), rhs.into())
    }

    /// Returns a vector containing the absolute value of each element of `self`.
    #[inline]
    pub fn abs(self) -> Self {
        macroquad::math::Vec2::abs(self.into()).into()
    }

    /// Returns a vector with elements representing the sign of `self`.
    ///
    /// - `1.0` if the number is positive, `+0.0` or `INFINITY`
    /// - `-1.0` if the number is negative, `-0.0` or `NEG_INFINITY`
    /// - `NAN` if the number is `NAN`
    #[inline]
    pub fn signum(self) -> Self {
        macroquad::math::Vec2::signum(self.into()).into()
    }

    /// Returns `true` if, and only if, all elements are finite.  If any element is either
    /// `NaN`, positive or negative infinity, this will return `false`.
    #[inline]
    pub fn is_finite(self) -> bool {
        macroquad::math::Vec2::is_finite(self.into())
    }

    /// Returns `true` if any elements are `NaN`.
    #[inline]
    pub fn is_nan(self) -> bool {
        macroquad::math::Vec2::is_nan(self.into())
    }

    /// Performs `is_nan` on each element of self, returning a vector mask of the results.
    ///
    /// In other words, this computes `[x.is_nan(), y.is_nan(), z.is_nan(), w.is_nan()]`.
    #[inline]
    pub fn is_nan_mask(self) -> BVec2 {
        macroquad::math::Vec2::is_nan_mask(self.into())
    }

    /// Computes the length of `self`.
    #[doc(alias = "magnitude")]
    #[inline]
    pub fn length(self) -> f32 {
        macroquad::math::Vec2::length(self.into())
    }

    /// Computes the squared length of `self`.
    ///
    /// This is faster than `length()` as it avoids a square root operation.
    #[doc(alias = "magnitude2")]
    #[inline]
    pub fn length_squared(self) -> f32 {
        macroquad::math::Vec2::length_squared(self.into())
    }

    /// Computes `1.0 / length()`.
    ///
    /// For valid results, `self` must _not_ be of length zero.
    #[inline]
    pub fn length_recip(self) -> f32 {
        macroquad::math::Vec2::length_recip(self.into())
    }

    /// Computes the Euclidean distance between two points in space.
    #[inline]
    pub fn distance(self, rhs: Self) -> f32 {
        macroquad::math::Vec2::distance(self.into(), rhs.into())
    }

    /// Compute the squared euclidean distance between two points in space.
    #[inline]
    pub fn distance_squared(self, rhs: Self) -> f32 {
        macroquad::math::Vec2::distance_squared(self.into(), rhs.into())
    }

    /// Returns `self` normalized to length 1.0.
    ///
    /// For valid results, `self` must _not_ be of length zero, nor very close to zero.
    ///
    /// See also [`Self::try_normalize`] and [`Self::normalize_or_zero`].
    ///
    /// Panics
    ///
    /// Will panic if `self` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn normalize(self) -> Self {
        macroquad::math::Vec2::normalize(self.into()).into()
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns `None`.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be `None`.
    ///
    /// See also [`Self::normalize_or_zero`].
    #[must_use]
    #[inline]
    pub fn try_normalize(self) -> Option<Self> {
        match macroquad::math::Vec2::try_normalize(self.into()) {
            Some(vec2) => Some(vec2.into()),
            None => None,
        }
    }

    /// Returns `self` normalized to length 1.0 if possible, else returns zero.
    ///
    /// In particular, if the input is zero (or very close to zero), or non-finite,
    /// the result of this operation will be zero.
    ///
    /// See also [`Self::try_normalize`].
    #[must_use]
    #[inline]
    pub fn normalize_or_zero(self) -> Self {
        macroquad::math::Vec2::normalize_or_zero(self.into()).into()
    }

    /// Returns whether `self` is length `1.0` or not.
    ///
    /// Uses a precision threshold of `1e-6`.
    #[inline]
    pub fn is_normalized(self) -> bool {
        // TODO: do something with epsilon
        macroquad::math::Vec2::is_normalized(self.into())
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is zero length when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto(self, rhs: Self) -> Self {
        macroquad::math::Vec2::project_onto(self.into(), rhs.into()).into()
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be of non-zero length.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` has a length of zero when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from(self, rhs: Self) -> Self {
        macroquad::math::Vec2::reject_from(self.into(), rhs.into()).into()
    }

    /// Returns the vector projection of `self` onto `rhs`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn project_onto_normalized(self, rhs: Self) -> Self {
        macroquad::math::Vec2::project_onto_normalized(self.into(), rhs.into()).into()
    }

    /// Returns the vector rejection of `self` from `rhs`.
    ///
    /// The vector rejection is the vector perpendicular to the projection of `self` onto
    /// `rhs`, in rhs words the result of `self - self.project_onto(rhs)`.
    ///
    /// `rhs` must be normalized.
    ///
    /// # Panics
    ///
    /// Will panic if `rhs` is not normalized when `glam_assert` is enabled.
    #[must_use]
    #[inline]
    pub fn reject_from_normalized(self, rhs: Self) -> Self {
        macroquad::math::Vec2::reject_from_normalized(self.into(), rhs.into()).into()
    }

    /// Returns a vector containing the nearest integer to a number for each element of `self`.
    /// Round half-way cases away from 0.0.
    #[inline]
    pub fn round(self) -> Self {
        macroquad::math::Vec2::round(self.into()).into()
    }

    /// Returns a vector containing the largest integer less than or equal to a number for each
    /// element of `self`.
    #[inline]
    pub fn floor(self) -> Self {
        macroquad::math::Vec2::floor(self.into()).into()
    }

    /// Returns a vector containing the smallest integer greater than or equal to a number for
    /// each element of `self`.
    #[inline]
    pub fn ceil(self) -> Self {
        macroquad::math::Vec2::ceil(self.into()).into()
    }

    /// Returns a vector containing the fractional part of the vector, e.g. `self -
    /// self.floor()`.
    ///
    /// Note that this is fast but not precise for large numbers.
    #[inline]
    pub fn fract(self) -> Self {
        macroquad::math::Vec2::fract(self.into()).into()
    }

    /// Returns a vector containing `e^self` (the exponential function) for each element of
    /// `self`.
    #[inline]
    pub fn exp(self) -> Self {
        macroquad::math::Vec2::exp(self.into()).into()
    }

    /// Returns a vector containing each element of `self` raised to the power of `n`.
    #[inline]
    pub fn powf(self, n: f32) -> Self {
        macroquad::math::Vec2::powf(self.into(), n).into()
    }

    /// Returns a vector containing the reciprocal `1.0/n` of each element of `self`.
    #[inline]
    pub fn recip(self) -> Self {
        macroquad::math::Vec2::recip(self.into()).into()
    }

    /// Performs a linear interpolation between `self` and `rhs` based on the value `s`.
    ///
    /// When `s` is `0.0`, the result will be equal to `self`.  When `s` is `1.0`, the result
    /// will be equal to `rhs`. When `s` is outside of range `[0, 1]`, the result is linearly
    /// extrapolated.
    #[doc(alias = "mix")]
    #[inline]
    pub fn lerp(self, rhs: Self, s: f32) -> Self {
        macroquad::math::Vec2::lerp(self.into(), rhs.into(), s).into()
    }

    /// Returns true if the absolute difference of all elements between `self` and `rhs` is
    /// less than or equal to `max_abs_diff`.
    ///
    /// This can be used to compare if two vectors contain similar elements. It works best when
    /// comparing with a known value. The `max_abs_diff` that should be used used depends on
    /// the values being compared against.
    ///
    /// For more see
    /// [comparing floating point numbers](https://randomascii.wordpress.com/2012/02/25/comparing-floating-point-numbers-2012-edition/).
    #[inline]
    pub fn abs_diff_eq(self, rhs: Self, max_abs_diff: f32) -> bool {
        macroquad::math::Vec2::abs_diff_eq(self.into(), rhs.into(), max_abs_diff)
    }

    /// Returns a vector with a length no less than `min` and no more than `max`
    ///
    /// # Panics
    ///
    /// Will panic if `min` is greater than `max` when `glam_assert` is enabled.
    #[inline]
    pub fn clamp_length(self, min: f32, max: f32) -> Self {
        macroquad::math::Vec2::clamp_length(self.into(), min, max).into()
    }

    /// Returns a vector with a length no more than `max`
    pub fn clamp_length_max(self, max: f32) -> Self {
        macroquad::math::Vec2::clamp_length_max(self.into(), max).into()
    }

    /// Returns a vector with a length no less than `min`
    pub fn clamp_length_min(self, min: f32) -> Self {
        macroquad::math::Vec2::clamp_length_min(self.into(), min).into()
    }

    /// Fused multiply-add. Computes `(self * a) + b` element-wise with only one rounding
    /// error, yielding a more accurate result than an unfused multiply-add.
    ///
    /// Using `mul_add` *may* be more performant than an unfused multiply-add if the target
    /// architecture has a dedicated fma CPU instruction. However, this is not always true,
    /// and will be heavily dependant on designing algorithms with specific target hardware in
    /// mind.
    #[inline]
    pub fn mul_add(self, a: Self, b: Self) -> Self {
        macroquad::math::Vec2::mul_add(self.into(), a.into(), b.into()).into()
    }

    /// Creates a 2D vector containing `[angle.cos(), angle.sin()]`. This can be used in
    /// conjunction with the `rotate` method, e.g. `Vec2::from_angle(PI).rotate(Vec2::Y)` will
    /// create the vector [-1, 0] and rotate `Vec2::Y` around it returning `-Vec2::Y`.
    #[inline]
    pub fn from_angle(angle: f32) -> Self {
        macroquad::math::Vec2::from_angle(angle).into()
    }

    /// Returns the angle (in radians) between `self` and `rhs`.
    ///
    /// The input vectors do not need to be unit length however they must be non-zero.
    #[inline]
    pub fn angle_between(self, rhs: Self) -> f32 {
        macroquad::math::Vec2::angle_between(self.into(), rhs.into())
    }

    /// Returns a vector that is equal to `self` rotated by 90 degrees.
    #[inline]
    pub fn perp(self) -> Self {
        macroquad::math::Vec2::perp(self.into()).into()
    }

    /// The perpendicular dot product of `self` and `rhs`.
    /// Also known as the wedge product, 2D cross product, and determinant.
    #[doc(alias = "wedge")]
    #[doc(alias = "cross")]
    #[doc(alias = "determinant")]
    #[inline]
    pub fn perp_dot(self, rhs: Self) -> f32 {
        macroquad::math::Vec2::perp_dot(self.into(), rhs.into())
    }

    /// Returns `rhs` rotated by the angle of `self`. If `self` is normalized,
    /// then this just rotation. This is what you usually want. Otherwise,
    /// it will be like a rotation with a multiplication by `self`'s length.
    #[must_use]
    #[inline]
    pub fn rotate(self, rhs: Self) -> Self {
        macroquad::math::Vec2::rotate(self.into(), rhs.into()).into()
    }

    /// Casts all elements of `self` to `f64`.
    #[inline]
    pub fn as_dvec2(&self) -> macroquad::math::DVec2 {
        macroquad::math::Vec2::as_dvec2(&self.into())
    }

    /// Casts all elements of `self` to `i32`.
    #[inline]
    pub fn as_ivec2(&self) -> macroquad::math::IVec2 {
        macroquad::math::Vec2::as_ivec2(&self.into())
    }

    /// Casts all elements of `self` to `u32`.
    #[inline]
    pub fn as_uvec2(&self) -> macroquad::math::UVec2 {
        macroquad::math::Vec2::as_uvec2(&self.into())
    }
}