#[derive(Debug)]
pub enum ExceptionType {
    ShiftOverflow,
    PageFault,
    DivByZero,
    IllegalInstr,
    ECall,
    EBreak,
    SRet,
}
