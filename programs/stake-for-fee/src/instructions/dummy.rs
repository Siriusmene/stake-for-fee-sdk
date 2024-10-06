use crate::states::{StakerBalance, StakerMetadata};
use anchor_lang::prelude::*;

#[account(zero_copy)]
pub struct StakerMetadataDummyAccount {
    // This trick IDL anchor to generate it as IdlTypes for typescript
    pub staker_metadata: StakerMetadata,
}

#[account(zero_copy)]
pub struct StakerBalanceDummyAccount {
    // This trick IDL anchor to generate it as IdlTypes for typescript
    pub staker_balance: StakerBalance,
}

// To force anchor to generate IDL for StakerMetadata
#[derive(Accounts)]
pub struct Dummy<'info> {
    pub staker_metadata: AccountLoader<'info, StakerMetadataDummyAccount>,
    pub staker_balance: AccountLoader<'info, StakerBalanceDummyAccount>,
}
