use std::ops::{Add, AddAssign};

use crate::tern_next::Ternary;
use crate::helper::add;

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
