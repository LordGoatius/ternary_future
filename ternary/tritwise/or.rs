use core::ops::{BitOr, BitOrAssign};

use crate::Ternary;

// binary and ternary `or` functions like a `max` function on each bit/trit.
#[inline]
pub const fn or<const SIZE: usize>(lhs: Ternary<SIZE>, rhs: Ternary<SIZE>) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let neg = lhs.neg & rhs.neg;
    let pos = lhs.pos | rhs.pos;

    Ternary { pos, neg }
}

impl<const SIZE: usize> const BitOr<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;

    fn bitor(self, rhs: Ternary<SIZE>) -> Self::Output {
        or(self, rhs)
    }
}

impl<const SIZE: usize> const BitOr<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: Ternary<SIZE>) -> Self::Output {
        or(*self, rhs)
    }
}

impl<const SIZE: usize> const BitOr<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: Ternary<SIZE>) -> Self::Output {
        or(*self, rhs)
    }
}

impl<const SIZE: usize> const BitOr<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        or(self, *rhs)
    }
}

impl<const SIZE: usize> const BitOr<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitOr<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: &Ternary<SIZE>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitOr<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        or(self, *rhs)
    }
}

impl<const SIZE: usize> const BitOr<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitOr<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn bitor(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        or(*self, *rhs)
    }
}

impl<const SIZE: usize> const BitOrAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitor_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self | rhs;
    }
}

impl<const SIZE: usize> const BitOrAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitor_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self |= *rhs;
    }
}

impl<const SIZE: usize> const BitOrAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitor_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self |= *rhs;
    }
}

impl<const SIZE: usize> const BitOrAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn bitor_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self |= *rhs;
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
