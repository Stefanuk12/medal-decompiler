use num_enum::TryFromPrimitive;

#[repr(u8)]
#[derive(Debug, TryFromPrimitive, Eq, PartialEq, Copy, Clone)]
pub enum OpCode {
    Move = 0,
    LoadConst,
    LoadBool,
    LoadNil,
    GetUpvalue,
    GetGlobal,
    Index,
    SetGlobal,
    SetUpvalue,
    NewIndex,
    NewTable,
    Self_,
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    UnaryMinus,
    Not,
    Len,
    Concat,
    Jump,
    Equal,
    LesserThan,
    LesserOrEqual,
    Test,
    TestSet,
    Call,
    TailCall,
    Return,
    ForLoop,
    ForPrep,
    TableForLoop,
    SetList,
    Close,
    Closure,
    VarArg,
}