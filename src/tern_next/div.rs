use std::ops::{Div, DivAssign};

use crate::tern_next::Ternary;

#[inline]
pub fn div<const S1: usize, const S2: usize>(_lhs: Ternary<S1>, _rhs: Ternary<S2>) -> Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    todo!()
}

impl<const S1: usize, const S2: usize> Div<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: Ternary<S2>) -> Self::Output {
        div(self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: Ternary<S2>) -> Self::Output {
        div(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: Ternary<S2>) -> Self::Output {
        div(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: &Ternary<S2>) -> Self::Output {
        div(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<&Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: &Ternary<S2>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<&Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: &Ternary<S2>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: &mut Ternary<S2>) -> Self::Output {
        div(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<&mut Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: &mut Ternary<S2>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Div<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn div(self, rhs: &mut Ternary<S2>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> DivAssign<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn div_assign(&mut self, rhs: Ternary<S2>) {
        *self = *self * rhs;
    }
}

impl<const S1: usize, const S2: usize> DivAssign<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn div_assign(&mut self, rhs: &Ternary<S2>) {
        *self = *self * *rhs;
    }
}

impl<const S1: usize, const S2: usize> DivAssign<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn div_assign(&mut self, rhs: &mut Ternary<S2>) {
        *self = *self * *rhs;
    }
}

impl<const S1: usize, const S2: usize> DivAssign<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn div_assign(&mut self, rhs: &mut Ternary<S2>) {
        **self = **self * *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tern_next::Ternary;

    #[test]
    fn tern_divtiplication() {
        let one: Ternary<9> = Ternary::ONE;
        let two: Ternary<9> = Ternary::ONE + Ternary::<9>::ONE;

        let three: Ternary<9> = Ternary::ONE << 1;
        let twenty_seven: Ternary<9> = Ternary::ONE << 3;

        let four = one + three;

        let div = three * three * three;
        assert_eq!(div, twenty_seven);

        let div_add = (three * four * two) + three;
        assert_eq!(div_add, twenty_seven);
    }
}
