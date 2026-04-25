use std::ops::{Add, Neg};

/// This data type represents a single ternary number, up to
/// 32 digits in length. It uses a BCT representation where
/// one u32 represents a negative value, and the other half positive.
/// Each bitwise index adds to be the total value of the trit.
/// The first one represents the positive, and the second, negative.
/// We arbitrarily choose 0,0 to be 0, and not 1,1.
/// Based on Frieder & Luk, 1975.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct Ternary<const SIZE: usize>
    where [(); SIZE + (usize::MAX - 32)]:
{
    pos: u32,
    neg: u32,
}

impl<const SIZE: usize> Ternary<SIZE> 
    where [(); SIZE + (usize::MAX - 32)]:
{
}

/// lhs SIZE > rhs SIZE
impl<const S1: usize, const S2: usize> Add<Ternary<S2>> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: Ternary<S2>) -> Self::Output {
        let mut pos = 0;
        let mut neg = 0;

        // By the propery of ME, neg_* and pos_* cannot both be
        // 1. (Frieder & Luk, 1975). This means that
        // neg_t & pos_t = 0.
        todo!()
    }
}

//== Ops ==//

impl<const SIZE: usize> Neg for Ternary<SIZE> 
    where [(); SIZE + (usize::MAX - 32)]:
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self{ pos: self.neg, neg: self.pos }
    }
}

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

    #[test]
    fn test() {}
}
