use super::*;

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
pub unsafe trait Encoding: Sized {
    fn get_op(self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(OP_IND))
    }
    
    fn get_rd(self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(RD_IND))
    }
    
    fn get_imm3(self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(IMM3_IND))
    }
    
    fn get_fn2(self) -> Option<Ternary<2>> {
        Some(self.get_val().slice(FN2_IND))
    }
    
    fn get_rs1(self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(RS1_IND))
    }
    
    fn get_imm18(self) -> Option<Ternary<18>> {
        Some(self.get_val().slice(IMM18_IND))
    }
    
    fn get_val3(self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(VAL3_IND))
    }
    
    fn get_rs2(self) -> Option<Ternary<3>> {
        Some(self.get_val().slice(RS2_IND))
    }
    
    fn get_imm12(self) -> Option<Ternary<12>> {
        Some(self.get_val().slice(IMM12_IND))
    }
    
    fn get_func4(self) -> Option<Ternary<4>> {
        Some(self.get_val().slice(FUNC4_IND))
    }
    
    fn get_imm9(self) -> Option<Ternary<9>> {
        Some(self.get_val().slice(IMM9_IND))
    }
    
    fn get_func5(self) -> Option<Ternary<5>> {
        Some(self.get_val().slice(FUNC5_IND))
    }
    

    #[inline]
    fn get_val(self) -> Word {
        unsafe {
            (&raw const self).cast::<Word>().read()
        }
    }
}

// TODO: Macro. Can't figure it out rn.
unsafe impl Encoding for RInstr {
    fn get_imm3(self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm9(self) -> Option<Ternary<9>> {
        None
    }

    fn get_imm12(self) -> Option<Ternary<12>> {
        None
    }

    fn get_imm18(self) -> Option<Ternary<18>> {
        None
    }
}

unsafe impl Encoding for SInstr {
    fn get_rd(self) -> Option<Ternary<3>> {
        None
    }

    fn get_func4(self) -> Option<Ternary<4>> {
        None
    }

    fn get_func5(self) -> Option<Ternary<5>> {
        None
    }

    fn get_imm12(self) -> Option<Ternary<12>> {
        None
    }

    fn get_imm18(self) -> Option<Ternary<18>> {
        None
    }
}

unsafe impl Encoding for IInstr {
    fn get_func4(self) -> Option<Ternary<4>> {
        None
    }

    fn get_func5(self) -> Option<Ternary<5>> {
        None
    }

    fn get_rs2(self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm3(self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm9(self) -> Option<Ternary<9>> {
        None
    }

    fn get_imm18(self) -> Option<Ternary<18>> {
        None
    }
}

unsafe impl Encoding for UInstr {
    fn get_imm3(self) -> Option<Ternary<3>> {
        None
    }

    fn get_imm9(self) -> Option<Ternary<9>> {
        None
    }

    fn get_imm12(self) -> Option<Ternary<12>> {
        None
    }

    fn get_func4(self) -> Option<Ternary<4>> {
        None
    }

    fn get_func5(self) -> Option<Ternary<5>> {
        None
    }

    fn get_rs2(self) -> Option<Ternary<3>> {
        None
    }

    fn get_val3(self) -> Option<Ternary<3>> {
        None
    }

    fn get_rs1(self) -> Option<Ternary<3>> {
        None
    }
}
