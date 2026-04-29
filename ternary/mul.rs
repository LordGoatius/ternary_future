use core::ops::{Mul, MulAssign};

use crate::Ternary;

#[inline]
pub fn mul<const SIZE: usize>(lhs: Ternary<SIZE>, rhs: Ternary<SIZE>) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let mut acc: Ternary<SIZE> = Ternary::ZERO;
    // Iterate over every trit in `rhs`
    for i in 0..SIZE {
        acc += match (((rhs.pos >> i) & 1), ((rhs.neg >> i) & 1)) {
            (1, 0) => {
                let Ternary { pos, neg } = lhs;
                Ternary { pos, neg }
            }
            (0, 1) => {
                let Ternary { pos, neg } = lhs;
                -Ternary { pos, neg }
            }
            _ => Ternary::<SIZE>::ZERO,
        } << (i as isize);
    }
    acc
}

impl<const SIZE: usize> Mul<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: Ternary<SIZE>) -> Self::Output {
        mul(self, rhs)
    }
}

impl<const SIZE: usize> Mul<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: Ternary<SIZE>) -> Self::Output {
        mul(*self, rhs)
    }
}

impl<const SIZE: usize> Mul<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: Ternary<SIZE>) -> Self::Output {
        mul(*self, rhs)
    }
}

impl<const SIZE: usize> Mul<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: &Ternary<SIZE>) -> Self::Output {
        mul(self, *rhs)
    }
}

impl<const SIZE: usize> Mul<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: &Ternary<SIZE>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const SIZE: usize> Mul<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: &Ternary<SIZE>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const SIZE: usize> Mul<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        mul(self, *rhs)
    }
}

impl<const SIZE: usize> Mul<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const SIZE: usize> Mul<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn mul(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const SIZE: usize> MulAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn mul_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self * rhs;
    }
}

impl<const SIZE: usize> MulAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn mul_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self = *self * *rhs;
    }
}

impl<const SIZE: usize> MulAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn mul_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self = *self * *rhs;
    }
}

impl<const SIZE: usize> MulAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn mul_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self = **self * *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Ternary;

    #[test]
    fn tern_multiplication() {
        let one: Ternary<9> = Ternary::ONE;
        let two: Ternary<9> = Ternary::ONE + Ternary::<9>::ONE;

        let three: Ternary<9> = Ternary::ONE << 1;
        let twenty_seven: Ternary<9> = Ternary::ONE << 3;

        let four = one + three;

        let mul = three * three * three;
        assert_eq!(mul, twenty_seven);

        let mul_add = (three * four * two) + three;
        assert_eq!(mul_add, twenty_seven);
    }
}
