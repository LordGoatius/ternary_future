#[allow(non_camel_case_types)]
use super::*;

pub type Reg = Ternary<3>;
pub type Op = Ternary<4>;
pub type Imm18 = Ternary<18>;
pub type Imm12 = Ternary<12>;

pub enum JTC_1701 {
    // Reg Type: rd, rs1, rs2
    ADD   (Reg, Reg, Reg),
    SUB   (Reg, Reg, Reg),
    AND   (Reg, Reg, Reg),
    OR    (Reg, Reg, Reg),
    XOR   (Reg, Reg, Reg),
    SHR   (Reg, Reg, Reg),
    SHL   (Reg, Reg, Reg),
    CMP   (Reg, Reg, Reg),
    // Imm Type: rd, rs1, imm18
    ADDI  (Reg, Reg, Imm12),
    SUBI  (Reg, Reg, Imm12),
    ANDI  (Reg, Reg, Imm12),
    ORI   (Reg, Reg, Imm12),
    XORI  (Reg, Reg, Imm12),
    SHRI  (Reg, Reg, Imm12),
    SHLI  (Reg, Reg, Imm12),
    CMPI  (Reg, Reg, Imm12),
    // Branch Type: rs1, rs2, imm12
    BEQ   (Reg, Reg, Imm12),
    BLT   (Reg, Reg, Imm12),
    BGT   (Reg, Reg, Imm12),
    BNE   (Reg, Reg, Imm12),
    BLEQ  (Reg, Reg, Imm12),
    BGEQ  (Reg, Reg, Imm12),
    // Upper: rd, rs1, imm12
    LUI   (Reg, Reg, Imm12),
    AUIPC (Reg, Reg, Imm12),
    // Loads: rd, rs1, imm12
    LT    (Reg, Reg, Imm12),
    LW    (Reg, Reg, Imm12),
    // Stores: rs1, rs2, imm12
    ST    (Reg, Reg, Imm12),
    SW    (Reg, Reg, Imm12),
    // Jumps: rd, imm18
    JAL   (Reg, Imm18),
    JALR  (Reg, Imm18),
}

pub const ARITH_OP:  Op = Op::from_str("0T01");
pub const LOAD_OP:   Op = Op::from_str("0T00");
pub const BRANCH_OP: Op = Op::from_str("0T11");
pub const IMM_OP:    Op = Op::from_str("0011");
pub const STORE_OP:  Op = Op::from_str("0T10");
pub const JUMP_OP:   Op = Op::from_str("0110");
pub const UPPER_OP:  Op = Op::from_str("0111");

pub mod funct_consts {
    use septivigntimal::*;
    use ternary::Ternary;

    pub const FUNCT4_ADD: Ternary<4> = A.extend();
    pub const FUNCT4_SUB: Ternary<4> = N.extend();
    pub const FUNCT4_AND: Ternary<4> = B.extend();
    pub const FUNCT4_OR:  Ternary<4> = C.extend();
    pub const FUNCT4_XOR: Ternary<4> = D.extend();
    pub const FUNCT4_SHR: Ternary<4> = E.extend();
    pub const FUNCT4_SHL: Ternary<4> = F.extend();
    pub const FUNCT4_CMP: Ternary<4> = G.extend();

    pub const FN2_BE:  Ternary<2> = Ternary::from_str("00");
    pub const FN2_BL:  Ternary<2> = Ternary::from_str("TT");
    pub const FN2_BG:  Ternary<2> = Ternary::from_str("11");
    pub const FN2_BN:  Ternary<2> = Ternary::from_str("1T");
    pub const FN2_BLE: Ternary<2> = Ternary::from_str("0");
    pub const FN2_BGE: Ternary<2> = Ternary::from_str("0");

    pub const FN2_ST: Ternary<2> = Ternary::from_str("00");
    pub const FN2_SW: Ternary<2> = Ternary::from_str("01");

    pub const FN2_LT: Ternary<2> = Ternary::from_str("00");
    pub const FN2_LW: Ternary<2> = Ternary::from_str("01");

    pub const FN2_JAL:  Ternary<2> =  Ternary::from_str("00");
    pub const FN2_JALR: Ternary<2> =  Ternary::from_str("01");

    pub const FN2_LUI:   Ternary<2> =  Ternary::from_str("00");
    pub const FN2_AUIPC: Ternary<2> =  Ternary::from_str("01");
}

#[rustfmt::skip]
const fn decode(instr: Word) -> JTC_1701 {
    match instr.slice::<4>(0) {
        ARITH_OP => { todo!() },
        LOAD_OP => { todo!() },
        BRANCH_OP => { todo!() },
        IMM_OP => { todo!() },
        STORE_OP => { todo!() },
        JUMP_OP => { todo!() },
        UPPER_OP => { todo!() },
        _ => panic!("Illegal Instr"),
    }
}
