#![expect(incomplete_features)]
#![feature(generic_const_exprs, const_trait_impl, explicit_tail_calls, type_info)]
#![cfg_attr(test, feature(test))]
#![allow(non_snake_case)]
#![cfg(test)]
extern crate test;

extern crate balanced_ternary as ternary;

mod instr;
pub mod machine;

#[cfg(test)]
pub mod tests {
    use crate::{
        instr::{Word, dec::{Imm12, Imm18, JTC_1701}, enc::encode},
        machine::{Machine, memory::Memory},
    };

    use memdevice::devices::rom::Rom;
    use memdevice::{MemEntry, MemoryDevice, devices::ram::Ram};
    use septivigntimal::*;

    #[test]
    #[cfg(feature = "ext_mul")]
    fn test_basic_machine() {
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
        const HALT: Imm12 = Imm12::from_str("110"); // +12
        const LOOP: Imm18 = Imm18::from_str("T00"); // -9
        let n: Imm12 = 6.try_into().unwrap();
        let instrs = [
            /* 0*/ encode(JTC_1701::ADDI(A, A, Imm12::ONE)),
            /* 3*/ encode(JTC_1701::ADDI(B, B, n)),
            /* 6*/ encode(JTC_1701::ADDI(C, C, Imm12::ONE)),
            /* 9*/ encode(JTC_1701::ADDI(D, D, Imm12::ZERO)),
            // loop
            /* 12 */ encode(JTC_1701::BLEQ(B, A, HALT)),
            /* 15 */ encode(JTC_1701::MUL(C, C, B)),
            /* 18 */ encode(JTC_1701::SUB(B, B, A)),
            /* 21 */ encode(JTC_1701::JAL(Z, LOOP)),
            // HLT
            /* 24 */ encode(JTC_1701::ECALL(A, Imm18::ZERO)),
        ];
        let rom = Rom::new(instrs);
        let memdevice = Box::leak(Box::new(rom)).to_mementry(Word::ZERO);

        let mem = Memory::from_slice(vec![memdevice].leak());

        let mut machine = Machine::from_memory(mem);
        machine.dispatch(Word::ZERO);
        assert_eq!(machine.regs.get(C), Word::from_str("1000T00"));
    }

    #[test]
    #[cfg(feature = "ext_mul")]
    fn test_factorial_ram_machine() {
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
        const HALT: Imm12 = Imm12::from_str("110"); // +12
        const LOOP: Imm18 = Imm18::from_str("T00"); // -9
        let n: Imm12 = 6.try_into().unwrap();
        let instrs: [Word; 9] = [
            /* 0*/ encode(JTC_1701::ADDI(A, A, Imm12::ONE)),
            /* 3*/ encode(JTC_1701::ADDI(B, B, n)),
            /* 6*/ encode(JTC_1701::ADDI(C, C, Imm12::ONE)),
            /* 9*/ encode(JTC_1701::ADDI(D, D, Imm12::ZERO)),
            // loop
            /* 12 */ encode(JTC_1701::BLEQ(B, A, HALT)),
            /* 15 */ encode(JTC_1701::MUL(C, C, B)),
            /* 18 */ encode(JTC_1701::SUB(B, B, A)),
            /* 21 */ encode(JTC_1701::JAL(Z, LOOP)),
            // HLT
            /* 24 */ encode(JTC_1701::ECALL(A, Imm18::ZERO)),
        ];

        let ram: &'static mut Ram =
            Box::leak(Box::new(Ram::new(Word::ZERO, Word::MAX / Word::TWO)));
        for (i, instr) in instrs.into_iter().enumerate() {
            ram.write_word(dbg!(i as isize * 3).try_into().unwrap(), instr);
        }
        let memslice: &'static mut [MemEntry] = vec![ram.to_mementry()].leak();

        let mem = Memory::from_slice(memslice);
        let mut machine = Machine::from_memory(mem);
        machine.dispatch(Word::ZERO);
        assert_eq!(machine.regs.get(C), Word::from_str("1000T00"));
    }

    #[test]
    #[cfg(feature = "ext_mul")]
    fn test_factorial_ram_recursive_machine() {
        // NOTE: "pseudocode"
        // fn fact(n: i27) -> i27 {
        //     if n <= 2 {
        //         return n
        //     } else {
        //         return n * fact(n - 1)
        //     }
        // }
        // 
        // 0 add r1, 2
        // 1 add r2, n
        // 2 add r10, 729 / 2 ; top word in our current 
        // 3 call FACT (jal r4, 6)
        // 4 j HALT
        // FACT: ; n passed in r2, call address in r4
        // 5 sw r4, r10
        // 6 add r10, -1
        // 7 bgt r2, r1, CALC ; if n <= 2
        // 8 add r3, r2 ; store n in r3
        // 9 j RET
        // CALC: ; n is in r2
        // 10 call FACT ; fact(n-1) will be in r3
        // 11 mul r3, r2
        // RET:
        // 12 add r10, 1
        // 13 lw r4, r10
        // 14 ret (jal r4)
        // HALT:
        // 15 ecall
        const TOP: Imm12 = Imm12::from_str("111110");
        let n: Imm12 = 6.try_into().unwrap();
        let instrs: [Word; 16] = {
            use JTC_1701::*;
            [
                /* 00 */ encode(ADDI(A, A, Imm12::TWO)),
                /* 03 */ encode(ADDI(B, B, n)),
                /* 06 */ encode(ADDI(J, J, TOP)),
                /* 09 */ encode(JAL(D, Imm18::from_str("1T0"))),
                /* 12 */ encode(JAL(ZERO, Imm18::from_str("1010"))),
                // FACT:
                /* 15 */ encode(SW(J, D, Imm12::ZERO)),
                /* 18 */ encode(ADDI(J, J, -Imm12::ONE)),
                /* 21 */ encode(BGT(B, A, Imm12::from_str("100"))),
                /* 24 */ encode(ADD(C, B, ZERO)),
                /* 27 */ encode(JAL(ZERO, Imm18::from_str("100"))),
                // CALC:
                /* 30 */ encode(JAL(D, Imm18::from_str("1TT0"))),
                /* 33 */ encode(MUL(C, C, B)),
                // RET:
                /* 36 */ encode(ADDI(J, J, Imm12::ONE)),
                /* 39 */ encode(LW(D, J, Imm12::ZERO)),
                /* 42 */ encode(JALR(ZERO, D, Imm12::ZERO)),
                encode(ECALL(A, Imm18::ZERO)),
            ]
        };

        let ram: &'static mut Ram =
            Box::leak(Box::new(Ram::new(Word::ZERO, Word::MAX / Word::TWO)));
        for (i, instr) in instrs.into_iter().enumerate() {
            ram.write_word((i as isize * 3).try_into().unwrap(), instr);
        }
        let memslice: &'static mut [MemEntry] = vec![ram.to_mementry()].leak();

        let mem = Memory::from_slice(memslice);
        let mut machine = Machine::from_memory(mem);
        machine.dispatch(Word::ZERO);
        assert_eq!(machine.regs.get(C), Word::from_str("1000T00"));
    }
}

#[cfg(test)]
pub mod benches {
    use memdevice::MemEntry;
    use septivigntimal::*;
    use test::Bencher;

    use crate::{
        instr::{
            dec::{Imm12, JTC_1701},
            enc::encode,
        },
        machine::Machine,
    };

    #[bench]
    fn bench_factorial(b: &mut Bencher) {
        use memdevice::devices::rom::Rom;

        use crate::{
            instr::{dec::Imm18, Word},
            machine::memory::Memory,
        };
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
        const HALT: Imm12 = Imm12::from_str("110"); // +12
        const LOOP: Imm18 = Imm18::from_str("T00"); // -9
        const N: Imm12 = Imm12::from_str("1T0"); // +6
        static INSTRS: [Word; 9] = [
            /* 0*/ encode(JTC_1701::ADDI(A, A, Imm12::ONE)),
            /* 3*/ encode(JTC_1701::ADDI(B, B, N)),
            /* 6*/ encode(JTC_1701::ADDI(C, C, Imm12::ONE)),
            /* 9*/ encode(JTC_1701::ADDI(D, D, Imm12::ZERO)),
            // loop
            /* 12 */ encode(JTC_1701::BLEQ(B, A, HALT)),
            /* 15 */ encode(JTC_1701::MUL(C, C, B)),
            /* 18 */ encode(JTC_1701::SUB(B, B, A)),
            /* 21 */ encode(JTC_1701::JAL(Z, LOOP)),
            // HLT
            /* 24 */ encode(JTC_1701::ECALL(A, Imm18::ZERO)),
        ];
        static mut ROM: Rom<9> = Rom::new(INSTRS);
        #[allow(static_mut_refs)]
        // SAFETY: Unsafe in a test is fine, it's just to ensure it all works.
        static mut MEMSLICE: &mut [MemEntry] = &mut [unsafe { ROM.to_mementry(Word::ZERO) }];

        b.iter(move || {
            let mem = Memory::from_slice(unsafe { MEMSLICE });
            let mut machine = Machine::from_memory(mem);
            machine.dispatch(Word::ZERO);
        });
    }
}
