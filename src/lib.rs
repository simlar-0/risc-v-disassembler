mod helpers;
mod registers;
mod conditions;
mod instructions;

#[derive(Debug, PartialEq)]
pub enum Error {
    InvalidCondition,
}