use std::hint::unreachable_unchecked;

use ternary::concat;

use crate::instr::enc::Encoding;

#[allow(non_camel_case_types)]
use super::*;

pub type Reg = Ternary<3>;
pub type Op = Ternary<4>;
pub type Fn2 = Ternary<2>;
pub type Imm18 = Ternary<18>;
pub type Imm12 = Ternary<12>;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JTC_1701 {
    // Reg Type: rd, rs1, rs2
    ADD   (Reg, Reg, Reg),
    SUB   (Reg, Reg, Reg),
    AND   (Reg, Reg, Reg),
    OR    (Reg, Reg, Reg),
    XOR   (Reg, Reg, Reg),
    SHR   (Reg, Reg, Reg),
    SHL   (Reg, Reg, Reg),
    CMP   (Reg, Reg, Reg),
    // Imm Type: rd, rs1, imm12
    ADDI  (Reg, Reg, Imm12),
    SUBI  (Reg, Reg, Imm12),
    ANDI  (Reg, Reg, Imm12),
    ORI   (Reg, Reg, Imm12),
    XORI  (Reg, Reg, Imm12),
    SHRI  (Reg, Reg, Imm12),
    SHLI  (Reg, Reg, Imm12),
    CMPI  (Reg, Reg, Imm12),
    // Branch Type: rs1, rs2, imm12
    BEQ   (Reg, Reg, Imm12),
    BLT   (Reg, Reg, Imm12),
    BGT   (Reg, Reg, Imm12),
    BNE   (Reg, Reg, Imm12),
    BLEQ  (Reg, Reg, Imm12),
    BGEQ  (Reg, Reg, Imm12),
    // Upper: rd, imm18
    LUI   (Reg, Imm18),
    AUIPC (Reg, Imm18),
    // Loads: rd, rs1, imm12
    LT    (Reg, Reg, Imm12),
    LW    (Reg, Reg, Imm12),
    // Stores: rs1, rs2, imm12
    ST    (Reg, Reg, Imm12),
    SW    (Reg, Reg, Imm12),
    // Jumps: rd, rs1, imm12
    JAL   (Reg, Reg, Imm12),
    JALR  (Reg, Reg, Imm12),
    // System (U)
    ECALL (Reg, Imm18),
    EBREAK(Reg, Imm18),
    SRET  (Reg, Imm18),
    WFI   (Reg, Imm18),
    // System (S)
    CSRW  (Reg, Reg, Imm12),
    // System (I)
    CSRR  (Reg, Reg, Imm12),
}

pub const ARITH_OP:  Op = Op::from_str("0T01");
pub const LOAD_OP:   Op = Op::from_str("0T00");
pub const BRANCH_OP: Op = Op::from_str("0T11");
pub const IMM_OP:    Op = Op::from_str("0011");
pub const STORE_OP:  Op = Op::from_str("0T10");
pub const JUMP_OP:   Op = Op::from_str("0110");
pub const UPPER_OP:  Op = Op::from_str("0111");
pub const SYSTEM_OP: Op = Op::from_str("0001");

pub mod funct_consts {
    use septivigntimal::*;

    use crate::instr::dec::Fn2;

    pub const FN2_ADD: Fn2 = A.slice(0);
    pub const FN2_SUB: Fn2 = N.slice(0);
    pub const FN2_AND: Fn2 = B.slice(0);
    pub const FN2_OR:  Fn2 = C.slice(0);
    pub const FN2_XOR: Fn2 = D.slice(0);
    pub const FN2_SHR: Fn2 = O.slice(0);
    pub const FN2_SHL: Fn2 = P.slice(0);
    pub const FN2_CMP: Fn2 = Q.slice(0);

    pub const FN2_BEQ:  Fn2 = Fn2::from_str("00");
    pub const FN2_BLT:  Fn2 = Fn2::from_str("TT");
    pub const FN2_BGT:  Fn2 = Fn2::from_str("11");
    pub const FN2_BNE:  Fn2 = Fn2::from_str("1T");
    pub const FN2_BLEQ: Fn2 = Fn2::from_str("T0");
    pub const FN2_BGEQ: Fn2 = Fn2::from_str("10");

    pub const FN2_ST: Fn2 = Fn2::from_str("00");
    pub const FN2_SW: Fn2 = Fn2::from_str("01");

    pub const FN2_LT: Fn2 = Fn2::from_str("00");
    pub const FN2_LW: Fn2 = Fn2::from_str("01");

    pub const FN2_JAL:  Fn2 =  Fn2::from_str("00");
    pub const FN2_JALR: Fn2 =  Fn2::from_str("01");

    pub const FN2_LUI:   Fn2 =  Fn2::from_str("00");
    pub const FN2_AUIPC: Fn2 =  Fn2::from_str("01");

    pub const FN2_ECALL:  Fn2 = Fn2::from_str("00");
    pub const FN2_EBREAK: Fn2 = Fn2::from_str("01");
    pub const FN2_SRET:   Fn2 = Fn2::from_str("0T");
    pub const FN2_CSRW:   Fn2 = Fn2::from_str("1T");
    pub const FN2_CSRR:   Fn2 = Fn2::from_str("T1");
    pub const FN2_WFI:    Fn2 = Fn2::from_str("T0");
}

// TODO: Check reserved/zero portions (func4/5) and return res/ill instr trap
//
// TODO(2): This is the only non-const function of the whole system right now.
// We need to be able to call enum constructors which are coerced into function
// pointers in a const context.
#[rustfmt::skip]
pub(crate) fn decode(instr: Word) -> JTC_1701 {
    use funct_consts::*;
    match instr.slice::<4>(0) {
        ARITH_OP => {
            let op = RInstr(instr);
            let rd = op.get_rd().unwrap();
            let rs1 = op.get_rs1().unwrap();
            let rs2 = op.get_rs2().unwrap();
            let fn2 = op.get_fn2().unwrap();
            // NOTE: funcs will be used later for extensions such as mul and div
            let _func4 = op.get_func4().unwrap();
            let _func5 = op.get_func5().unwrap();

            (match fn2 {
                FN2_ADD => JTC_1701::ADD,
                FN2_SUB => JTC_1701::SUB,
                FN2_AND => JTC_1701::AND,
                FN2_OR  => JTC_1701::OR,
                FN2_XOR => JTC_1701::XOR,
                FN2_SHR => JTC_1701::SHR,
                FN2_SHL => JTC_1701::SHL,
                FN2_CMP => JTC_1701::CMP,
                _ => panic!("Illegal Instr (TODO)")
            })(
                rd,
                rs1,
                rs2
            )
        },
        IMM_OP => {
            let op = IInstr(instr);
            let rd = op.get_rd().unwrap();
            let rs1 = op.get_rs1().unwrap();
            let imm12 = op.get_imm12().unwrap();
            let fn2 = op.get_fn2().unwrap();

            (match fn2 {
                FN2_ADD => JTC_1701::ADDI,
                FN2_SUB => JTC_1701::SUBI,
                FN2_AND => JTC_1701::ANDI,
                FN2_OR  => JTC_1701::ORI,
                FN2_XOR => JTC_1701::XORI,
                FN2_SHR => JTC_1701::SHRI,
                FN2_SHL => JTC_1701::SHLI,
                FN2_CMP => JTC_1701::CMPI,
                _ => panic!("Illegal Instr (TODO)")
            })(
                rd,
                rs1,
                imm12,
            )
        },
        LOAD_OP => {
            let op = IInstr(instr);
            let rd = op.get_rd().unwrap();
            let rs1 = op.get_rs1().unwrap();
            let imm12 = op.get_imm12().unwrap();
            let fn2 = op.get_fn2().unwrap();

            (match fn2 {
                FN2_LT => JTC_1701::LT,
                FN2_LW => JTC_1701::LW,
                _ => panic!("Illegal Instr (TODO)")
            })(
                rd,
                rs1,
                imm12
            )
        },
        BRANCH_OP => {
            let op = SInstr(instr);
            let rs1 = op.get_rs1().unwrap();
            let rs2 = op.get_rs2().unwrap();
            let imm9 = op.get_imm9().unwrap();
            let imm3 = op.get_imm3().unwrap();
            let fn2 = op.get_fn2().unwrap();
            let imm12 = concat::<9, 3, 12>(imm9, imm3);

            (match fn2 {
                FN2_BEQ  => JTC_1701::BEQ,
                FN2_BLT  => JTC_1701::BLT,
                FN2_BGT  => JTC_1701::BGT,
                FN2_BNE  => JTC_1701::BNE,
                FN2_BLEQ => JTC_1701::BLEQ,
                FN2_BGEQ => JTC_1701::BGEQ,
                _ => panic!("Illegal Instr (TODO)")
            })(
                rs1,
                rs2,
                imm12
            )
        },
        STORE_OP => {
            let op = SInstr(instr);
            let rs1 = op.get_rs1().unwrap();
            let rs2 = op.get_rs2().unwrap();
            let imm9 = op.get_imm9().unwrap();
            let imm3 = op.get_imm3().unwrap();
            let fn2 = op.get_fn2().unwrap();
            let imm12 = concat::<9, 3, 12>(imm9, imm3);

            (match fn2 {
                FN2_ST => JTC_1701::ST,
                FN2_SW => JTC_1701::SW,
                _ => panic!("Illegal Instr (TODO)")
            })(
                rs1,
                rs2,
                imm12,
            )
        },
        JUMP_OP => {
            let op = IInstr(instr);
            let rd = op.get_rd().unwrap();
            let rs1 = op.get_rs1().unwrap();
            let fn2 = op.get_fn2().unwrap();
            let imm12 = op.get_imm12().unwrap();

            (match fn2 {
                FN2_JAL  => JTC_1701::JAL,
                FN2_JALR => JTC_1701::JALR,
                _ => panic!("Illegal Instr (TODO)")
            })(
                rd,
                rs1,
                imm12,
            )
        },
        UPPER_OP => {
            let op = UInstr(instr);
            let rd = op.get_rd().unwrap();
            let fn2 = op.get_fn2().unwrap();
            let imm18 = op.get_imm18().unwrap();

            (match fn2 {
                FN2_LUI   => JTC_1701::LUI,
                FN2_AUIPC => JTC_1701::AUIPC,
                _ => panic!("Illegal Instr (TODO)")
            })(
                rd,
                imm18,
            )
        },
        SYSTEM_OP => {
            let uop = UInstr(instr);
            let fn2 = uop.get_fn2().unwrap();
            match fn2 {
                val @ (FN2_ECALL |
                FN2_EBREAK |
                FN2_SRET |
                FN2_WFI) => {
                    let rd = uop.get_rd().unwrap();
                    let imm = uop.get_imm18().unwrap();
                    (match val {
                        FN2_ECALL  => JTC_1701::ECALL,
                        FN2_EBREAK => JTC_1701::EBREAK,
                        FN2_SRET   => JTC_1701::SRET,
                        FN2_WFI    => JTC_1701::WFI,
                        _          => unsafe { unreachable_unchecked() },
                    })(
                        rd,
                        imm
                    )
                },
                FN2_CSRR => {
                    let op = IInstr(instr);
                    let rd = op.get_rd().unwrap();
                    let rs1 = op.get_rs1().unwrap();
                    let imm12 = op.get_imm12().unwrap();
                    JTC_1701::CSRR(rd, rs1, imm12)
                },
                FN2_CSRW => {
                    let op = SInstr(instr);
                    let rs1 = op.get_rs1().unwrap();
                    let rs2 = op.get_rs2().unwrap();
                    let imm9 = op.get_imm9().unwrap();
                    let imm3 = op.get_imm3().unwrap();
                    let imm12 = concat::<9, 3, 12>(imm9, imm3);
                    JTC_1701::CSRW(rs1, rs2, imm12)
                },
                _ => panic!("Illegal Instruction")
            }
        },
        _ => panic!("Illegal Instr"),
    }
}

#[cfg(test)]
pub mod tests {
    use crate::instr::dec::funct_consts::*;
    use itertools::Itertools;

    #[test]
    fn fn_asserts() {
        let consts = [
            FN2_ADD,
            FN2_SUB,
            FN2_AND,
            FN2_OR,
            FN2_XOR,
            FN2_SHR,
            FN2_SHL,
            FN2_CMP
        ];

        for combos in consts.iter().combinations(2) {
            println!("{:?}, {:?}", combos[0], combos[1]);
            assert_ne!(combos[0], combos[1]);
        }

        let consts = [
            FN2_BEQ,
            FN2_BLT,
            FN2_BGT,
            FN2_BNE,
            FN2_BLEQ,
            FN2_BGEQ];

        for combos in consts.iter().combinations(2) {
            assert_ne!(combos[0], combos[1]);
        }

        assert_ne!(FN2_ST, FN2_SW);

        assert_ne!(FN2_LT, FN2_LW);

        assert_ne!(FN2_JAL, FN2_JALR);

        assert_ne!(FN2_LUI, FN2_AUIPC);
    }
}
