use super::Ternary;

fn into_isize<const SIZE: usize>(val: Ternary<SIZE>) -> isize
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let mut sum = 0;
    for i in 0..SIZE {
        sum += ((val.pos >> i) & 1) as isize * 3isize.pow(i as u32);
        sum -= ((val.neg >> i) & 1) as isize * 3isize.pow(i as u32);
    }
    sum
}

// n: bits in `val`
// b: original radix (2)
// a_k: kth bit
// $\sum_{k=0}^{n} a_k b^k
fn into_ternary<const SIZE: usize>(val: isize) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let sign = val.signum();
    if sign == 0 { return Ternary::ZERO; }
    let val = val.abs();
    let mut sum = Ternary::ZERO;
    const LOG_2_3POW32: usize = 51;
    // 51 bits is all the information required to represent a 32 trit number
    for k in 0..LOG_2_3POW32 {
        let bk = Ternary::TWO.pow(k as u32);
        let pos = (val >> k) as u32 & 1;
        sum += Ternary { pos, neg: 0 } * bk;
    }
    if sign == 1 {
        sum
    } else {
        -sum
    }
}

impl<const SIZE: usize> Into<isize> for Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn into(self) -> isize {
        into_isize(self)
    }
}

impl<const SIZE: usize> Into<isize> for &Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn into(self) -> isize {
        into_isize(*self)
    }
}

impl<const SIZE: usize> Into<isize> for &mut Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn into(self) -> isize {
        into_isize(*self)
    }
}

impl<const SIZE: usize> Into<Ternary<SIZE>> for isize
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn into(self) -> Ternary<SIZE> {
        into_ternary(self)
    }
}

impl<const SIZE: usize> Into<Ternary<SIZE>> for &isize
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn into(self) -> Ternary<SIZE> {
        into_ternary(*self)
    }
}

impl<const SIZE: usize> Into<Ternary<SIZE>> for &mut isize
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    fn into(self) -> Ternary<SIZE> {
        into_ternary(*self)
    }
}

#[cfg(test)]
pub mod tests {
    use crate::tern_next::{Ternary, convert::{into_isize, into_ternary}};

    #[test]
    fn tern_conversion() {
        let mut ternary: Ternary<9> = Ternary::MIN;

        let tone: Ternary<9> = Ternary::ONE;

        for _ in 0..(3usize.pow(9)) {
            println!("0b{:b}, 0t{:b}", into_isize(ternary), ternary);
            assert_eq!(ternary, into_ternary(into_isize(ternary)));
            ternary = ternary + tone;
        }
    
    }
}
