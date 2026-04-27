use std::ops::{Add, AddAssign};

use crate::helper::add;
use crate::tern_next::Ternary;

impl<const S1: usize, const S2: usize> Add<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: Ternary<S2>) -> Self::Output {
        add(self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: Ternary<S2>) -> Self::Output {
        add(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: Ternary<S2>) -> Self::Output {
        add(*self, rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: &Ternary<S2>) -> Self::Output {
        add(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<&Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: &Ternary<S2>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<&Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: &Ternary<S2>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: &mut Ternary<S2>) -> Self::Output {
        add(self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<&mut Ternary<S2>> for &Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: &mut Ternary<S2>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> Add<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    type Output = Ternary<S1>;
    fn add(self, rhs: &mut Ternary<S2>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const S1: usize, const S2: usize> AddAssign<Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn add_assign(&mut self, rhs: Ternary<S2>) {
        *self = *self + rhs;
    }
}

impl<const S1: usize, const S2: usize> AddAssign<&Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn add_assign(&mut self, rhs: &Ternary<S2>) {
        *self = *self + *rhs;
    }
}

impl<const S1: usize, const S2: usize> AddAssign<&mut Ternary<S2>> for Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn add_assign(&mut self, rhs: &mut Ternary<S2>) {
        *self = *self + *rhs;
    }
}

impl<const S1: usize, const S2: usize> AddAssign<&mut Ternary<S2>> for &mut Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    fn add_assign(&mut self, rhs: &mut Ternary<S2>) {
        **self = **self + *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tern_next::Ternary;

    #[test]
    fn tern_addition() {
        let mut ternary: Ternary<9> = Ternary::MIN;

        let tone: Ternary<9> = Ternary::ONE;

        // I know this looks redundant and stupid but I like it.
        let mut binary = -9841;
        let bone = 1isize;

        for _ in 0..(3usize.pow(9)) {
            assert_eq!(binary, ternary.into());
            println!("0b{:b}, 0t{:b}", binary, ternary);
            ternary = ternary + tone;
            binary = binary + bone;
        }
    }
}
