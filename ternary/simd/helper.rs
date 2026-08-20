use core::simd::Simd;
use super::TernarySimd;

pub use self::a1_simd as mutually_exclusive_simd;
pub use self::a2_simd as minor_pair_simd;

pub use self::b1_simd as neg_simd;
pub use self::b2_simd as partial_add_simd;
pub use self::b3_simd as add_simd;

/// a and b are mutually exclusive iff a & b == 0
#[expect(unused)]
struct Def1;

/// Algorithm a1 in (Frieder & Luk, 1975)
/// Takes in pair (a, b), and returns pair (c, d), where (c, d) are mutually exclusive.
/// [`Def1`]
#[inline]
pub fn a1_simd<const N: usize>((a, b): (Simd<u32, N>, Simd<u32, N>)) -> (Simd<u32, N>, Simd<u32, N>) {
    let (c, d) = (a & !b, b & !a);
    (c, d)
}

/// a is minor to b iff a & b == b
#[expect(unused)]
struct Def2;

/// Algorithm a2 in (Frieder & Luk, 1975)
/// Takes in pair (a, b), and returns pair (c, d), where d is minor to c.
/// [`Def2`]
#[inline]
pub fn a2_simd<const N: usize>((a, b): (Simd<u32, N>, Simd<u32, N>)) -> (Simd<u32, N>, Simd<u32, N>) {
    let c = (b ^ a) | a;
    let d = (b ^ a) ^ c;
    (c, d)
}

/// Algorithm b1 in (Frieder & Luk, 1975)
/// Takes in BT number A, and returns BT number -A
#[inline]
pub fn b1_simd<const N: usize, const SIZE: usize>(a: TernarySimd<SIZE, N>) -> TernarySimd<SIZE, N>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    TernarySimd {
        pos: a.neg,
        neg: a.pos,
    }
}

/// Algorithm b2 in (Frieder & Luk, 1975)
/// Partial BT addition
/// Given a = [a1, a2], b = [b1, 0]
/// And b1 is minor to a1
#[inline]
pub fn b2_simd<const N: usize, const SIZE: usize>(a: TernarySimd<SIZE, N>, b: TernarySimd<SIZE, N>) -> TernarySimd<SIZE, N>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let TernarySimd { pos: a1, neg: a2 } = a;
    let TernarySimd { pos: b1, neg: _ } = b;
    let x1 = a1 + b1;
    let x2 = a1 & !x1;
    let s1 = x1 & !b1;
    let s2 = x2 | a2;
    let (s1, s2) = mutually_exclusive_simd((s1, s2));
    TernarySimd { pos: s1, neg: s2 }
}

/// Algorithm b3 in (Frieder & Luk, 1975)
/// Full BT addition
///
/// The consts here are to make sure the larger ternary size gets returned.
#[inline]
pub fn b3_simd<const N: usize, const SIZE: usize>(a: TernarySimd<SIZE, N>, b: TernarySimd<SIZE, N>) -> TernarySimd<SIZE, N>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    let TernarySimd { pos: a1, neg: a2 } = a;
    let TernarySimd { pos: b1, neg: b2 } = b;
    let (a2, b1) = mutually_exclusive_simd((a2, b1));
    let (a1, b1) = minor_pair_simd((a1, b1));
    let ct = partial_add_simd(
        TernarySimd::<SIZE, N> { pos: a1, neg: a2 },
        TernarySimd::<SIZE, N> { pos: b1, neg: b2 },
    );
    let TernarySimd { pos: c1, neg: c2 } = ct;
    let (c1, b2) = mutually_exclusive_simd((c1, b2));
    let (c2, b2) = minor_pair_simd((c2, b2));
    let brackets = partial_add_simd(
        TernarySimd::<SIZE, N> { pos: c2, neg: c1 },
        TernarySimd::<SIZE, N> { pos: b2, neg: Simd::from_array([0; N]) },
    );
    neg_simd(brackets)
}
