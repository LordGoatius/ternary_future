use std::ops::{Sub, SubAssign};

use crate::tern_next::Ternary;
use crate::helper::add;

#[inline]
pub fn sub<const S1: usize, const S2: usize>(lhs: Ternary<S1>, rhs: Ternary<S2>) -> Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    add(lhs, -rhs)
}

impl<const S1: usize, const S2: usize> Sub<Ternary<S2>> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: Ternary<S2>) -> Self::Output {
        sub(self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<Ternary<S2>> for &Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: Ternary<S2>) -> Self::Output {
        sub(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<Ternary<S2>> for &mut Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: Ternary<S2>) -> Self::Output {
        sub(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<&Ternary<S2>> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: &Ternary<S2>) -> Self::Output {
        sub(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<&Ternary<S2>> for &Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: &Ternary<S2>) -> Self::Output {
        sub(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<&Ternary<S2>> for &mut Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: &Ternary<S2>) -> Self::Output {
        sub(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<&mut Ternary<S2>> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: &mut Ternary<S2>) -> Self::Output {
        sub(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<&mut Ternary<S2>> for &Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: &mut Ternary<S2>) -> Self::Output {
        sub(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Sub<&mut Ternary<S2>> for &mut Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn sub(self, rhs: &mut Ternary<S2>) -> Self::Output {
        sub(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> SubAssign<Ternary<S2>> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    fn sub_assign(&mut self, rhs: Ternary<S2>) {
        *self = *self + rhs;
    }
}

impl<const S1: usize, const S2: usize> SubAssign<&Ternary<S2>> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    fn sub_assign(&mut self, rhs: &Ternary<S2>) {
        *self = *self + *rhs;
    }
}

impl<const S1: usize, const S2: usize> SubAssign<&mut Ternary<S2>> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    fn sub_assign(&mut self, rhs: &mut Ternary<S2>) {
        *self = *self + *rhs;
    }
}

impl<const S1: usize, const S2: usize> SubAssign<&mut Ternary<S2>> for &mut Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
        [(); S2 + (usize::MAX - 32)]:,
        [(); S1 - S2]:,
{
    fn sub_assign(&mut self, rhs: &mut Ternary<S2>) {
        **self = **self + *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tern_next::Ternary;

    #[test]
    fn tern_substitution() {
        let mut ternary: Ternary<9> = Ternary::MAX;

        let tone: Ternary<9> = Ternary::ONE;

        // I know this looks redundant and stupid but I like it.
        let mut binary = 9841;
        let bone = 1isize;

        for _ in 0..(3usize.pow(9)) {
            assert_eq!(binary, ternary.into());
            println!("0b{:b}, 0t{:b}", binary, ternary);
            ternary = ternary - tone;
            binary = binary - bone;
        }
    }
}
