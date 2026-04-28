use std::fmt::{Binary, Debug, Display};

use super::Ternary;

impl<const SIZE: usize> Display for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let value: isize = self.into();
        write!(f, "{value}")
    }
}

impl<const SIZE: usize> Debug for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct(&format!("Ternary<{SIZE}>"))
            .field_with("pos", |f| write!(f, "{:b}", &self.pos))
            .field_with("neg", |f| write!(f, "{:b}", &self.neg))
            .finish()
    }
}

impl<const SIZE: usize> Binary for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // TODO: Buffer prints all right now. Change to take in formatting options?
        // How handle that?
        let mut buf: [u8; SIZE] = [b'0'; SIZE];
        let Ternary { pos, neg } = self;
        for i in (0..SIZE).rev() {
            match ((pos >> i) & 1, (neg >> i) & 1) {
                (0, 1) => buf[SIZE - i - 1] = b'T',
                (1, 0) => buf[SIZE - i - 1] = b'1',
                _ => (),
            }
        }
        write!(f, "{}", unsafe { str::from_utf8_unchecked(&buf) })
    }
}

