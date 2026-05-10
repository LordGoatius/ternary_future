use core::fmt::{Binary, Debug, Display};

use super::Ternary;

/// Evil hack to allow printing generic const values with debug impl
const fn to_str<const SIZE: usize>() -> &'static str {
    match SIZE {
        0  => "0",
        1  => "1",
        2  => "2",
        3  => "3",
        4  => "4",
        5  => "5",
        6  => "6",
        7  => "7",
        8  => "8",
        9  => "9",
        10 => "10",
        11 => "11",
        12 => "12",
        13 => "13",
        14 => "14",
        15 => "15",
        16 => "16",
        17 => "17",
        18 => "18",
        19 => "19",
        20 => "20",
        21 => "21",
        22 => "22",
        23 => "23",
        24 => "24",
        25 => "25",
        26 => "26",
        27 => "27",
        28 => "28",
        29 => "29",
        30 => "30",
        31 => "31",
        32 => "32",
        _ => panic!()
    }
}

impl<const SIZE: usize> Display for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let value: isize = self.into();
        write!(f, "{value}")
    }
}

impl<const SIZE: usize> Debug for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        // Yes I know it's stupid but I want my debug types to say
        // "Ternary<SIZE>" not just "Ternary" and I am willing to do
        // unspeakable things to achieve that.
        let name = ["Ternary", "<", to_str::<SIZE>(), ">"];
        f.write_str(name[0])?;
        f.write_str(name[1])?;
        f.write_str(name[2])?;
        f.write_str(name[3])?;
        f.debug_struct("")
            .field_with("pos", |f| write!(f, "0b{:032b}", &self.pos))
            .field_with("neg", |f| write!(f, "0b{:032b}", &self.neg))
            .finish()
    }
}

impl<const SIZE: usize> Binary for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
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
