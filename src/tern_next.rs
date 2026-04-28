use std::fmt::{Binary, Debug, Display};

mod add;
mod mul;
mod shift;
mod sub;
mod tritwise;

/// This data type represents a single ternary number, up to
/// 32 digits in length. It uses a BCT representation where
/// one u32 represents a negative value, and the other half positive.
/// Each bitwise index adds to be the total value of the trit.
/// The first one represents the positive, and the second, negative.
/// We choose 0,0 to be 0, and not 1,1.
/// Based on Frieder & Luk, 1975.
///
// TODO: Do we want to ensure that all bits at locations >= SIZE are 0?
// This is likely what we want? It *does* affect things *if* we allow conversion
// and operations
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

    pub const ONE: Self = Ternary { pos: 0b1, neg: 0 };
}

//== Easy Helper Traits ==//

impl<const SIZE: usize> Into<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
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
where
    [(); SIZE + (usize::MAX - 32)]:,
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

impl<const SIZE: usize> Into<isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
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
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: isize = self.into();
        write!(f, "{value}")
    }
}

impl<const SIZE: usize> Debug for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Ternary<{SIZE}>"))
            .field_with("pos", |f| write!(f, "{:b}", &self.pos))
            .field_with("neg", |f| write!(f, "{:b}", &self.neg))
            .finish()
    }
}

impl<const SIZE: usize> Binary for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Buffer prints all right now. Change to take in formatting options?
        let mut buf: [u8; SIZE] = [b'0'; SIZE];
        let Ternary { pos, neg } = self;
        for i in (0..SIZE).rev() {
            match ((pos >> i) & 1, (neg >> i) & 1) {
                (0, 1) => buf[SIZE - i - 1] = b'T',
                (1, 0) => buf[SIZE - i - 1] = b'1',
                _ => (),
            }
        }
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&buf) })
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
