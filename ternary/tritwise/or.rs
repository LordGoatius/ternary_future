use std::ops::{BitOr, BitOrAssign};

use crate::Ternary;

// binary and ternary `or` functions like a `max` function on each bit/trit.
#[inline]
pub const fn or<const S1: usize, const S2: usize>(lhs: Ternary<S1>, rhs: Ternary<S2>) -> Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    let neg = lhs.neg & rhs.neg;
    let pos = lhs.pos | rhs.pos;

    Ternary { pos, neg }
}

impl<const S1: usize, const S2: usize> BitOr<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;

    fn bitor(self, rhs: Ternary<S2>) -> Self::Output {
        or(self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: Ternary<S2>) -> Self::Output {
        or(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: Ternary<S2>) -> Self::Output {
        or(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: &Ternary<S2>) -> Self::Output {
        or(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<&Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: &Ternary<S2>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<&Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: &Ternary<S2>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: &mut Ternary<S2>) -> Self::Output {
        or(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<&mut Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: &mut Ternary<S2>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOr<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn bitor(self, rhs: &mut Ternary<S2>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> BitOrAssign<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitor_assign(&mut self, rhs: Ternary<S2>) {
        *self = *self & rhs;
    }
}

impl<const S1: usize, const S2: usize> BitOrAssign<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitor_assign(&mut self, rhs: &Ternary<S2>) {
        *self = *self & *rhs;
    }
}

impl<const S1: usize, const S2: usize> BitOrAssign<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitor_assign(&mut self, rhs: &mut Ternary<S2>) {
        *self = *self & *rhs;
    }
}

impl<const S1: usize, const S2: usize> BitOrAssign<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn bitor_assign(&mut self, rhs: &mut Ternary<S2>) {
        **self = **self & *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use super::or;
    use crate::Ternary;

    #[test]
    fn tern_or() {
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
            pos: 0b11111010,
            neg: 0b00000100,
        };
        eprintln!("{c:b}");
        assert_eq!(or(a, b), c)
    }
}
