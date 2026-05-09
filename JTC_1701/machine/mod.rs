use crate::{instr::{
    Word, dec::{Imm12, Imm18, Reg}
}, machine::{memory::Memory, registers::Registers}};

pub mod dispatch;
pub mod memory;
pub mod registers;

pub struct Machine {
    regs: Registers,
    memory: Memory,
}

impl Machine {
    // Reg Type: rd, rs1, rs2
    fn add(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    fn sub(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    fn and(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    fn or(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    fn xor(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    fn shr(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    fn shl(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    fn cmp(&mut self, rd: Reg, rs1: Reg, rs2: Reg) {}
    // Imm Type: rd, rs1, imm12
    fn addi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn subi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn andi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn ori(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn xori(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn shri(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn shli(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn cmpi(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    // Branch Type: rs1, rs2, imm12
    fn beq(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    fn blt(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    fn bgt(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    fn bne(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    fn bleq(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    fn bgeq(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    // Upper: rd, imm18
    fn lui(&mut self, rd: Reg, imm18: Imm18) {}
    fn auipc(&mut self, rd: Reg, imm18: Imm18) {}
    // Loads: rd, rs1, imm12
    fn lt(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn lw(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    // Stores: rs1, rs2, imm12
    fn st(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    fn sw(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    // Jumps: rd, rs1, imm12
    fn jal(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    fn jalr(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
    // System (U)
    fn ecall(&mut self, rd: Reg, imm18: Imm18) {}
    fn ebreak(&mut self, rd: Reg, imm18: Imm18) {}
    fn sret(&mut self, rd: Reg, imm18: Imm18) {}
    fn wfi(&mut self, rd: Reg, imm18: Imm18) {}
    // System (S)
    fn csrw(&mut self, rs1: Reg, rs2: Reg, imm12: Imm12) {}
    // System (I)
    fn csrr(&mut self, rd: Reg, rs1: Reg, imm12: Imm12) {}
}
