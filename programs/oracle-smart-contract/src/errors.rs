use anchor_lang::prelude::*;

#[error_code]
pub enum MyError {
    ApiTooLarge,
    HeadersTooLarge,
    ParamsTooLarge,
    OracleCapReached,
    NoOracles
}