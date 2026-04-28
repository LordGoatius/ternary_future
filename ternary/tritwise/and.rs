use std::ops::{BitAnd, BitAndAssign};

use crate::Ternary;

// binary and ternary `and` functions like a `min` function on each bit/trit.
#[inline]
pub const fn and<const S1: usize, const S2: usize>(
    lhs: Ternary<S1>,
    rhs: Ternary<S2>,
) -> Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    let neg = lhs.neg | rhs.neg;
    let pos = lhs.pos & rhs.pos;

    Ternary { pos, neg }
}

impl<const S1: usize, const S2: usize> BitAnd<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;

    fn bitand(self, rhs: Ternary<S2>) -> Self::Output {
        and(self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: Ternary<S2>) -> Self::Output {
        and(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: Ternary<S2>) -> Self::Output {
        and(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: &Ternary<S2>) -> Self::Output {
        and(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<&Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: &Ternary<S2>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<&Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: &Ternary<S2>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: &mut Ternary<S2>) -> Self::Output {
        and(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<&mut Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: &mut Ternary<S2>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAnd<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitand(self, rhs: &mut Ternary<S2>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitAndAssign<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitand_assign(&mut self, rhs: Ternary<S2>) {
        *self = *self & rhs;
    }
}

impl<const S1: usize, const S2: usize> BitAndAssign<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitand_assign(&mut self, rhs: &Ternary<S2>) {
        *self = *self & *rhs;
    }
}

impl<const S1: usize, const S2: usize> BitAndAssign<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitand_assign(&mut self, rhs: &mut Ternary<S2>) {
        *self = *self & *rhs;
    }
}

impl<const S1: usize, const S2: usize> BitAndAssign<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitand_assign(&mut self, rhs: &mut Ternary<S2>) {
        **self = **self & *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use super::and;
    use crate::Ternary;

    #[test]
    fn tern_and() {
        let a: Ternary<9> = Ternary {
            pos: 0b11001010,
            neg: 0b00100101,
        };
        let b: Ternary<9> = Ternary {
            pos: 0b00110000,
            neg: 0b11001100,
        };
        eprintln!("{a:b}");
        eprintln!("{b:b}");

        let c: Ternary<9> = Ternary {
            pos: 0b00000000,
            neg: 0b11101101,
        };
        eprintln!("{c:b}");
        assert_eq!(and(a, b), c)
    }
}
