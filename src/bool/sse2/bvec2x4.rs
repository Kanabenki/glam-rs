// Generated from vec_mask_wide.rs.tera template. Edit the template, not the generated file.

use core::fmt;
use core::ops::*;

#[cfg(target_arch = "x86")]
use core::arch::x86::*;
#[cfg(target_arch = "x86_64")]
use core::arch::x86_64::*;

#[repr(C)]
union UnionCast {
    a: [u32; 4],
    v: __m128,
}

#[inline(always)]
#[must_use]
pub const fn bvec2ax4(x: [bool; 4], y: [bool; 4]) -> BVec2AX4 {
    BVec2AX4::new(x, y)
}
#[derive(Clone, Copy)]
pub struct BVec2AX4 {
    pub(crate) x: __m128,
    pub(crate) y: __m128,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec2AX4 {
    #[inline(always)]
    #[must_use]
    pub const fn new(x: [bool; 4], y: [bool; 4]) -> Self {
        Self {
            x: unsafe {
                UnionCast {
                    a: [
                        MASK[x[0] as usize],
                        MASK[x[1] as usize],
                        MASK[x[2] as usize],
                        MASK[x[3] as usize],
                    ],
                }
                .v
            },
            y: unsafe {
                UnionCast {
                    a: [
                        MASK[y[0] as usize],
                        MASK[y[1] as usize],
                        MASK[y[2] as usize],
                        MASK[y[3] as usize],
                    ],
                }
                .v
            },
        }
    }
}
