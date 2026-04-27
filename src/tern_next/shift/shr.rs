use std::ops::{Shr, ShrAssign};

use crate::tern_next::Ternary;
use super::shr;

impl<const S1: usize> Shr<isize> for Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(self, rhs)
    }
}

impl<const S1: usize> Shr<isize> for &Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(*self, rhs)
    }
}

impl<const S1: usize> Shr<isize> for &mut Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(*self, rhs)
    }
}

impl<const S1: usize> Shr<&isize> for Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(self, *rhs)
    }
}

impl<const S1: usize> Shr<&isize> for &Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const S1: usize> Shr<&isize> for &mut Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const S1: usize> Shr<&mut isize> for Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(self, *rhs)
    }
}

impl<const S1: usize> Shr<&mut isize> for &Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const S1: usize> Shr<&mut isize> for &mut Ternary<S1>
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    type Output = Ternary<S1>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

impl<const SIZE: usize> ShrAssign<isize> for Ternary<SIZE> 
    where
        [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: isize) {
        *self = *self >> rhs;
    }
}

impl<const S1: usize> ShrAssign<&isize> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &isize) {
        *self = *self >> *rhs;
    }
}

impl<const S1: usize> ShrAssign<&mut isize> for Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &mut isize) {
        *self = *self >> *rhs;
    }
}

impl<const S1: usize> ShrAssign<&mut isize> for &mut Ternary<S1> 
    where
        [(); S1 + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &mut isize) {
        **self = **self >> *rhs;
    }
}
