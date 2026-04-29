use core::ops::Neg;

pub mod and;
pub mod or;
pub mod xor;

use crate::helper::neg;
use crate::Ternary;

impl<const SIZE: usize> Neg for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        neg(self)
    }
}
