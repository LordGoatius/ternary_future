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
    if sign == 0 {
        return Ternary::ZERO;
    }
    let val = val.abs();
    let mut sum = Ternary::ZERO;
    // `ceil(log_2(3^SIZE))` bits is all the information required to
    // represent a SIZE trit number.
    // In the exposed conversion impl, we will check this.
    for k in 0..const { 3usize.pow(SIZE as u32).ilog2() + 1 } {
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

// NOTE: An `isize` is always capable of holding a larger number
// than a 32 bit ternary number

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

//== TryInto ==//

#[derive(Debug)]
pub struct TernaryConversionError;

impl<const SIZE: usize> TryInto<Ternary<SIZE>> for isize
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Error = TernaryConversionError;
    fn try_into(self) -> Result<Ternary<SIZE>, TernaryConversionError> {
        if self.abs().leading_zeros()
            > const { (size_of::<isize>() * 8) as u32 - (3usize.pow(SIZE as u32).ilog2() + 1) }
        {
            Ok(into_ternary(self))
        } else {
            Err(TernaryConversionError)
        }
    }
}

impl<const SIZE: usize> TryInto<Ternary<SIZE>> for &isize
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Error = TernaryConversionError;
    fn try_into(self) -> Result<Ternary<SIZE>, TernaryConversionError> {
        if self.abs().leading_zeros()
            > const { (size_of::<isize>() * 8) as u32 - (3usize.pow(SIZE as u32).ilog2() + 1) }
        {
            Ok(into_ternary(*self))
        } else {
            Err(TernaryConversionError)
        }
    }
}

// TODO: Fix the conversion errors, because it will succeed for some values it should fail at.

impl<const SIZE: usize> TryInto<Ternary<SIZE>> for &mut isize
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    type Error = TernaryConversionError;
    fn try_into(self) -> Result<Ternary<SIZE>, TernaryConversionError> {
        if self.abs().leading_zeros()
            > const { (size_of::<isize>() * 8) as u32 - (3usize.pow(SIZE as u32).ilog2() + 1) }
        {
            Ok(into_ternary(*self))
        } else {
            Err(TernaryConversionError)
        }
    }
}

#[cfg(test)]
pub mod tests {
    use crate::{
        convert::{into_isize, into_ternary},
        Ternary,
    };

    #[test]
    fn tern_conversion() {
        let mut ternary: Ternary<9> = Ternary::MIN;

        let tone: Ternary<9> = Ternary::ONE;

        for _ in 0..(3usize.pow(9)) {
            println!("0b{:b}, 0t{:b}", into_isize(ternary), ternary);
            assert_eq!(ternary, into_ternary(into_isize(ternary)));
            ternary = ternary + tone;
        }

        for binary in (-3isize.pow(9) / 2)..(3isize.pow(9 / 2)) {
            println!("0b{:b}, 0t{:b}", binary, into_ternary::<9>(binary));
            assert_eq!(binary, into_isize(into_ternary::<9>(binary)));
        }
    }

    #[test]
    fn tern_conversion_traits() {
        let mut ternary: Ternary<9> = Ternary::MIN;

        let tone: Ternary<9> = Ternary::ONE;

        for _ in 0..(3usize.pow(9)) {
            println!("0b{:b}, 0t{:b}", into_isize(ternary), ternary);
            let i: isize = ternary.into();
            let t: Ternary<9> = i.try_into().unwrap();
            assert_eq!(ternary, t);
            ternary = ternary + tone;
        }

        for binary in (-3isize.pow(9) / 2)..(3isize.pow(9 / 2)) {
            println!("0b{:b}, 0t{:b}", binary, into_ternary::<9>(binary));
            let t: Ternary<9> = binary.try_into().unwrap();
            let b: isize = t.into();
            assert_eq!(binary, b);
        }
    }

    #[test]
    #[should_panic]
    fn tern_conversion_fail() {
        let num: isize = (1 << 16) - 1;
        let t: Ternary<9> = num.try_into().unwrap();
    }
}
