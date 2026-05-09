use crate::{instr::{Word, dec::JTC_1701}, machine::Machine};

type DispatchFn = fn(&mut Machine, instr: JTC_1701, pc: Word) -> Word;

static TABLE: [DispatchFn; 36] = [
    add,
    sub,
    and,
    or,
    xor,
    shr,
    shl,
    cmp,
    addi,
    subi,
    andi,
    ori,
    xori,
    shri,
    shli,
    cmpi,
    beq,
    blt,
    bgt,
    bne,
    bleq,
    bgeq,
    lui,
    auipc,
    lt,
    lw,
    st,
    sw,
    jal,
    jalr,
    ecall,
    ebreak,
    sret,
    wfi,
    csrw,
    csrr,
];

fn add(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ADD(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}
fn sub(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SUB(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}
fn and(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::AND(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}
fn or(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::OR(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}
fn xor(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::XOR(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}
fn shr(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHR(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}
fn shl(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHL(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}
fn cmp(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CMP(rd, rs1, rs2) = instr else { panic!() };
    todo!()
}

fn addi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ADDI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn subi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SUBI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn andi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ANDI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn ori(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ORI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn xori(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::XORI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn shri(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHRI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn shli(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHLI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn cmpi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CMPI(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn beq(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BEQ(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn blt(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BLT(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn bgt(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BGT(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn bne(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BNE(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn bleq(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BLEQ(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn bgeq(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BGEQ(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn lui(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::LUI(rd, imm18) = instr else { panic!() };
    todo!()
}
fn auipc(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::AUIPC(rd, imm18) = instr else { panic!() };
    todo!()
}
fn lt(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::LT(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn lw(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::LW(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
fn st(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ST(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn sw(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SW(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn jal(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::JAL(rd, imm18) = instr else { panic!() };
    todo!()
}
fn jalr(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::JALR(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}

fn ecall(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ECALL(rd, imm18) = instr else { panic!() };
    todo!()
}
fn ebreak(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::EBREAK(rd, imm18) = instr else { panic!() };
    todo!()
}
fn sret(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SRET(rd, imm18) = instr else { panic!() };
    todo!()
}
fn wfi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::WFI(rd, imm18) = instr else { panic!() };
    todo!()
}
fn csrw(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CSRW(rs1, rs2, imm12) = instr else { panic!() };
    todo!()
}
fn csrr(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CSRR(rd, rs1, imm12) = instr else { panic!() };
    todo!()
}
