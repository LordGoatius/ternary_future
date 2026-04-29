use core::cmp::Ordering;

use crate::Ternary;

// TODO: Bitwise operations that can do better than this? Subtraction isn't ideal.
pub fn cmp<const SIZE: usize>(val1: Ternary<SIZE>, val2: Ternary<SIZE>) -> Ordering
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    (val1 - val2).sign_innner()
}

impl<const SIZE: usize> PartialOrd for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(cmp(*self, *other))
    }
}

impl<const SIZE: usize> Ord for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

#[cfg(test)]
pub mod tests {
    use crate::Ternary;

    #[test]
    fn test_cmp() {
        let mut val = Ternary::<9>::ONE;
        let max = Ternary::<9>::MAX;
        for _ in 0..(3usize.pow(8) / 2) {
            assert!(val < max);
            val += Ternary::ONE;
        }
    }
}
