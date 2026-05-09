use crate::instr::dec::{ARITH_OP, BRANCH_OP, Fn2, IMM_OP, Imm12, Imm18, JTC_1701, JUMP_OP, LOAD_OP, Op, Reg, STORE_OP, SYSTEM_OP, UPPER_OP};

use super::*;

const fn create_rinstr(rd: Reg, rs1: Reg, rs2: Reg, fn2: Fn2, op: Op) -> Word {
    let word = Word::ZERO;
    let word = word.set(op, OP_IND);
    let word = word.set(rd, RD_IND);
    let word = word.set(rs1, RS1_IND);
    let word = word.set(rs2, RS2_IND);
    word.set(fn2, FN2_IND)
}

const fn create_sinstr(rs1: Reg, rs2: Reg, imm12: Imm12, fn2: Fn2, op: Op) -> Word {
    let word = Word::ZERO;
    let word = word.set(op, OP_IND);
    let word = word.set(rs1, RS1_IND);
    let word = word.set(rs2, RS2_IND);
    let word = word.set(imm12.slice::<3>(0), IMM3_IND);
    let word = word.set(imm12.slice::<9>(3), IMM9_IND);
    word.set(fn2, FN2_IND)
}

const fn create_iinstr(rd: Reg, rs1: Reg, imm12: Imm12, fn2: Fn2, op: Op) -> Word {
    let word = Word::ZERO;
    let word = word.set(op, OP_IND);
    let word = word.set(rd, RD_IND);
    let word = word.set(rs1, RS1_IND);
    let word = word.set(imm12, IMM12_IND);
    word.set(fn2, FN2_IND)
}

const fn create_uinstr(rd: Reg, imm18: Imm18, fn2: Fn2, op: Op) -> Word {
    let word = Word::ZERO;
    let word = word.set(op, OP_IND);
    let word = word.set(rd, RD_IND);
    let word = word.set(imm18, IMM18_IND);
    word.set(fn2, FN2_IND)
}

pub(crate) const fn encode(instr: JTC_1701) -> Word {
    use super::dec::funct_consts::*;
    use JTC_1701::*;
    // This should be able to be optimized well.
    // If not, I have some ideas.
    match instr {
        // Arith
        ADD(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_ADD, ARITH_OP),
        SUB(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_SUB, ARITH_OP),
        AND(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_AND, ARITH_OP),
        OR (rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_OR , ARITH_OP),
        XOR(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_XOR, ARITH_OP),
        SHR(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_SHR, ARITH_OP),
        SHL(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_SHL, ARITH_OP),
        CMP(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_CMP, ARITH_OP),
        // Arith Imm
        ADDI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_ADD, IMM_OP),
        SUBI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_SUB, IMM_OP),
        ANDI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_AND, IMM_OP),
        ORI (rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_OR , IMM_OP),
        XORI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_XOR, IMM_OP),
        SHRI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_SHR, IMM_OP),
        SHLI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_SHL, IMM_OP),
        CMPI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_CMP, IMM_OP),
        // Loads
        LT(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_LT, LOAD_OP),
        LW(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_LW, LOAD_OP),
        // Stores
        ST(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_ST, STORE_OP),
        SW(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_SW, STORE_OP),
        // Jumps
        JAL (rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_JAL , JUMP_OP),
        JALR(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_JALR, JUMP_OP),
        // Branches
        BEQ (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BEQ , BRANCH_OP),
        BLT (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BLT , BRANCH_OP),
        BGT (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BGT , BRANCH_OP),
        BNE (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BNE , BRANCH_OP),
        BLEQ(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BLEQ, BRANCH_OP),
        BGEQ(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BGEQ, BRANCH_OP),
        // Loading Uppers
        LUI   (rd, imm18) => create_uinstr(rd, imm18, FN2_LUI  , UPPER_OP),
        AUIPC (rd, imm18) => create_uinstr(rd, imm18, FN2_AUIPC, UPPER_OP),
        // System
        ECALL (rd, imm18) => create_uinstr(rd, imm18, FN2_ECALL , SYSTEM_OP),
        EBREAK(rd, imm18) => create_uinstr(rd, imm18, FN2_EBREAK, SYSTEM_OP),
        SRET  (rd, imm18) => create_uinstr(rd, imm18, FN2_SRET  , SYSTEM_OP),
        WFI   (rd, imm18) => create_uinstr(rd, imm18, FN2_WFI   , SYSTEM_OP),
        // CSRR
        CSRW  (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_CSRW, SYSTEM_OP),
        // CSRW
        CSRR  (rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_CSRR, SYSTEM_OP),
    }
}

const OP_IND:    u32 = 0;
const RD_IND:    u32 = 4;
const IMM3_IND:  u32 = 4;
const FN2_IND:   u32 = 7;
const RS1_IND:   u32 = 9;
const IMM18_IND: u32 = 9;
const VAL3_IND:  u32 = 12;
const RS2_IND:   u32 = 15;
const IMM12_IND: u32 = 15;
const FUNC4_IND: u32 = 18;
const IMM9_IND:  u32 = 18;
const FUNC5_IND: u32 = 22;

/// Can't believe I'm doing this again.
/// # Safety
/// Don't implement on anything that isn't `#[repr(transparent)]`
/// [`Ternary<27>`]
pub(crate) const unsafe trait Encoding: Sized {
    fn get_op(&self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(OP_IND))
    }
    
    fn get_rd(&self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(RD_IND))
    }
    
    fn get_imm3(&self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(IMM3_IND))
    }
    
    fn get_fn2(&self) -> Option<Ternary<2>> {
        Some(self.get_val().slice(FN2_IND))
    }
    
    fn get_rs1(&self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(RS1_IND))
    }
    
    fn get_imm18(&self) -> Option<Ternary<18>> {
        Some(self.get_val().slice(IMM18_IND))
    }
    
    fn get_val3(&self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(VAL3_IND))
    }
    
    fn get_rs2(&self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(RS2_IND))
    }
    
    fn get_imm12(&self) -> Option<Ternary<12>> {
        Some(self.get_val().slice(IMM12_IND))
    }
    
    fn get_func4(&self) -> Option<Ternary<4>> {
        Some(self.get_val().slice(FUNC4_IND))
    }
    
    fn get_imm9(&self) -> Option<Ternary<9>> {
        Some(self.get_val().slice(IMM9_IND))
    }
    
    fn get_func5(&self) -> Option<Ternary<5>> {
        Some(self.get_val().slice(FUNC5_IND))
    }
    

    #[inline]
    fn get_val(&self) -> Word {
        unsafe {
            (&raw const *self).cast::<Word>().read()
        }
    }
}

// TODO: Macro. Can't figure it out rn.
unsafe impl const Encoding for RInstr {
    fn get_imm3(&self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm9(&self) -> Option<Ternary<9>> {
        None
    }

    fn get_imm12(&self) -> Option<Ternary<12>> {
        None
    }

    fn get_imm18(&self) -> Option<Ternary<18>> {
        None
    }
}

unsafe impl const Encoding for SInstr {
    fn get_rd(&self) -> Option<Ternary<3>> {
        None
    }

    fn get_func4(&self) -> Option<Ternary<4>> {
        None
    }

    fn get_func5(&self) -> Option<Ternary<5>> {
        None
    }

    fn get_imm12(&self) -> Option<Ternary<12>> {
        None
    }

    fn get_imm18(&self) -> Option<Ternary<18>> {
        None
    }
}

unsafe impl const Encoding for IInstr {
    fn get_func4(&self) -> Option<Ternary<4>> {
        None
    }

    fn get_func5(&self) -> Option<Ternary<5>> {
        None
    }

    fn get_rs2(&self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm3(&self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm9(&self) -> Option<Ternary<9>> {
        None
    }

    fn get_imm18(&self) -> Option<Ternary<18>> {
        None
    }
}

unsafe impl const Encoding for UInstr {
    fn get_imm3(&self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm9(&self) -> Option<Ternary<9>> {
        None
    }

    fn get_imm12(&self) -> Option<Ternary<12>> {
        None
    }

    fn get_func4(&self) -> Option<Ternary<4>> {
        None
    }

    fn get_func5(&self) -> Option<Ternary<5>> {
        None
    }

    fn get_rs2(&self) -> Option<Ternary<3>> {
        None
    }

    fn get_val3(&self) -> Option<Ternary<3>> {
        None
    }

    fn get_rs1(&self) -> Option<Ternary<3>> {
        None
    }
}
