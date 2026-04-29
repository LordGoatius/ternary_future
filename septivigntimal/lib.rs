#![expect(incomplete_features)]
#![feature(generic_const_exprs)]
#![cfg_attr(not(test), no_std)]

use balanced_ternary::Ternary;

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
