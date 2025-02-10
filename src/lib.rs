mod macros;
mod helpers;
mod registers;
mod conditions;
mod instructions;
mod decoder;
mod parser;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCondition,
}