use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    OptionsTooLong,
    OracleCapReached,
    NoOracles
}