//! Algorithms and definitions from (Frieder & Luk, 1975)

use crate::tern_next::Ternary;

pub use self::a1 as mutually_exclusive;
pub use self::a2 as minor_pair;

pub use self::b1 as neg;
pub use self::b2 as partial_add;
pub use self::b3 as add;

/// a and b are mutually exclusive iff a & b == 0
#[expect(unused)]
struct Def1;

/// Algorithm a1 in (Frieder & Luk, 1975)
/// Takes in pair (a, b), and returns pair (c, d), where (c, d) are mutually exclusive.
/// [`Def1`]
#[inline]
pub const fn a1((a, b): (u32, u32)) -> (u32, u32) {
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
pub const fn a2((a, b): (u32, u32)) -> (u32, u32) {
    let c = (b ^ a) | a;
    let d = (b ^ a) ^ c;
    (c, d)
}

/// Algorithm b1 in (Frieder & Luk, 1975)
/// Takes in BT number A, and returns BT number -A
#[inline]
pub const fn b1<const SIZE: usize>(a: Ternary<SIZE>) -> Ternary<SIZE>
where
    [(); SIZE + (usize::MAX - 32)]:,
{
    Ternary {
        pos: a.neg,
        neg: a.pos,
    }
}

/// Algorithm b2 in (Frieder & Luk, 1975)
/// Partial BT addition
/// Given a = [a1, a2], b = [b1, 0]
/// And b1 is minor to a1
#[inline]
pub const fn b2<const S1: usize, const S2: usize>(a: Ternary<S1>, b: Ternary<S2>) -> Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    let Ternary { pos: a1, neg: a2 } = a;
    let Ternary { pos: b1, neg: _ } = b;
    let x1 = a1 + b1;
    let x2 = a1 & !x1;
    let s1 = x1 & !b1;
    let s2 = x2 | a2;
    let (s1, s2) = mutually_exclusive((s1, s2));
    let st = Ternary { pos: s1, neg: s2 };
    st
}

/// Algorithm b3 in (Frieder & Luk, 1975)
/// Full BT addition
///
/// The consts here are to make sure the larger ternary size gets returned.
#[inline]
pub const fn b3<const S1: usize, const S2: usize>(a: Ternary<S1>, b: Ternary<S2>) -> Ternary<S1>
where
    [(); S1 + (usize::MAX - 32)]:,
    [(); S2 + (usize::MAX - 32)]:,
    [(); S1 - S2]:,
{
    let Ternary { pos: a1, neg: a2 } = a;
    let Ternary { pos: b1, neg: b2 } = b;
    let (a2, b1) = mutually_exclusive((a2, b1));
    let (a1, b1) = minor_pair((a1, b1));
    let ct = partial_add(
        Ternary::<S1> { pos: a1, neg: a2 },
        Ternary::<S2> { pos: b1, neg: b2 },
    );
    let Ternary { pos: c1, neg: c2 } = ct;
    let (c1, b2) = mutually_exclusive((c1, b2));
    let (c2, b2) = minor_pair((c2, b2));
    let brackets = partial_add(
        Ternary::<S1> { pos: c2, neg: c1 },
        Ternary::<S2> { pos: b2, neg: 0 },
    );
    let st = neg(brackets);
    st
}
