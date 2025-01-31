// Generated from vec_wide.rs.tera template. Edit the template, not the generated file.

use crate::sse2::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

use crate::Vec3;

#[repr(C)]
union UnionCast {
    a: [f32; 4],
    v: __m128,
}

#[inline(always)]
#[must_use]
pub const fn vec3x4(x: [f32; 4], y: [f32; 4], z: [f32; 4]) -> Vec3X4 {
    Vec3X4::new(x, y, z)
}

#[derive(Clone, Copy)]
pub struct Vec3X4 {
    pub(crate) x: __m128,
    pub(crate) y: __m128,
    pub(crate) z: __m128,
}

impl Vec3X4 {
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

    pub const X: Self = Self::new([1.0; 4], [0.0; 4], [0.0; 4]);

    pub const Y: Self = Self::new([0.0; 4], [1.0; 4], [0.0; 4]);

    pub const Z: Self = Self::new([0.0; 4], [0.0; 4], [1.0; 4]);

    pub const NEG_X: Self = Self::new([-1.0; 4], [0.0; 4], [0.0; 4]);

    pub const NEG_Y: Self = Self::new([0.0; 4], [-1.0; 4], [0.0; 4]);

    pub const NEG_Z: Self = Self::new([0.0; 4], [0.0; 4], [-1.0; 4]);

    #[inline(always)]
    #[must_use]
    pub const fn new(x: [f32; 4], y: [f32; 4], z: [f32; 4]) -> Self {
        unsafe {
            Self {
                x: UnionCast { a: x }.v,
                y: UnionCast { a: y }.v,
                z: UnionCast { a: z }.v,
            }
        }
    }

    #[inline]
    #[must_use]
    pub const fn broadcast(v: [f32; 4]) -> Self {
        Self::new(v, v, v)
    }

    #[inline]
    #[must_use]
    pub const fn splat(x: f32, y: f32, z: f32) -> Self {
        unsafe {
            Self {
                x: UnionCast { a: [x; 4] }.v,
                y: UnionCast { a: [y; 4] }.v,
                z: UnionCast { a: [z; 4] }.v,
            }
        }
    }

    #[inline]
    #[must_use]
    pub const fn splat_vec(v: Vec3) -> Self {
        let v = v.to_array();
        unsafe {
            Self {
                x: UnionCast { a: [v[0]; 4] }.v,
                y: UnionCast { a: [v[1]; 4] }.v,
                z: UnionCast { a: [v[2]; 4] }.v,
            }
        }
    }

    #[doc(alias = "magnitude")]
    #[inline]
    #[must_use]
    pub fn length(self) -> [f32; 4] {
        unsafe {
            let dot = dot3x4(self.x, self.y, self.z, self.x, self.y, self.z);
            let length = _mm_sqrt_ps(dot);
            UnionCast { v: length }.a
        }
    }

    #[inline]
    #[must_use]
    pub fn dot(self, rhs: Self) -> [f32; 4] {
        unsafe {
            let dot = dot3x4(self.x, self.y, self.z, rhs.x, rhs.y, rhs.z);
            UnionCast { v: dot }.a
        }
    }
}
