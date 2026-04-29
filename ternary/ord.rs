use core::cmp::Ordering;

use crate::Ternary;

impl<const SIZE: usize> PartialOrd for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }
}
