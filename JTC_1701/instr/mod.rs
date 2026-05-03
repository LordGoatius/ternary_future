pub mod enc;
pub mod dec;

use ternary::Ternary;

pub type Word = Ternary<27>;
pub type Tryte = Ternary<6>;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct RInstr(Word);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct IInstr(Word);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct SInstr(Word);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UInstr(Word);

#[cfg(test)]
pub mod tests {
    use septivigntimal::*;

    use crate::instr::{dec::{Imm12, Imm18, decode}, enc::encode};

    #[test]
    fn decenc() {
        use super::dec::JTC_1701::*;
        let imm12 = Imm12::MAX;
        let imm18 = Imm18::MIN;
        let instrs = [
            // Reg Type: rd, rs1, rs2
            ADD   (A, B, C),
            SUB   (A, B, C),
            AND   (A, B, C),
            OR    (A, B, C),
            XOR   (A, B, C),
            SHR   (A, B, C),
            SHL   (A, B, C),
            CMP   (A, B, C),
            // Imm Type: rd, rs1, imm12
            ADDI  (A, B, imm12),
            SUBI  (A, B, imm12),
            ANDI  (A, B, imm12),
            ORI   (A, B, imm12),
            XORI  (A, B, imm12),
            SHRI  (A, B, imm12),
            SHLI  (A, B, imm12),
            CMPI  (A, B, imm12),
            // Branch Type: rs1, rs2, imm12
            BEQ   (A, B, imm12),
            BLT   (A, B, imm12),
            BGT   (A, B, imm12),
            BNE   (A, B, imm12),
            BLEQ  (A, B, imm12),
            BGEQ  (A, B, imm12),
            // Upper: rd, imm18
            LUI   (A, imm18),
            AUIPC (A, imm18),
            // Loads: rd, rs1, imm12
            LT    (A, B, imm12),
            LW    (A, B, imm12),
            // Stores: rs1, rs2, imm12
            ST    (A, B, imm12),
            SW    (A, B, imm12),
            // Jumps: rd, rs1, imm12
            JAL   (A, B, imm12),
            JALR  (A, B, imm12),
        ];

        for instr in instrs {
            println!("{instr:?}");
            let enc = encode(instr);
            println!("{enc:?}");
            let dec = decode(enc);
            println!("{dec:?}");
            assert_eq!(instr, dec);
        }
    }
}
