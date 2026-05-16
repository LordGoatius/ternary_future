use std::mem::type_info::{Type, TypeKind};

use crate::instr::Word;

// TODO: Map out valid ranges and whatever but for now that doesn't matter.
#[repr(usize)]
pub enum CSR {
    // Supervisor CSR
    SSTATUS,
    SIE,
    STVEC,
    SCOUNTEREN,
    SSCRATCH,
    SEPC,
    STCAUSE,
    STVAL,
    SIP,
    SATP,
}

const CSR_LEN: usize = {
    // Liking this compie-time reflection
    let info = Type::of::<CSR>();
    if let TypeKind::Enum(val) = info.kind {
        val.variants.len()
    } else { panic!() }
};

// This is insufficient.
// CSR are not like this.
#[derive(Default)]
pub(crate) struct Csr {
    values: [Word; CSR_LEN]
}

impl Csr {
    pub fn read(&self, index: CSR) -> Word {
        self.values[index as usize]
    }
    pub fn write(&mut self, index: CSR, value: Word) {
        self.values[index as usize] = value
    }
}
