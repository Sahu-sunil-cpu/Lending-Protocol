use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
#[msg("insufficient funds to withdraw")]
Insufficientfunds,

#[msg("Over Reapy")]
OverRepay,
}