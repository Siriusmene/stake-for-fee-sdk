use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(InitSpace, Debug, Default)]
pub struct Unstake {
    pub stake_escrow: Pubkey,
    pub unstake_amount: u64,
    pub created_at: i64,
    pub release_at: i64,
    pub _padding: [u64; 30],
}
