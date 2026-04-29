use core::ops::{Add, AddAssign};

use crate::helper::add;
use super::Ternary;

impl<const SIZE: usize> Add<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: Ternary<SIZE>) -> Self::Output {
        add(self, rhs)
    }
}

impl<const SIZE: usize> Add<Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: Ternary<SIZE>) -> Self::Output {
        add(*self, rhs)
    }
}

impl<const SIZE: usize> Add<Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: Ternary<SIZE>) -> Self::Output {
        add(*self, rhs)
    }
}

impl<const SIZE: usize> Add<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: &Ternary<SIZE>) -> Self::Output {
        add(self, *rhs)
    }
}

impl<const SIZE: usize> Add<&Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: &Ternary<SIZE>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const SIZE: usize> Add<&Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: &Ternary<SIZE>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const SIZE: usize> Add<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        add(self, *rhs)
    }
}

impl<const SIZE: usize> Add<&mut Ternary<SIZE>> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const SIZE: usize> Add<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Ternary<SIZE>;
    fn add(self, rhs: &mut Ternary<SIZE>) -> Self::Output {
        add(*self, *rhs)
    }
}

impl<const SIZE: usize> AddAssign<Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn add_assign(&mut self, rhs: Ternary<SIZE>) {
        *self = *self + rhs;
    }
}

impl<const SIZE: usize> AddAssign<&Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn add_assign(&mut self, rhs: &Ternary<SIZE>) {
        *self = *self + *rhs;
    }
}

impl<const SIZE: usize> AddAssign<&mut Ternary<SIZE>> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn add_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        *self = *self + *rhs;
    }
}

impl<const SIZE: usize> AddAssign<&mut Ternary<SIZE>> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn add_assign(&mut self, rhs: &mut Ternary<SIZE>) {
        **self = **self + *rhs;
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Ternary;
    extern crate test;

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

    #[bench]
    fn tern_bench(b: &mut test::Bencher) {
        let mut ternary: Ternary<9> = Ternary::MIN;

        let tone: Ternary<9> = Ternary::ONE;

        b.iter(|| {
            for _ in 0..(3usize.pow(9)) {
                ternary = core::hint::black_box(ternary + tone);
            }
        });

        println!("0t{ternary:b}")
    }

    #[bench]
    fn bin_bench_cmp(b: &mut test::Bencher) {
        let mut binary = -9841;

        b.iter(|| {
            for _ in 0..(3usize.pow(9)) {
                binary = core::hint::black_box(binary + 1);
            }
        });

        println!("0b{binary:b}")
    }
}
