use crate::instr::dec::{Fn2, Imm12, Imm18, JTC_1701, Reg};

use super::*;

fn create_rinstr(rd: Reg, rs1: Reg, rs2: Reg, fn2: Fn2) -> Word {
    let word = Word::ZERO;
    let word = word.set(rd, RD_IND);
    let word = word.set(rs1, RS1_IND);
    let word = word.set(rs2, RS2_IND);
    word.set(fn2, FN2_IND)
}

fn create_sinstr(rs1: Reg, rs2: Reg, imm12: Imm12, fn2: Fn2) -> Word {
    let word = Word::ZERO;
    let word = word.set(rs1, RS1_IND);
    let word = word.set(rs2, RS2_IND);
    let word = word.set(imm12.slice::<3>(0), IMM3_IND);
    let word = word.set(imm12.slice::<9>(3), IMM9_IND);
    word.set(fn2, FN2_IND)
}

fn create_iinstr(rd: Reg, rs1: Reg, imm12: Imm12, fn2: Fn2) -> Word {
    let word = Word::ZERO;
    let word = word.set(rd, RD_IND);
    let word = word.set(rs1, RS1_IND);
    let word = word.set(imm12, IMM12_IND);
    word.set(fn2, FN2_IND)
}

fn create_uinstr(rd: Reg, imm18: Imm18, fn2: Fn2) -> Word {
    let word = Word::ZERO;
    let word = word.set(rd, RD_IND);
    let word = word.set(imm18, IMM18_IND);
    word.set(fn2, FN2_IND)
}

pub(crate) fn encode(instr: JTC_1701) -> Word {
    use super::dec::funct_consts::*;
    use JTC_1701::*;
    // This should be able to be optimized well.
    // If not, I have some ideas.
    match instr {
        // Arith
        ADD(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_ADD),
        SUB(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_SUB),
        AND(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_AND),
        OR (rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_OR ),
        XOR(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_XOR),
        SHR(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_SHR),
        SHL(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_SHL),
        CMP(rd, rs1, rs2) => create_rinstr(rd, rs1, rs2, FN2_CMP),
        // Arith Imm
        ADDI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_ADD),
        SUBI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_SUB),
        ANDI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_AND),
        ORI (rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_OR ),
        XORI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_XOR),
        SHRI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_SHR),
        SHLI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_SHL),
        CMPI(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_CMP),
        // Loads
        LT(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_LT),
        LW(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_LW),
        // Stores
        ST(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_ST),
        SW(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_SW),
        // Jumps
        JAL (rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_JAL),
        JALR(rd, rs1, imm12) => create_iinstr(rd, rs1, imm12, FN2_JALR),
        // Branches
        BEQ (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BEQ),
        BLT (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BLT),
        BGT (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BGT),
        BNE (rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BNE),
        BLEQ(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BLEQ),
        BGEQ(rs1, rs2, imm12) => create_sinstr(rs1, rs2, imm12, FN2_BGEQ),
        // Loading Uppers
        LUI  (rd, imm18) => create_uinstr(rd, imm18, FN2_LUI),
        AUIPC(rd, imm18) => create_uinstr(rd, imm18, FN2_AUIPC),
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
