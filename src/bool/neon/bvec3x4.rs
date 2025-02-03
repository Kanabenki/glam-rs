// Generated from vec_mask_wide.rs.tera template. Edit the template, not the generated file.

use core::fmt;
use core::ops::*;

#[inline(always)]
#[must_use]
pub const fn bvec3ax4(x: [bool; 4], y: [bool; 4], z: [bool; 4]) -> BVec3AX4 {
    BVec3AX4::new(x, y, z)
}
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
#[repr(C, align(16))]
pub struct BVec3AX4 {
    pub x: u32,
    pub y: u32,
    pub z: u32,
}

const MASK: [u32; 2] = [0, 0xff_ff_ff_ff];

impl BVec3AX4 {
    #[inline(always)]
    #[must_use]
    pub const fn new(x: [bool; 4], y: [bool; 4], z: [bool; 4]) -> Self {
        Self {
            x: [
                MASK[x[i] as usize],
                MASK[x[i] as usize],
                MASK[x[i] as usize],
                MASK[x[i] as usize],
            ],
            y: [
                MASK[y[i] as usize],
                MASK[y[i] as usize],
                MASK[y[i] as usize],
                MASK[y[i] as usize],
            ],
            z: [
                MASK[z[i] as usize],
                MASK[z[i] as usize],
                MASK[z[i] as usize],
                MASK[z[i] as usize],
            ],
        }
    }
}
