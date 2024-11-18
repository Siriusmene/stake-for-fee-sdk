use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeVaultParams {
    pub top_list_length: u16,
    pub seconds_to_full_unlock: u64,
    pub unstake_lock_duration: u64,
    pub start_fee_distribute_timestamp: Option<u64>,
    pub padding: [u8; 64],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakerBalance {
    pub balance: u64,
    pub owner: Pubkey,
    pub is_in_top_list: u8,
    pub padding: [u8; 7],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakerMetadata {
    pub stake_amount: u64,
    pub full_balance_index: i64,
    pub owner: Pubkey,
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Configuration {
    pub seconds_to_full_unlock: u64,
    pub unstake_lock_duration: u64,
    pub start_fee_distribute_timestamp: i64,
    pub padding0: u64,
    pub padding: [u128; 4],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Metrics {
    pub total_staked_amount: u64,
    pub total_stake_escrow_count: u64,
    pub ongoing_total_partial_unstake_amount: u64,
    pub padding0: u64,
    pub total_fee_a_amount: u128,
    pub total_fee_b_amount: u128,
    pub user_total_claimed_fee_a: u128,
    pub user_total_claimed_fee_b: u128,
    pub padding: [u128; 4],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopStakerInfo {
    pub top_list_length: u64,
    pub current_length: u64,
    pub effective_stake_amount: u64,
    pub last_claim_fee_at: i64,
    pub last_updated_at: i64,
    pub locked_fee_a: u64,
    pub locked_fee_b: u64,
    pub padding0: u64,
    pub cumulative_fee_a_per_liquidity: u128,
    pub cumulative_fee_b_per_liquidity: u128,
    pub padding: [u128; 4],
}
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Rounding {
    Up,
    Down,
}
