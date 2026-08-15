use core::ops::{Shr, ShrAssign};

use super::shr;
use crate::Ternary;

const impl<const SIZE: usize> Shr<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(self, rhs)
    }
}

const impl<const SIZE: usize> Shr<isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(*self, rhs)
    }
}

const impl<const SIZE: usize> Shr<isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: isize) -> Self::Output {
        shr(*self, rhs)
    }
}

const impl<const SIZE: usize> Shr<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(self, *rhs)
    }
}

const impl<const SIZE: usize> Shr<&isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

const impl<const SIZE: usize> Shr<&isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

const impl<const SIZE: usize> Shr<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(self, *rhs)
    }
}

const impl<const SIZE: usize> Shr<&mut isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

const impl<const SIZE: usize> Shr<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn shr(self, rhs: &mut isize) -> Self::Output {
        shr(*self, *rhs)
    }
}

const impl<const SIZE: usize> ShrAssign<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: isize) {
        *self = *self >> rhs;
    }
}

const impl<const SIZE: usize> ShrAssign<&isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &isize) {
        *self = *self >> *rhs;
    }
}

const impl<const SIZE: usize> ShrAssign<&mut isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &mut isize) {
        *self = *self >> *rhs;
    }
}

const impl<const SIZE: usize> ShrAssign<&mut isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn shr_assign(&mut self, rhs: &mut isize) {
        **self = **self >> *rhs;
    }
}
