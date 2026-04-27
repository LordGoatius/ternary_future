use std::{fmt::{Binary, Debug, Display}, ops::{Mul, Neg, Shl, Shr, Sub}};

use crate::helper::neg;
use crate::helper::add;

mod add;

/// This data type represents a single ternary number, up to
/// 32 digits in length. It uses a BCT representation where
/// one u32 represents a negative value, and the other half positive.
/// Each bitwise index adds to be the total value of the trit.
/// The first one represents the positive, and the second, negative.
/// We arbitrarily choose 0,0 to be 0, and not 1,1.
/// Based on Frieder & Luk, 1975.
///
// TODO: Do we want to ensure that all bits at locations >= SIZE are 0?
// This is likely what we want.
#[repr(C)]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[derive_const(Default)]
pub struct Ternary<const SIZE: usize>
    where [(); SIZE + (usize::MAX - 32)]:
{
    pub(crate) pos: u32,
    pub(crate) neg: u32,
}

impl<const SIZE: usize> Ternary<SIZE> 
    where [(); SIZE + (usize::MAX - 32)]:
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

    pub const ONE: Self = Ternary { pos: 0b1, neg: 0 };
}

//== Ops ==//

impl<const S1: usize> Shl<isize> for Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shl(self, rhs: isize) -> Self::Output {
        let Ternary { pos, neg } = self;
        let pos = pos << rhs;
        let neg = neg << rhs;
        Ternary { pos, neg }
    }
}

impl<const S1: usize> Shr<isize> for Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: isize) -> Self::Output {
        let Ternary { pos, neg } = self;
        let pos = pos >> rhs;
        let neg = neg >> rhs;
        Ternary { pos, neg }
    }
}

impl<const S1: usize, const S2: usize> Sub<Ternary<S2>> for Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: Ternary<S2>) -> Self::Output {
        add(self, -rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<Ternary<S2>> for Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: Ternary<S2>) -> Self::Output {
        let mut acc: Ternary<S1> = Ternary::ZERO;
        // Iterate over every trit in `rhs`
        for i in 0..S2 {
            acc += match (((rhs.pos >> i) & 1), ((rhs.neg >> i) & 1)) {
                (1, 0) =>  {
                    let Ternary { pos, neg } = self;
                    Ternary { pos, neg }
                },
                (0, 1) => {
                    let Ternary { pos, neg } = self;
                    -Ternary { pos, neg }
                },
                _      => Ternary::<S2>::ZERO,
            } << (i as isize);

        }
        acc
    }
}

impl<const SIZE: usize> Neg for Ternary<SIZE> 
    where [(); SIZE + (usize::MAX - 32)]:
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        neg(self)
    }
}

//== Helper Traits ==//

impl<const SIZE: usize> Into<isize> for Ternary<SIZE>
    where [(); SIZE + (usize::MAX - 32)]:
{
    fn into(self) -> isize {
        let mut sum = 0;
        for i in 0..SIZE {
           sum += ((self.pos >> i) & 1) as isize * 3isize.pow(i as u32); 
           sum -= ((self.neg >> i) & 1) as isize * 3isize.pow(i as u32); 
        }
        sum
    }
}

impl<const SIZE: usize> Into<isize> for &Ternary<SIZE>
    where [(); SIZE + (usize::MAX - 32)]:
{
    fn into(self) -> isize {
        let mut sum = 0;
        for i in 0..SIZE {
           sum += ((self.pos >> i) & 1) as isize * 3isize.pow(i as u32); 
           sum -= ((self.neg >> i) & 1) as isize * 3isize.pow(i as u32); 
        }
        sum
    }
}

impl<const SIZE: usize> Display for Ternary<SIZE>
    where [(); SIZE + (usize::MAX - 32)]:
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: isize = self.into();
        write!(f, "{value}")
    }
}

impl<const SIZE: usize> Binary for Ternary<SIZE>
    where [(); SIZE + (usize::MAX - 32)]:
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Buffer prints all right now. Change to take in formatting options?
        let mut buf: [u8; SIZE] = [b'0'; SIZE];
        let Ternary { pos, neg } = self;
        for i in (0..SIZE).rev() {
            match ((pos >> i) & 1, (neg >> i) & 1) {
                (0, 1) => { buf[SIZE - i - 1] = b'T' }
                (1, 0) => { buf[SIZE - i - 1] = b'1' }
                _ => ()
            }
        }
        write!(f, "{}", unsafe {
            str::from_utf8_unchecked(&buf)
        })
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tern_next::Ternary;

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

    #[test]
    fn tern_addition() {
        let mut ternary: Ternary<9> = Ternary::MIN;

        let tone: Ternary<9> = Ternary::ONE;

        // I know this looks redundant and stupid but I like it.
        let mut binary = -9841;
        let bone = 1isize;

        for _ in 0..(3usize.pow(9)) {
            assert_eq!(binary, ternary.into());
            println!("0b{:b}, 0t{:b}", binary, ternary);
            ternary = ternary + tone;
            binary = binary + bone;
        }
    }
}
