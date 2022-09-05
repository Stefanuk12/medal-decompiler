use nom::{
    error::{Error, ErrorKind, ParseError},
    Err, IResult,
};
use num_traits::ToPrimitive;

use argument::{Constant, Function, Register, RegisterOrConstant, Upvalue};
use layout::Layout;
use operation_code::OperationCode;

pub mod argument;
mod layout;
mod operation_code;
pub mod position;

#[derive(Debug)]
struct RawInstruction(OperationCode, Layout);

impl RawInstruction {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let operation_code = OperationCode::parse(input).map(|r| r.1)?;
        let (input, layout) = Layout::parse(input, operation_code.to_u8().unwrap())?;

        Ok((input, Self(operation_code, layout)))
    }
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Move {
        destination: Register,
        source: Register,
    },
    LoadConstant {
        destination: Register,
        source: Constant,
    },
    LoadBoolean {
        destination: Register,
        value: bool,
        skip_next: bool,
    },
    LoadNil(Vec<Register>),
    GetUpvalue {
        destination: Register,
        upvalue: Upvalue,
    },
    GetGlobal {
        destination: Register,
        global: Constant,
    },
    GetTable {
        destination: Register,
        table: Register,
        key: RegisterOrConstant,
    },
    SetGlobal {
        destination: Constant,
        value: Register,
    },
    SetUpvalue {
        destination: Upvalue,
        source: Register,
    },
    SetTable {
        table: Register,
        key: RegisterOrConstant,
        value: RegisterOrConstant,
    },
    NewTable {
        destination: Register,
        array_size: u8,
        hash_size: u8,
    },
    Self_ {
        destination: Register,
        table: Register,
        method: RegisterOrConstant,
    },
    Add {
        destination: Register,
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
    },
    Sub {
        destination: Register,
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
    },
    Mul {
        destination: Register,
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
    },
    Div {
        destination: Register,
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
    },
    Mod {
        destination: Register,
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
    },
    Pow {
        destination: Register,
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
    },
    Minus {
        destination: Register,
        operand: Register,
    },
    Not {
        destination: Register,
        operand: Register,
    },
    Length {
        destination: Register,
        operand: Register,
    },
    Concatenate {
        destination: Register,
        operands: Vec<Register>,
    },
    Jump(i32),
    Equal {
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
        comparison_value: bool,
    },
    LessThan {
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
        comparison_value: bool,
    },
    LessThanOrEqual {
        lhs: RegisterOrConstant,
        rhs: RegisterOrConstant,
        comparison_value: bool,
    },
    Test {
        value: Register,
        comparison_value: bool,
    },
    TestSet {
        destination: Register,
        value: Register,
        comparison_value: bool,
    },
    Call {
        function: Register,
        arguments: u8,
        return_values: u8,
    },
    TailCall {
        function: Register,
        arguments: u8,
    },
    Return(Register, u8),
    IterateNumericForLoop {
        control: Vec<Register>,
        step: i32,
    },
    PrepareNumericForLoop {
        control: Vec<Register>,
        step: i32,
    },
    IterateGenericForLoop {
        iterator: Register,
        number_of_variables: u8,
    },
    SetList {
        table: Register,
        number_of_elements: u8,
        block_number: u8,
    },
    Close(Register),
    Closure {
        destination: Register,
        function: Function,
    },
    VarArg(Register, u8),
}

impl Instruction {
    pub fn parse(input: &[u8]) -> IResult<&[u8], Self> {
        let (input, instruction) = RawInstruction::parse(input)?;
        let instruction = match instruction {
            RawInstruction(OperationCode::Move, Layout::ABC { a, b, .. }) => Self::Move {
                destination: Register(a),
                source: Register(b as u8),
            },
            RawInstruction(OperationCode::LoadConstant, Layout::ABx { a, bx }) => {
                Self::LoadConstant {
                    destination: Register(a),
                    source: Constant(bx),
                }
            }
            RawInstruction(OperationCode::LoadBoolean, Layout::ABC { a, b, c }) => {
                Self::LoadBoolean {
                    destination: Register(a),
                    value: b == 1,
                    skip_next: c == 1,
                }
            }
            RawInstruction(OperationCode::LoadNil, Layout::ABC { a, b, .. }) => {
                Self::LoadNil((a..=b as u8).map(Register).collect())
            }
            RawInstruction(OperationCode::GetUpvalue, Layout::ABC { a, b, .. }) => {
                Self::GetUpvalue {
                    destination: Register(a),
                    upvalue: Upvalue(b as u8),
                }
            }
            RawInstruction(OperationCode::GetGlobal, Layout::ABx { a, bx }) => Self::GetGlobal {
                destination: Register(a),
                global: Constant(bx),
            },
            RawInstruction(OperationCode::GetTable, Layout::ABC { a, b, c }) => Self::GetTable {
                destination: Register(a),
                table: Register(b as u8),
                key: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::SetGlobal, Layout::ABx { a, bx }) => Self::SetGlobal {
                destination: Constant(bx),
                value: Register(a),
            },
            RawInstruction(OperationCode::SetTable, Layout::ABC { a, b, c }) => Self::SetTable {
                table: Register(a),
                key: RegisterOrConstant::from(b as u32),
                value: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::NewTable, Layout::ABC { a, b, c }) => Self::NewTable {
                destination: Register(a),
                array_size: b as u8,
                hash_size: c as u8,
            },
            RawInstruction(OperationCode::Self_, Layout::ABC { a, b, c }) => Self::Self_ {
                destination: Register(a),
                table: Register(b as u8),
                method: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::Add, Layout::ABC { a, b, c }) => Self::Add {
                destination: Register(a),
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::Subtract, Layout::ABC { a, b, c }) => Self::Sub {
                destination: Register(a),
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::Multiply, Layout::ABC { a, b, c }) => Self::Mul {
                destination: Register(a),
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::Divide, Layout::ABC { a, b, c }) => Self::Div {
                destination: Register(a),
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::Modulo, Layout::ABC { a, b, c }) => Self::Mod {
                destination: Register(a),
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::Power, Layout::ABC { a, b, c }) => Self::Pow {
                destination: Register(a),
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
            },
            RawInstruction(OperationCode::Minus, Layout::ABC { a, b, .. }) => Self::Minus {
                destination: Register(a),
                operand: Register(b as u8),
            },
            RawInstruction(OperationCode::Not, Layout::ABC { a, b, c: _ }) => Self::Not {
                destination: Register(a),
                operand: Register(b as u8),
            },
            RawInstruction(OperationCode::Length, Layout::ABC { a, b, c: _ }) => Self::Length {
                destination: Register(a),
                operand: Register(b as u8),
            },
            RawInstruction(OperationCode::Concatenate, Layout::ABC { a, b, c }) => {
                Self::Concatenate {
                    destination: Register(a),
                    operands: (b..c).map(|r| Register(r as u8)).collect(),
                }
            }
            RawInstruction(OperationCode::Jump, Layout::AsBx { sbx, .. }) => Self::Jump(sbx),
            RawInstruction(OperationCode::Equal, Layout::ABC { a, b, c }) => Self::Equal {
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
                comparison_value: a == 1,
            },
            RawInstruction(OperationCode::LessThan, Layout::ABC { a, b, c }) => Self::LessThan {
                lhs: RegisterOrConstant::from(b as u32),
                rhs: RegisterOrConstant::from(c as u32),
                comparison_value: a == 1,
            },
            RawInstruction(OperationCode::LessThanOrEqual, Layout::ABC { a, b, c }) => {
                Self::LessThanOrEqual {
                    lhs: RegisterOrConstant::from(b as u32),
                    rhs: RegisterOrConstant::from(c as u32),
                    comparison_value: a == 1,
                }
            }
            RawInstruction(OperationCode::Test, Layout::ABC { a, c, .. }) => Self::Test {
                value: Register(a),
                comparison_value: c == 1,
            },
            RawInstruction(OperationCode::TestSet, Layout::ABC { a, b, c }) => Self::TestSet {
                destination: Register(a),
                value: Register(b as u8),
                comparison_value: c == 1,
            },
            RawInstruction(OperationCode::Call, Layout::ABC { a, b, c }) => Self::Call {
                function: Register(a),
                arguments: b as u8,
                return_values: c as u8,
            },
            RawInstruction(OperationCode::TailCall, Layout::ABC { a, b, .. }) => Self::TailCall {
                function: Register(a),
                arguments: b as u8,
            },
            RawInstruction(OperationCode::Return, Layout::ABC { a, b, .. }) => {
                Self::Return(Register(a), b as u8)
            }
            RawInstruction(OperationCode::IterateNumericForLoop, Layout::AsBx { a, sbx }) => {
                Self::IterateNumericForLoop {
                    control: (a..=a + 4).map(Register).collect(),
                    step: sbx,
                }
            }
            RawInstruction(OperationCode::PrepareNumericForLoop, Layout::AsBx { a, sbx }) => {
                Self::PrepareNumericForLoop {
                    control: (a..=a + 4).map(Register).collect(),
                    step: sbx,
                }
            }
            RawInstruction(OperationCode::IterateGenericForLoop, Layout::ABC { a, c, .. }) => {
                Self::IterateGenericForLoop {
                    iterator: Register(a),
                    number_of_variables: c as u8,
                }
            }
            RawInstruction(OperationCode::SetList, Layout::ABC { a, b, c }) => Self::SetList {
                table: Register(a),
                number_of_elements: b as u8,
                block_number: c as u8,
            },
            RawInstruction(OperationCode::Close, Layout::ABC { a, .. }) => Self::Close(Register(a)),
            RawInstruction(OperationCode::Closure, Layout::ABx { a, bx }) => Self::Closure {
                destination: Register(a),
                function: Function(bx),
            },
            RawInstruction(OperationCode::VarArg, Layout::ABC { a, b, .. }) => {
                Self::VarArg(Register(a), b as u8)
            }
            _ => {
                return Err(Err::Failure(Error::from_error_kind(
                    input,
                    ErrorKind::Switch,
                )))
            }
        };

        Ok((input, instruction))
    }
}
