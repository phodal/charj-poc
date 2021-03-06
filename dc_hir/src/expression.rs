use dc_lexer::Location;
use num_bigint::BigInt;

use crate::Type;

#[derive(PartialEq, Clone, Debug)]
pub enum Builtin {
    Assert,
    Print,
}

#[derive(Clone, Debug)]
pub enum Expression {
    Placeholder,
    Variable {
        location: Location,
        ty: Type,
        // change to symbol table
        value: String,
    },
    StringLiteral {
        location: Location,
        value: String,
    },
    NumberLiteral {
        location: Location,
        ty: Type,
        value: BigInt,
    },
    BytesLiteral {
        location: Location,
        ty: Type,
        value: Vec<u8>,
    },
    InternalFunctionCall {
        location: Location,
        function: Box<Expression>,
        args: Vec<Expression>,
    },
    Builtin {
        location: Location,
        types: Vec<Type>,
        builtin: Builtin,
        args: Vec<Expression>,
    },
}
