use core::ops::Neg;

pub mod and;
pub mod or;
pub mod xor;

use crate::Ternary;
use crate::helper::neg;

const impl<const SIZE: usize> Neg for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        neg(self)
    }
}
