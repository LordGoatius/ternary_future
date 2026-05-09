use memdevice::MemoryDevice;

use crate::instr::Word;

pub struct Machine {
    regs: [Word; 27],
    memspace: [&'static dyn MemoryDevice],
}

impl Machine {
    
}
