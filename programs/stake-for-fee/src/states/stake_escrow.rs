use anchor_lang::prelude::*;

#[account(zero_copy)]
#[derive(InitSpace, Debug, Default)]
pub struct StakeEscrow {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub full_balance_index: u64,
    pub stake_amount: u64,
    pub in_top_list: u8,
    pub padding_0: [u8; 15],
    pub ongoing_total_partial_unstake_amount: u64,
    pub created_at: i64,
    pub fee_a_claimed_amount: u128,
    pub fee_b_claimed_amount: u128,
    pub fee_a_per_liquidity_checkpoint: u128,
    pub fee_b_per_liquidity_checkpoint: u128,
    pub fee_a_pending: u64,
    pub fee_b_pending: u64,
    pub padding: [u128; 20],
}
