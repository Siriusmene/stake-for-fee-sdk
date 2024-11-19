use solana_program::{
    decode_error::DecodeError, msg, program_error::{PrintProgramError, ProgramError},
};
use thiserror::Error;
#[derive(Clone, Copy, Debug, Eq, Error, num_derive::FromPrimitive, PartialEq)]
pub enum StakeForFeeError {
    #[error("Escrow owner is not vault")]
    InvalidEscrowOwner = 6000,
    #[error("Invalid top list length")]
    InvalidTopListLength = 6001,
    #[error("Invalid seconds to full unlock")]
    InvalidSecondsToFullUnlock = 6002,
    #[error("Pool missing SOL/USDC token or invalid stake mint")]
    MustHaveQuoteTokenOrInvalidStakeMint = 6003,
    #[error("Missing dropped stake escrow")]
    MissingDroppedStakeEscrow = 6004,
    #[error("Invalid stake escrow")]
    InvalidStakeEscrow = 6005,
    #[error("Full balance list is full")]
    FullBalanceListFull = 6006,
    #[error("Mint does not belong to the pool")]
    MintDoesNotBelongToPool = 6007,
    #[error("Insufficient stake amount")]
    InsufficientStakeAmount = 6008,
    #[error("Unstake amount release date not reached")]
    CannotWithdrawUnstakeAmount = 6009,
    #[error("Invalid admin")]
    InvalidAdmin = 6010,
    #[error("Invalid unstake lock duration")]
    InvalidUnstakeLockDuration = 6011,
    #[error("Invalid join window duration")]
    InvalidJoinWindowDuration = 6012,
    #[error("Invalid custom start fee distribute timestamp")]
    InvalidCustomStartFeeDistributeTimestamp = 6013,
    #[error("Invalid smallest stake escrow")]
    InvalidSmallestStakeEscrow = 6014,
    #[error("MathOverflow")]
    MathOverflow = 6015,
    #[error("Type casting failed")]
    TypeCastFailed = 6016,
    #[error("Invalid lock escrow related accounts")]
    InvalidLockEscrowRelatedAccounts = 6017,
    #[error("Only constant product pool is supported")]
    OnlyConstantProductPool = 6018,
    #[error("Undetermined error")]
    UndeterminedError = 6019,
    #[error("Missing smallest stake escrow")]
    MissingSmallestStakeEscrow = 6020,
    #[error("Updated value is the same as old value")]
    UpdatedValueIsTheSame = 6021,
    #[error("Invalid fee crank instruction")]
    InvalidFeeCrankIx = 6022,
}
impl From<StakeForFeeError> for ProgramError {
    fn from(e: StakeForFeeError) -> Self {
        ProgramError::Custom(e as u32)
    }
}
impl<T> DecodeError<T> for StakeForFeeError {
    fn type_of() -> &'static str {
        "StakeForFeeError"
    }
}
impl PrintProgramError for StakeForFeeError {
    fn print<E>(&self)
    where
        E: 'static + std::error::Error + DecodeError<E> + PrintProgramError
            + num_traits::FromPrimitive,
    {
        msg!(& self.to_string());
    }
}
