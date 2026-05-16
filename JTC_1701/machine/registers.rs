use crate::instr::{Word, dec::Reg};

#[derive(Default)]
pub struct Registers {
    regs: [Word; 27],
}

impl Registers {
    pub fn get(&self, index: Reg) -> Word {
        let ind: isize = index.into();
        if ind == 0 { return Word::ZERO }
        let ind = ind + 13;
        // type `Reg` always a value between -13..=13, so +13 means between
        // 0..=27. Indexing can never panic.
        unsafe {
            *self.regs.get_unchecked(ind as usize)
        }
    }

    pub fn set(&mut self, index: Reg, val: Word) {
        let ind: isize = index.into();
        if ind == 0 { return }
        let ind = ind + 13;
        // type `Reg` always a value between -13..=13, so +13 means between
        // 0..=27. Indexing can never panic.
        unsafe {
            *self.regs.get_unchecked_mut(ind as usize) = val;
        }
    }
}
