#![expect(incomplete_features)]
#![feature(
    generic_const_exprs,
    const_default,
    derive_const,
    const_trait_impl,
    debug_closure_helpers,
    const_ops,
    const_cmp,
    const_convert,
)]
#![cfg_attr(not(test), no_std)]

#![cfg_attr(test, feature(test))]

#![warn(clippy::missing_const_for_fn)]

use core::cmp::Ordering;

mod helper;
mod add;
mod mul;
mod sub;
mod div;
mod shift;
mod tritwise;
mod convert;
mod display;
mod ord;

pub type Trit = Ternary<1>;

/// This data type represents a single ternary number, up to
/// 32 digits in length. It uses a BCT representation where
/// one u32 represents a negative value, and the other half positive.
/// Each bitwise index adds to be the total value of the trit.
/// The first one represents the positive, and the second, negative.
/// We choose 0,0 to be 0, and not 1,1.
/// Based on Frieder & Luk, 1975.
#[repr(C)]
#[derive(Clone, Copy)]
#[derive_const(Default, Eq, PartialEq)]
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

    pub const fn pow(self, val: u32) -> Self {
        let mut ret = Ternary::ONE;
        let mut i = 0;
        while i < val {
            ret *= self;
            i += 1;
        }
        ret
    }

    pub const fn get_trit(self, index: usize) -> Trit {
        let Ternary { pos, neg } = self;
        let pos = (pos >> index) & 1;
        let neg = (neg >> index) & 1;
        Ternary::<1> { pos, neg }
    }

    pub const fn set_trit(&mut self, trit: Trit, index: usize) {
        assert!(trit.pos.leading_zeros() >= 31);
        assert!(trit.neg.leading_zeros() >= 31);
        let Ternary { mut pos, mut neg } = *self;
        let mask = !(1 << index);
        pos &= mask;
        neg &= mask;
        pos |= trit.pos << index;
        neg |= trit.neg << index;
        *self = Ternary { pos, neg }
    }

    pub(crate) const fn sign_innner(self) -> Ordering {
        let Ternary { pos, neg } = self;
        pos.cmp(&neg)
    }

    pub const fn sign(self) -> Trit {
        match self.sign_innner() {
            Ordering::Less => -Trit::ONE,
            Ordering::Equal => Trit::ZERO,
            Ordering::Greater => Trit::ONE,
        }
    }

    pub const fn from_str(str: &str) -> Self {
        let mut val = Ternary::ZERO;
        let bytes = str.as_bytes();
        let mut i = 0;
        let len = str.len();
        let mut char;
        while i < len {
            char = bytes[i];
            match char {
                b'0' => (),
                b'1' => val.set_trit(Trit::ONE, len - 1- i),
                b'T' | b't' => val.set_trit(-Trit::ONE, len - 1- i),
                _ => panic!("Invalid char in ternary conversion")
            }
            i += 1;
        }
        val
    }

    #[inline]
    pub const fn slice<const S2: usize>(self, index: u32) -> Ternary<S2>
    where
        [(); S2 + (usize::MAX - 32)]:,
    {
        let Ternary { pos, neg } = self;
        let mask = (1 << S2) - 1;
        let pos = (pos >> index) & mask;
        let neg = (neg >> index) & mask;
        Ternary { pos, neg }
    }

    #[inline]
    pub const fn extend<const S2: usize>(self) -> Ternary<S2>
    where
        [(); S2 + (usize::MAX - 32)]:,
    {
        let Ternary { pos, neg } = self;
        Ternary { pos, neg }
    }

    /// Set `S2` trits of `self` to `val`, starting at `index`
    #[inline]
    pub const fn set<const S2: usize>(self, val: Ternary<S2>, index: u32) -> Self
    where
        [(); S2 + (usize::MAX - 32)]:,
    {
        let Ternary { pos, neg } = self;
        let Ternary { pos: posval, neg: negval } = val;

        let mask = ((1 << S2) - 1) << index;
        // Erase bits
        let pos = pos & !mask;
        let neg = neg & !mask;

        // Set bits
        let pos = pos | ((posval & (mask >> index)) << index);
        let neg = neg | ((negval & (mask >> index)) << index);

        Ternary { pos, neg }
    }
}


pub const fn concat<const S1: usize, const S2: usize, const S3: usize>(val1: Ternary<S1>, val2: Ternary<S2>) -> Ternary<{ S1 + S2 }>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); (S1 + S2) + (usize::MAX - 32)]:,
{
    let Ternary { pos: pos1, neg: neg1 } = val1;
    let Ternary { pos: pos2, neg: neg2 } = val2;

    let pos = (pos1 << S2) | (pos2 & ((1 << S2) - 1));
    let neg = (neg1 << S2) | (neg2 & ((1 << S2) - 1));

    Ternary { pos, neg }
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
