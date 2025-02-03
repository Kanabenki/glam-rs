// Generated from vec_wide.rs.tera template. Edit the template, not the generated file.

use crate::Vec2;

#[inline(always)]
#[must_use]
pub const fn vec2x4(x: [f32; 4], y: [f32; 4]) -> Vec2X4 {
    Vec2X4::new(x, y)
}

#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), repr(C))]
#[cfg_attr(target_arch = "spirv", repr(simd))]
pub struct Vec2X4 {
    pub x: [f32; 4],
    pub y: [f32; 4],
}

impl Vec2X4 {
    /// All zeroes.
    pub const ZERO: Self = Self::broadcast([0.0; 4]);

    /// All ones.
    pub const ONE: Self = Self::broadcast([1.0; 4]);

    /// All negative ones.
    pub const NEG_ONE: Self = Self::broadcast([-1.0; 4]);

    /// All `f32::MIN`.
    pub const MIN: Self = Self::broadcast([f32::MIN; 4]);

    /// All `f32::MAX`.
    pub const MAX: Self = Self::broadcast([f32::MAX; 4]);

    /// All `f32::NAN`.
    pub const NAN: Self = Self::broadcast([f32::NAN; 4]);

    /// All `f32::INFINITY`.
    pub const INFINITY: Self = Self::broadcast([f32::INFINITY; 4]);

    /// All `f32::NEG_INFINITY`.
    pub const NEG_INFINITY: Self = Self::broadcast([f32::NEG_INFINITY; 4]);

    pub const X: Self = Self::new([1.0; 4], [0.0; 4]);

    pub const Y: Self = Self::new([0.0; 4], [1.0; 4]);

    pub const NEG_X: Self = Self::new([-1.0; 4], [0.0; 4]);

    pub const NEG_Y: Self = Self::new([0.0; 4], [-1.0; 4]);

    #[inline(always)]
    #[must_use]
    pub const fn new(x: [f32; 4], y: [f32; 4]) -> Self {
        Self { x, y }
    }

    #[inline]
    #[must_use]
    pub const fn broadcast(v: [f32; 4]) -> Self {
        Self::new(v, v)
    }

    #[inline]
    #[must_use]
    pub const fn splat(x: f32, y: f32) -> Self {
        Self {
            x: [x; 4],
            y: [y; 4],
        }
    }

    #[inline]
    #[must_use]
    pub const fn splat_vec(v: Vec2) -> Self {
        Self {
            x: [v.x; 4],
            y: [v.y; 4],
        }
    }

    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> [f32; 4] {
        self.dot(self).map(math::sqrt)
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> [f32; 4] {
        [
            [(self.x[i] * rhs.x[i]) + (self.y[i] * rhs.y[i])],
            [(self.x[i] * rhs.x[i]) + (self.y[i] * rhs.y[i])],
            [(self.x[i] * rhs.x[i]) + (self.y[i] * rhs.y[i])],
            [(self.x[i] * rhs.x[i]) + (self.y[i] * rhs.y[i])],
        ]
    }

    #[inline]
    #[must_use]
    pub fn is_finite(self) -> bool {
        todo!()
    }

    #[inline]
    #[must_use]
    pub fn normalize(self) -> Self {
        todo!()
    }

    #[inline]
    #[must_use]
    pub fn length_recip(self) -> [f32; 4] {
        todo!()
    }
}
