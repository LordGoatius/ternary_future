use crate::{instr::{Word, dec::JTC_1701}, machine::{Machine, err::ExecErr}};

// TODO:
// Exceptional Control Flow:
// Return or Trap Dispatch FN?
// Best Option: Likely return, with
// separate coroutines for handling,
// and then dispatch the core again?

type DispatchFn = fn(&mut Machine, instr: JTC_1701, pc: Word) -> Word;

const DEFAULT_TABLE_SIZE: usize = 36;
const MUL_TABLE_SIZE: usize = 6;

const DISPATCH_TABLE_SIZE: usize = {
    DEFAULT_TABLE_SIZE + 1 + {
        if cfg!(feature = "ext_mul") {
            MUL_TABLE_SIZE
        } else {
            0
        }
    }
};

const TRAP: usize = DISPATCH_TABLE_SIZE - 1;

static TABLE: [DispatchFn; DISPATCH_TABLE_SIZE] = [
    add,
    sub,
    and,
    or,
    xor,
    shr,
    shl,
    cmp,
    #[cfg(feature = "ext_mul")]
    mul,
    #[cfg(feature = "ext_mul")]
    div,
    #[cfg(feature = "ext_mul")]
    rem,
    addi,
    subi,
    andi,
    ori,
    xori,
    shri,
    shli,
    cmpi,
    #[cfg(feature = "ext_mul")]
    muli,
    #[cfg(feature = "ext_mul")]
    divi,
    #[cfg(feature = "ext_mul")]
    remi,
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
    trap,
];

fn add(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ADD(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.add(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn sub(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SUB(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.sub(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn and(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::AND(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.and(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn or(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::OR(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.or(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn xor(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::XOR(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.xor(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn shr(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHR(rd, rs1, rs2) = instr else { panic!() };
    let pc_res = machine.shr(rd, rs1, rs2, pc);
    match pc_res {
        Ok(pc)   => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        // PC is not incrememented during TRAPS
        Err(ExecErr::ShiftOverflow) => {
            // let next = machine.next(pc);
            return pc;
            // become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}
fn shl(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHL(rd, rs1, rs2) = instr else { panic!() };
    let pc_res = machine.shl(rd, rs1, rs2, pc);
    match pc_res {
        Ok(pc)   => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        // PC is not incrememented during TRAPS
        Err(ExecErr::ShiftOverflow) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}
fn cmp(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CMP(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.cmp(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn mul(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::MUL(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.mul(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn div(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::DIV(rd, rs1, rs2) = instr else { panic!() };
    let pc_res = machine.div(rd, rs1, rs2, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        Err(ExecErr::DivByZero) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}
fn rem(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::REM(rd, rs1, rs2) = instr else { panic!() };
    let pc_res = machine.rem(rd, rs1, rs2, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        Err(ExecErr::DivByZero) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}

fn addi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ADDI(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.addi(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn subi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SUBI(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.subi(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn andi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ANDI(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.andi(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn ori(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ORI(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.ori(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn xori(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::XORI(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.xori(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn shri(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHRI(rd, rs1, imm12) = instr else { panic!() };
    let pc_res = machine.shri(rd, rs1, imm12, pc);
    match pc_res {
        Ok(pc)   => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        // PC is not incrememented during TRAPS
        Err(ExecErr::ShiftOverflow) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}
fn shli(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SHLI(rd, rs1, imm12) = instr else { panic!() };
    let pc_res = machine.shli(rd, rs1, imm12, pc);
    match pc_res {
        Ok(pc)   => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        // PC is not incrememented during TRAPS
        Err(ExecErr::ShiftOverflow) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}
fn cmpi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CMPI(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.cmpi(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn muli(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::MULI(rd, rs1, rs2) = instr else { panic!() };
    let pc = machine.muli(rd, rs1, rs2, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn divi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::DIVI(rd, rs1, rs2) = instr else { panic!() };
    let pc_res = machine.divi(rd, rs1, rs2, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        Err(ExecErr::DivByZero) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}
fn remi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::REMI(rd, rs1, rs2) = instr else { panic!() };
    let pc_res = machine.remi(rd, rs1, rs2, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        },
        Err(ExecErr::DivByZero) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        },
        _ => unreachable!()
    }
}

fn beq(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BEQ(rs1, rs2, imm12) = instr else { panic!() };
    let pc = machine.beq(rs1, rs2, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn blt(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BLT(rs1, rs2, imm12) = instr else { panic!() };
    let pc = machine.blt(rs1, rs2, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn bgt(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BGT(rs1, rs2, imm12) = instr else { panic!() };
    let pc = machine.bgt(rs1, rs2, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn bne(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BNE(rs1, rs2, imm12) = instr else { panic!() };
    let pc = machine.bne(rs1, rs2, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn bleq(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BLEQ(rs1, rs2, imm12) = instr else { panic!() };
    let pc = machine.bleq(rs1, rs2, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn bgeq(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::BGEQ(rs1, rs2, imm12) = instr else { panic!() };
    let pc = machine.bgeq(rs1, rs2, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn lui(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::LUI(rd, imm18) = instr else { panic!() };
    let pc = machine.lui(rd, imm18, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn auipc(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::AUIPC(rd, imm18) = instr else { panic!() };
    let pc = machine.auipc(rd, imm18, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn lt(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::LT(rd, rs1, imm12) = instr else { panic!() };
    let pc_res = machine.lt(rd, rs1, imm12, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        }
        Err(ExecErr::PageFault) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        }
        _ => unreachable!()
    }
}
fn lw(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::LW(rd, rs1, imm12) = instr else { panic!() };
    let pc_res = machine.lw(rd, rs1, imm12, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        }
        Err(ExecErr::PageFault) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        }
        _ => unreachable!()
    }
}
fn st(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ST(rs1, rs2, imm12) = instr else { panic!() };
    let pc_res = machine.st(rs1, rs2, imm12, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        }
        Err(ExecErr::PageFault) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        }
        _ => unreachable!()
    }
}
fn sw(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SW(rs1, rs2, imm12) = instr else { panic!() };
    let pc_res = machine.sw(rs1, rs2, imm12, pc);
    match pc_res {
        Ok(pc) => {
            let next = machine.next(pc);
            become TABLE[next.disc() as usize](machine, next, pc)
        }
        Err(ExecErr::PageFault) => {
            let next = machine.next(pc);
            become TABLE[TRAP](machine, next, pc)
        }
        _ => unreachable!()
    }
}
fn jal(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::JAL(rd, imm18) = instr else { panic!() };
    let pc = machine.jal(rd, imm18, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn jalr(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::JALR(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.jalr(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}

fn ecall(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::ECALL(rd, imm18) = instr else { panic!() };
    let pc = machine.ecall(rd, imm18, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn ebreak(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::EBREAK(rd, imm18) = instr else { panic!() };
    let pc = machine.ebreak(rd, imm18, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn sret(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::SRET(rd, imm18) = instr else { panic!() };
    let pc = machine.sret(rd, imm18, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn wfi(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::WFI(rd, imm18) = instr else { panic!() };
    let pc = machine.wfi(rd, imm18, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn csrw(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CSRW(rs1, rs2, imm12) = instr else { panic!() };
    let pc = machine.csrw(rs1, rs2, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn csrr(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    let JTC_1701::CSRR(rd, rs1, imm12) = instr else { panic!() };
    let pc = machine.csrr(rd, rs1, imm12, pc);
    let next = machine.next(pc);
    become TABLE[next.disc() as usize](machine, next, pc)
}
fn trap(machine: &mut Machine, instr: JTC_1701, pc: Word) -> Word {
    todo!()
}
