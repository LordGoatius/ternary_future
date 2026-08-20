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

struct STVEC(Word);
struct SCOUNTEREN(Word);
struct SSCRATCH(Word);
struct SEPC(Word);
struct STCAUSE(Word);
struct STVAL(Word);
struct SIP(Word);
struct SATP(Word);

/// SSTATUS:
///     SIE: Interrupt enable
///     SPIE: The SPIE bit indicates whether supervisor
///           interrupts were enabled prior to trapping into supervisor mode.
///           When a trap is taken into supervisor mode, SPIE is set to SIE,
///           and SIE is set to 0. When an SRET instruction is executed, SIE
///           is set to SPIE, then SPIE is set to 1.
///     SPP: 0 if trap comes from userspace
struct SSTATUS(Word);
impl SSTATUS {
    fn write(&mut self) {}
}

struct SIE(Word);
