use core::ops::{Div, DivAssign};

use crate::Ternary;

#[inline]
pub fn div<const SIZE: usize>(lhs: Ternary<SIZE>, rhs: Ternary<SIZE>) -> Option<(Ternary<SIZE>, Ternary<SIZE>)>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    // God I hope this works because ternary division is a pain in the ass.
    // Also this is only for positive numbers oops.
    // Can at least be used to add and remove signs if necessary, which is easy with ternary.
    let (n, d) = (lhs, rhs);
    if d == Ternary::ZERO { return None; }
    let mut q = Ternary::ZERO;
    let mut r = Ternary::MIN;
    for i in (0..SIZE).rev() {
        r <<= 1;
        r.set_trit(n.get_trit(i), i);
        if r >= d {
            r = r - d;
            q.set_trit(Ternary::<1>::ONE, i);
        }
    }
    Some((q, r))
    // if D = 0 then error(DivisionByZeroException) end
    // Q := 0                  -- Initialize quotient and remainder to zero
    // R := 0
    // for i := n − 1 .. 0 do  -- Where n is number of bits in N
    //   R := R << 1           -- Left-shift R by 1 bit
    //   R(0) := N(i)          -- Set the least-significant bit of R equal to bit i of the numerator
    //   if R ≥ D then
    //     R := R − D
    //     Q(i) := 1
    //   end
    // end
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
