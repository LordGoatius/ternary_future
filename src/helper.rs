/// This function takes a [`Ternary<SIZE>`] and normalizes the internal representation
/// through turning each number into the mutually exclusive version of itself
/// Algorithm a1 in (Frieder & Luk, 1975)
#[inline]
fn mutually_exclude(data: (u32, u32)) -> (u32, u32) {
    let (a, b) = data;
    // This should theoretically allow a very smart compiler to
    // optimize on the fact that these are in fact mutually exlusive
    assert_eq!(a & b, 0);
    (a & !b, b & !a)
}

