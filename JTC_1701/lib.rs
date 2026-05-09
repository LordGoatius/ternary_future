#![expect(incomplete_features)]
#![feature(
    generic_const_exprs,
    const_trait_impl,
    const_ops,
    explicit_tail_calls,
)]
#![allow(non_snake_case)]

extern crate balanced_ternary as ternary;

mod instr;
mod machine;

#[cfg(test)]
pub mod tests {
    use septivigntimal::*;

    use crate::{instr::{dec::{Imm12, JTC_1701}, enc::encode}, machine::Machine};

    #[test]
    #[cfg(feature = "ext_mul")]
    fn test_basic_machine() {
        use crate::instr::dec::Imm18;

        let mut machine = Machine::default();
        // 0 add r1, r1, 1
        // 1 add r2, r2, n
        // 2 add r3, r3, 1
        // 3 add r4, r4, 0
        //   loop:
        // 4   bleq r2, r1, hlt
        // 5   mul r3, r2, 0
        // 6   sub r2, r1, 0
        // 7   bleq r4, r4, loop
        //   loop_15:
        // 8   add r5, r5, 15
        //   cont:
        // 9   bleq r5, r4, hlt
        // 10  sub r5, r4, 1
        // 11  bleq r4, r4, cont
        //   hlt:
        // 12   hlt
        // TODO: Change to Offsets
        const HLT: Imm12 = Imm12::from_str("110");
        const LOOP: Imm18 = Imm18::from_str("11");
        let n: Imm12 = 6.try_into().unwrap();
        let instrs = [
            JTC_1701::ADDI(A, A, Imm12::ONE),
            JTC_1701::ADDI(B, B, n),
            JTC_1701::ADDI(C, C, Imm12::ONE),
            JTC_1701::ADDI(D, D, Imm12::ZERO),
            // loop
            JTC_1701::BLEQ(B, A, HLT),
            JTC_1701::MUL(C, C, B),
            JTC_1701::SUB(B, B, A),
            JTC_1701::JAL(Z, LOOP),
            // HLT
            JTC_1701::ECALL(A, Imm18::ZERO)
        ];
    }
}
