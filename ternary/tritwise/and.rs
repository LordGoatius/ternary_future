use core::ops::{BitAnd, BitAndAssign};

use crate::Ternary;

// binary and ternary `and` functions like a `min` function on each bit/trit.
#[inline]
pub const fn and<const SIZE: usize>(
    lhs: Ternary<SIZE>,
    rhs: Ternary<SIZE>,
) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let neg = lhs.neg | rhs.neg;
    let pos = lhs.pos & rhs.pos;

    Ternary { pos, neg }
}

impl<const SIZE: usize> const BitAnd<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;

    fn bitand(self, rhs: Ternary<SIZE>) -> Self::Output {
        and(self, rhs)
    }
}

impl<const SIZE: usize> const BitAnd<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: Ternary<SIZE>) -> Self::Output {
        and(*self, rhs)
    }
}

impl<const SIZE: usize> const BitAnd<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: Ternary<SIZE>) -> Self::Output {
        and(*self, rhs)
    }
}

impl<const SIZE: usize> const BitAnd<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: &Ternary<SIZE>) -> Self::Output {
        and(self, *rhs)
    }
}

impl<const SIZE: usize> const BitAnd<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: &Ternary<SIZE>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitAnd<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: &Ternary<SIZE>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitAnd<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        and(self, *rhs)
    }
}

impl<const SIZE: usize> const BitAnd<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitAnd<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitand(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        and(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitAndAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitand_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self & rhs;
    }
}

impl<const SIZE: usize> const BitAndAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitand_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self &= *rhs;
    }
}

impl<const SIZE: usize> const BitAndAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitand_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self &= *rhs;
    }
}

impl<const SIZE: usize> const BitAndAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitand_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self &= *rhs;
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
