use std::ops::{Mul, MulAssign};

use crate::tern_next::Ternary;

#[inline]
pub fn mul<const S1: usize, const S2: usize>(lhs: Ternary<S1>, rhs: Ternary<S2>) -> Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    let mut acc: Ternary<S1> = Ternary::ZERO;
    // Iterate over every trit in `rhs`
    for i in 0..S2 {
        acc += match (((rhs.pos >> i) & 1), ((rhs.neg >> i) & 1)) {
            (1, 0) => {
                let Ternary { pos, neg } = lhs;
                Ternary { pos, neg }
            }
            (0, 1) => {
                let Ternary { pos, neg } = lhs;
                -Ternary { pos, neg }
            }
            _ => Ternary::<S2>::ZERO,
        } << (i as isize);
    }
    acc
}

impl<const S1: usize, const S2: usize> Mul<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: Ternary<S2>) -> Self::Output {
        mul(self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: Ternary<S2>) -> Self::Output {
        mul(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: Ternary<S2>) -> Self::Output {
        mul(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: &Ternary<S2>) -> Self::Output {
        mul(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<&Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: &Ternary<S2>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<&Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: &Ternary<S2>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: &mut Ternary<S2>) -> Self::Output {
        mul(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<&mut Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: &mut Ternary<S2>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Mul<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn mul(self, rhs: &mut Ternary<S2>) -> Self::Output {
        mul(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> MulAssign<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn mul_assign(&mut self, rhs: Ternary<S2>) {
        *self = *self * rhs;
    }
}

impl<const S1: usize, const S2: usize> MulAssign<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn mul_assign(&mut self, rhs: &Ternary<S2>) {
        *self = *self * *rhs;
    }
}

impl<const S1: usize, const S2: usize> MulAssign<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn mul_assign(&mut self, rhs: &mut Ternary<S2>) {
        *self = *self * *rhs;
    }
}

impl<const S1: usize, const S2: usize> MulAssign<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn mul_assign(&mut self, rhs: &mut Ternary<S2>) {
        **self = **self * *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tern_next::Ternary;

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
