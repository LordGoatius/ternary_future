use core::ops::{Div, DivAssign};
use core::ops::Rem;
use core::ops::RemAssign;

use crate::Ternary;

#[inline]
pub fn div<const SIZE: usize>(lhs: Ternary<SIZE>, rhs: Ternary<SIZE>) -> Option<(Ternary<SIZE>, Ternary<SIZE>)>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    // TODO: I give up on division for now
    // I hope to never have to implement division ever again
    if rhs == Ternary::ZERO {
        None
    } else {
        // This is my greatest shame
        let val1: isize = lhs.into();
        let val2: isize = rhs.into();
        let q = val1.div_euclid(val2).try_into().expect("Couldn't convert into ternary");
        let r = val1.rem_euclid(val2).try_into().expect("Couldn't convert into ternary");
        Some((q, r))
    }
}

impl<const SIZE: usize> Div<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(self, rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(*self, rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(*self, rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(self, *rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(self, *rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> Div<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn div(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").0
    }
}

impl<const SIZE: usize> DivAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self / rhs;
    }
}

impl<const SIZE: usize> DivAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self /= *rhs;
    }
}

impl<const SIZE: usize> DivAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self /= *rhs;
    }
}

impl<const SIZE: usize> DivAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn div_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self /= *rhs;
    }
}

impl<const SIZE: usize> Rem<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(self, rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(*self, rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: Ternary<SIZE>) -> Self::Output {
        div(*self, rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(self, *rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: &Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(self, *rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> Rem<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn rem(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        div(*self, *rhs).expect("Cannot divide by 0").1
    }
}

impl<const SIZE: usize> RemAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn rem_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self % rhs;
    }
}

impl<const SIZE: usize> RemAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn rem_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self %= *rhs;
    }
}

impl<const SIZE: usize> RemAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn rem_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self %= *rhs;
    }
}

impl<const SIZE: usize> RemAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn rem_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self %= *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Ternary;

    // TODO: Better div test lol
    #[test]
    fn tern_division() {
        let three: Ternary<9> = Ternary::ONE << 1;
        let nine: Ternary<9> = Ternary::ONE << 2;
        let twenty_seven: Ternary<9> = Ternary::ONE << 3;

        assert_eq!(nine, twenty_seven / three);
        assert_eq!(three, nine / three);

    }
}
