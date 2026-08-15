use core::ops::{Sub, SubAssign};

use crate::Ternary;
use crate::helper::add;

#[inline]
pub const fn sub<const SIZE: usize>(lhs: Ternary<SIZE>, rhs: Ternary<SIZE>) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    add(lhs, -rhs)
}

const impl<const SIZE: usize> Sub<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: Ternary<SIZE>) -> Self::Output {
        sub(self, rhs)
    }
}

const impl<const SIZE: usize> Sub<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: Ternary<SIZE>) -> Self::Output {
        sub(*self, rhs)
    }
}

const impl<const SIZE: usize> Sub<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: Ternary<SIZE>) -> Self::Output {
        sub(*self, rhs)
    }
}

const impl<const SIZE: usize> Sub<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: &Ternary<SIZE>) -> Self::Output {
        sub(self, *rhs)
    }
}

const impl<const SIZE: usize> Sub<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: &Ternary<SIZE>) -> Self::Output {
        sub(*self, *rhs)
    }
}

const impl<const SIZE: usize> Sub<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: &Ternary<SIZE>) -> Self::Output {
        sub(*self, *rhs)
    }
}

const impl<const SIZE: usize> Sub<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        sub(self, *rhs)
    }
}

const impl<const SIZE: usize> Sub<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        sub(*self, *rhs)
    }
}

const impl<const SIZE: usize> Sub<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn sub(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        sub(*self, *rhs)
    }
}

const impl<const SIZE: usize> SubAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn sub_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self - rhs;
    }
}

const impl<const SIZE: usize> SubAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn sub_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self -= *rhs;
    }
}

const impl<const SIZE: usize> SubAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn sub_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self -= *rhs;
    }
}

const impl<const SIZE: usize> SubAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn sub_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self -= *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Ternary;

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
