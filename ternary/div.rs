use core::ops::{Div, DivAssign};

use crate::Ternary;

#[inline]
pub fn div<const SIZE: usize>(_lhs: Ternary<SIZE>, _rhs: Ternary<SIZE>) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    todo!()
}

impl<const SIZE: usize> Div<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(self, rhs)
    }
}

impl<const SIZE: usize> Div<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(*self, rhs)
    }
}

impl<const SIZE: usize> Div<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(*self, rhs)
    }
}

impl<const SIZE: usize> Div<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(self, *rhs)
    }
}

impl<const SIZE: usize> Div<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const SIZE: usize> Div<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const SIZE: usize> Div<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(self, *rhs)
    }
}

impl<const SIZE: usize> Div<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const SIZE: usize> Div<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs)
    }
}

impl<const SIZE: usize> DivAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self * rhs;
    }
}

impl<const SIZE: usize> DivAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self = *self * *rhs;
    }
}

impl<const SIZE: usize> DivAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self = *self * *rhs;
    }
}

impl<const SIZE: usize> DivAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self = **self * *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Ternary;

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
