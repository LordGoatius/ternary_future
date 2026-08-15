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
    pub fn read(&self, csr: CSR) -> Word {
        match csr {
            CSR::SSTATUS => todo!(),
            CSR::SIE => todo!(),
            CSR::STVEC => todo!(),
            CSR::SCOUNTEREN => todo!(),
            CSR::SSCRATCH => todo!(),
            CSR::SEPC => todo!(),
            CSR::STCAUSE => todo!(),
            CSR::STVAL => todo!(),
            CSR::SIP => todo!(),
            CSR::SATP => todo!(),
        }
        self.values[csr as usize]
    }
    pub fn write(&mut self, csr: CSR, value: Word) {
        match csr {
            CSR::SSTATUS => todo!(),
            CSR::SIE => todo!(),
            CSR::STVEC => todo!(),
            CSR::SCOUNTEREN => todo!(),
            CSR::SSCRATCH => todo!(),
            CSR::SEPC => todo!(),
            CSR::STCAUSE => todo!(),
            CSR::STVAL => todo!(),
            CSR::SIP => todo!(),
            CSR::SATP => todo!(),
        }
        self.values[csr as usize] = value
    }
}
