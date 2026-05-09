use std::cmp::Ordering;

use crate::{instr::{
    Word, dec::{Imm12, Imm18, JTC_1701, Reg, decode}, enc::encode
}, machine::{err::ExceptionType, memory::Memory, registers::Registers}};

pub mod dispatch;
pub mod memory;
pub mod registers;
pub mod err;

#[derive(Default)]
pub struct Machine {
    regs: Registers,
    memory: Memory,
}

impl Machine {
    // Reg Type: rd, rs1, rs2
    // rd = rs1 OP rs2
    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] + self.regs[rs2];
        pc + Word::THREE
    }
    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] - self.regs[rs2];
        pc + Word::THREE
    }
    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] & self.regs[rs2];
        pc + Word::THREE
    }
    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] | self.regs[rs2];
        pc + Word::THREE
    }
    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] ^ self.regs[rs2];
        pc + Word::THREE
    }
    fn shr(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Result<Word, ExceptionType> {
        let imm: isize = self.regs[rs2].into();
        if imm > 27 {
            Err(ExceptionType::ShiftOverflow)
        } else {
            self.regs[rd] = self.regs[rs1] >> imm;
            Ok(pc + Word::THREE)
        }
    }
    fn shl(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Result<Word, ExceptionType> {
        let imm: isize = self.regs[rs2].into();
        if imm > 27 {
            Err(ExceptionType::ShiftOverflow)
        } else {
            self.regs[rd] = self.regs[rs1] << imm;
            Ok(pc + Word::THREE)
        }
    }
    fn cmp(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Word {
        self.regs[rd] = match self.regs[rs1].cmp(&self.regs[rs2]) {
            Ordering::Less    => -Word::ONE,
            Ordering::Equal   => Word::ZERO,
            Ordering::Greater => Word::ONE,
        };
        pc + Word::THREE
    }
    fn mul(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] * self.regs[rs2];
        pc + Word::THREE
    }
    fn div(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Result<Word, ExceptionType> {
        let denominator = self.regs[rs2];
        if denominator == Word::ZERO {
            Err(ExceptionType::DivByZero)
        } else {
            self.regs[rd] = self.regs[rs1] / denominator;
            Ok(pc + Word::THREE)
        }
    }
    fn rem(&mut self, rd: Reg, rs1: Reg, rs2: Reg, pc: Word) -> Result<Word, ExceptionType> {
        let denominator = self.regs[rs2];
        if denominator == Word::ZERO {
            Err(ExceptionType::DivByZero)
        } else {
            self.regs[rd] = self.regs[rs1] % denominator;
            Ok(pc + Word::THREE)
        }
    }
    // Imm Type: rd, rs1, imm12
    // rd = rs1 OP imm12
    fn addi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] + imm12.extend();
        pc + Word::THREE
    }
    fn subi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] - imm12.extend();
        pc + Word::THREE
    }
    fn andi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] & imm12.extend();
        pc + Word::THREE
    }
    fn ori(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] | imm12.extend();
        pc + Word::THREE
    }
    fn xori(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] ^ imm12.extend();
        pc + Word::THREE
    }
    fn shri(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        let imm: isize = imm12.into();
        if imm > 27 {
            Err(ExceptionType::ShiftOverflow)
        } else {
            self.regs[rd] = self.regs[rs1] >> imm;
            Ok(pc + Word::THREE)
        }
    }
    fn shli(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        let imm: isize = imm12.into();
        if imm > 27 {
            Err(ExceptionType::ShiftOverflow)
        } else {
            self.regs[rd] = self.regs[rs1] << imm;
            Ok(pc + Word::THREE)
        }
    }
    fn cmpi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = match self.regs[rs1].cmp(&imm12.extend()) {
            Ordering::Less    => -Word::ONE,
            Ordering::Equal   =>  Word::ZERO,
            Ordering::Greater =>  Word::ONE,
        };
        pc + Word::THREE
    }
    fn muli(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = self.regs[rs1] * imm12.extend();
        pc + Word::THREE
    }
    fn divi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        let denominator = imm12.extend();
        if denominator == Word::ZERO {
            Err(ExceptionType::DivByZero)
        } else {
            self.regs[rd] = self.regs[rs1] / denominator;
            Ok(pc + Word::THREE)
        }
    }
    fn remi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        let denominator = imm12.extend();
        if denominator == Word::ZERO {
            Err(ExceptionType::DivByZero)
        } else {
            self.regs[rd] = self.regs[rs1] % denominator;
            Ok(pc + Word::THREE)
        }
    }
    // Branch Type: rs1, rs2, imm12
    fn beq (&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Word {
        if self.regs[rs1] == self.regs[rs2] {
            pc + imm12.extend()
        } else {
            pc + Word::THREE
        }
    }
    fn blt (&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Word {
        if self.regs[rs1] < self.regs[rs2] {
            pc + imm12.extend()
        } else {
            pc + Word::THREE
        }
    }
    fn bgt (&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Word {
        if self.regs[rs1] > self.regs[rs2] {
            pc + imm12.extend()
        } else {
            pc + Word::THREE
        }
    }
    fn bne (&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Word {
        if self.regs[rs1] != self.regs[rs2] {
            pc + imm12.extend()
        } else {
            pc + Word::THREE
        }
    }
    fn bleq(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Word {
        if self.regs[rs1] <= self.regs[rs2] {
            pc + imm12.extend()
        } else {
            pc + Word::THREE
        }
    }
    fn bgeq(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Word {
        if self.regs[rs1] >= self.regs[rs2] {
            pc + imm12.extend()
        } else {
            pc + Word::THREE
        }
    }
    // Upper: rd, imm18
    fn lui(&mut self, rd: Reg, imm18: Imm18, pc: Word) -> Word {
        self.regs[rd] = imm18.extend() << 9;
        pc + Word::THREE
    }
    fn auipc(&mut self, rd: Reg, imm18: Imm18, pc: Word) -> Word {
        self.regs[rd] = pc + (imm18.extend() << 9);
        pc + Word::THREE
    }
    // Loads: rd, rs1, imm12
    // rd = M[rs1 + imm]
    fn lt(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        self.memory.read_tryte(self.regs[rs1] + imm12.extend()).map(|val| {
            self.regs[rd] = val.extend();
            pc + Word::THREE
        }).ok_or(ExceptionType::PageFault)
    }
    fn lw(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        self.memory.read_word(self.regs[rs1] + imm12.extend()).map(|val| {
            self.regs[rd] = val;
            pc + Word::THREE
        }).ok_or(ExceptionType::PageFault)
    }
    // Stores: rs1, rs2, imm12
    fn st(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        self.memory
            .write_tryte(self.regs[rs1] + imm12.slice(0), self.regs[rs2].slice(0))
            .map(|_| pc + Word::THREE)
            .ok_or(ExceptionType::PageFault)
    }
    fn sw(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Result<Word, ExceptionType> {
        self.memory
            .write_word(self.regs[rs1] + imm12.slice(0), self.regs[rs2])
            .map(|_| pc + Word::THREE)
            .ok_or(ExceptionType::PageFault)
    }
    // Jumps: rd, rs1, imm12
    // rd = PC + 3; pc += (rs1 + imm18)
    fn jal(&mut self, rd: Reg, imm18: Imm18, pc: Word) -> Word {
        self.regs[rd] = pc + Word::THREE;
        pc + imm18.extend()
    }
    fn jalr(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word {
        self.regs[rd] = pc + Word::THREE;
        pc + self.regs[rs1] + imm12.extend()
    }
    // System (U)
    fn ecall(&mut self, rd: Reg, imm18: Imm18, pc: Word) -> Word { return pc; todo!() }
    fn ebreak(&mut self, rd: Reg, imm18: Imm18, pc: Word) -> Word { todo!() }
    fn sret(&mut self, rd: Reg, imm18: Imm18, pc: Word) -> Word { todo!() }
    fn wfi(&mut self, rd: Reg, imm18: Imm18, pc: Word) -> Word { todo!() }
    // System (S)
    fn csrw(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12, pc: Word) -> Word { todo!() }
    // System (I)
    fn csrr(&mut self, rd: Reg, rs1: Reg, imm12: Imm12, pc: Word) -> Word { todo!() }

    //== HELPER FUNCTIONS ==//
    fn next(&self, pc: Word) -> Result<JTC_1701, ExceptionType> {
        self.memory.read_word(pc).ok_or(ExceptionType::PageFault).and_then(|val| decode(val))
    }
}
