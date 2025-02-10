mod macros;
mod variable_bit_integers;
mod registers;
mod conditions;
mod instructions;
mod decoder;
mod parser;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCondition,
}