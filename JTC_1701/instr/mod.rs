pub mod enc;
pub mod dec;

use ternary::Ternary;

pub type Word = Ternary<27>;
pub type Tryte = Ternary<6>;

#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct RInstr(Word);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct IInstr(Word);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct SInstr(Word);
#[repr(transparent)]
#[derive(Clone, Copy)]
pub struct UInstr(Word);
