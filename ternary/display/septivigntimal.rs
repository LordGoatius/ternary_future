#![expect(unused)]
use crate::Ternary;

pub type Tribble = Ternary<3>;

pub const Z:    Tribble = Tribble::from_str("TTT");
pub const Y:    Tribble = Tribble::from_str("TT0");
pub const X:    Tribble = Tribble::from_str("TT1");
pub const W:    Tribble = Tribble::from_str("T0T");
pub const V:    Tribble = Tribble::from_str("T00");
pub const U:    Tribble = Tribble::from_str("T01");
pub const T:    Tribble = Tribble::from_str("T1T");
pub const S:    Tribble = Tribble::from_str("T10");
pub const R:    Tribble = Tribble::from_str("T11");
pub const Q:    Tribble = Tribble::from_str("0TT");
pub const P:    Tribble = Tribble::from_str("0T0");
pub const O:    Tribble = Tribble::from_str("0T1");
pub const N:    Tribble = Tribble::from_str("00T");
pub const ZERO: Tribble = Tribble::from_str("000");
pub const A:    Tribble = Tribble::from_str("001");
pub const B:    Tribble = Tribble::from_str("01T");
pub const C:    Tribble = Tribble::from_str("010");
pub const D:    Tribble = Tribble::from_str("011");
pub const E:    Tribble = Tribble::from_str("1TT");
pub const F:    Tribble = Tribble::from_str("1T0");
pub const G:    Tribble = Tribble::from_str("1T1");
pub const H:    Tribble = Tribble::from_str("10T");
pub const I:    Tribble = Tribble::from_str("100");
pub const J:    Tribble = Tribble::from_str("101");
pub const K:    Tribble = Tribble::from_str("11T");
pub const L:    Tribble = Tribble::from_str("110");
pub const M:    Tribble = Tribble::from_str("111");

pub const fn to_ascii_upper(val: Tribble) -> u8 {
    match val {
        Z => b'Z',
        Y => b'Y',
        X => b'X',
        W => b'W',
        V => b'V',
        U => b'U',
        T => b'T',
        S => b'S',
        R => b'R',
        Q => b'Q',
        P => b'P',
        O => b'O',
        N => b'N',
        ZERO => b'0',
        A => b'A',
        B => b'B',
        C => b'C',
        D => b'D',
        E => b'E',
        F => b'F',
        G => b'G',
        H => b'H',
        I => b'I',
        J => b'J',
        K => b'K',
        L => b'L',
        M => b'M',
        _ => panic!("Unreachable")
    }
    
}

#[cfg(test)]
pub mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn uniq() {
         let vals = [
             Z,
             Y,
             X,
             W,
             V,
             U,
             T,
             S,
             R,
             Q,
             P,
             O,
             N,
             ZERO,
             A,
             B,
             C,
             D,
             E,
             F,
             G,
             H,
             I,
             J,
             K,
             L,
             M,
         ];

         vals.iter()
             .combinations(2)
             .for_each(|vec| assert_ne!(vec[0], vec[1]));
    }
}
