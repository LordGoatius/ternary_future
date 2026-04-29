#![expect(incomplete_features)]
#![feature(
    generic_const_exprs,
    const_default,
    derive_const,
    const_trait_impl,
    debug_closure_helpers
)]
#![cfg_attr(not(test), no_std)]

mod helper;
mod add;
mod mul;
mod sub;
mod div;
mod shift;
mod tritwise;
mod convert;
mod display;

/// This data type represents a single ternary number, up to
/// 32 digits in length. It uses a BCT representation where
/// one u32 represents a negative value, and the other half positive.
/// Each bitwise index adds to be the total value of the trit.
/// The first one represents the positive, and the second, negative.
/// We choose 0,0 to be 0, and not 1,1.
/// Based on Frieder & Luk, 1975.
#[repr(C)]
#[derive(Clone, Copy, Eq, PartialEq)]
#[derive_const(Default)]
pub struct Ternary<const SIZE: usize>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    pub(crate) pos: u32,
    pub(crate) neg: u32,
}

impl<const SIZE: usize> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    pub const MAX: Self = {
        let pos = (1 << SIZE) - 1;
        let neg = 0;
        Ternary { pos, neg }
    };

    pub const MIN: Self = {
        let pos = 0;
        let neg = (1 << SIZE) - 1;
        Ternary { pos, neg }
    };

    pub const ZERO: Self = Ternary { pos: 0, neg: 0 };

    pub const ONE: Self = Ternary { pos:  0b1, neg:  0b0 };
    pub const TWO: Self = Ternary { pos: 0b10, neg: 0b01 };

    pub fn pow(self, val: u32) -> Self {
        let mut ret = Ternary::ONE;
        for _ in 0..val {
            ret *= self;
        }
        ret
    }
}

#[cfg(test)]
pub mod tests {
    /// Used to toggle compile time tests. If I let it compile,
    /// everything yells at me and I can't say `should_panic`
    /// because it doesn't panic. but I want to know it works.
    ///
    /// If this code compiles, `rustc` will error.
    #[cfg(feature = "compile_fail")]
    mod _compile_tests {
        use crate::tern_next::Ternary;
        #[test]
        fn _test1() {
            let _ = Ternary::<33>(0, 0);
        }

        fn _test2() {
            let t1 = Ternary::<32>(0, 0);
            let t2 = Ternary::<15>(0, 0);
            t2 + t1
        }
    }
}
