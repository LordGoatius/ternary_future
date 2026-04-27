use std::ops::Neg;

pub mod and;
pub mod or;
pub mod xor;

use crate::tern_next::Ternary;
use crate::helper::neg;

impl<const SIZE: usize> Neg for Ternary<SIZE> 
    where [(); SIZE + (usize::MAX - 32)]:
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        neg(self)
    }
}

