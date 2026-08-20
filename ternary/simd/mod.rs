//! Algorithms and definitions from (Frieder & Luk, 1975)

use core::simd::Simd;

use crate::Ternary;

mod helper;

/// Like [`crate::Ternary`] but simd
#[repr(C)]
#[derive(Clone, Copy)]
#[derive(Default, Eq, PartialEq)]
pub struct TernarySimd<const SIZE: usize, const N: usize>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    pub(crate) pos: Simd<u32, N>,
    pub(crate) neg: Simd<u32, N>,
}

impl<const SIZE: usize, const N: usize> TernarySimd<SIZE, N>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    // fn to_parts(self) -> [Ternary<SIZE>; N] {
    //     let TernarySimd { pos, neg } = self;
    //     pos.to_array();
    //     neg.to_array();
    //     pos.to
    // }
}
