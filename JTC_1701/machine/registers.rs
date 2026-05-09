use std::ops::{Index, IndexMut};

use crate::instr::{Word, dec::Reg};

pub struct Registers {
    regs: [Word; 27],
}

impl Index<Reg> for Registers {
    type Output = Word;

    fn index(&self, index: Reg) -> &Self::Output {
        let ind: isize = index.into();
        let ind = ind + 13;
        // type `Reg` always a value between -13..=13, so +13 means between
        // 0..=27. Indexing can never panic.
        unsafe {
            self.regs.get_unchecked(ind as usize)
        }
    }
}

impl IndexMut<Reg> for Registers {
    fn index_mut(&mut self, index: Reg) -> &mut Self::Output {
        let ind: isize = index.into();
        let ind = ind + 13;
        // type `Reg` always a value between -13..=13, so +13 means between
        // 0..=27. Indexing can never panic.
        unsafe {
            self.regs.get_unchecked_mut(ind as usize)
        }
    }
}
