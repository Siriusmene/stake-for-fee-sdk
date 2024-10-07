use crate::states::{FeeVault, StakeEscrow, TopListMetadata};
use crate::states::{FullBalanceListMetadata, Unstake};
use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[derive(Accounts)]
#[event_cpi]
pub struct CancelUnstake<'info> {
    #[account(
        mut,
        has_one = stake_escrow,
        close = owner
    )]
    pub unstake: AccountLoader<'info, Unstake>,

    #[account(
        mut,
        has_one = vault,
        has_one = owner,
    )]
    pub stake_escrow: AccountLoader<'info, StakeEscrow>,

    #[account(mut)]
    pub smallest_stake_escrow: Option<AccountLoader<'info, StakeEscrow>>,

    #[account(mut)]
    pub top_staker_list: AccountLoader<'info, TopListMetadata>,

    #[account(mut)]
    pub full_balance_list: AccountLoader<'info, FullBalanceListMetadata>,

    #[account(
        mut,
        has_one = pool,
        has_one = lock_escrow,
        has_one = token_a_vault,
        has_one = token_b_vault,
        has_one = top_staker_list,
        has_one = full_balance_list,
    )]
    pub vault: AccountLoader<'info, FeeVault>,

    #[account(mut)]
    pub token_a_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub token_b_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    /// CHECK: Dynamic AMM Pool
    #[account(mut)]
    pub pool: UncheckedAccount<'info>,

    #[account(mut)]
    pub lp_mint: Box<Account<'info, Mint>>,

    /// CHECK: Dynamic AMM LockEscrow
    #[account(mut)]
    pub lock_escrow: UncheckedAccount<'info>,

    /// CHECK: Dynamic AMM LockEscrow vault
    #[account(mut)]
    pub escrow_vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub a_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub b_token_vault: Box<Account<'info, TokenAccount>>,

    /// CHECK: Dynamic Vault of Dynamic AMM pool
    #[account(mut)]
    pub a_vault: UncheckedAccount<'info>,

    /// CHECK: Dynamic Vault of Dynamic AMM pool
    #[account(mut)]
    pub b_vault: UncheckedAccount<'info>,

    #[account(mut)]
    pub a_vault_lp: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub b_vault_lp: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub a_vault_lp_mint: Box<Account<'info, Mint>>,

    #[account(mut)]
    pub b_vault_lp_mint: Box<Account<'info, Mint>>,

    /// CHECK: Dynamic AMM Program
    pub amm_program: UncheckedAccount<'info>,
    /// CHECK: Dynamic Vault Program
    pub vault_program: UncheckedAccount<'info>,
    pub token_program: Program<'info, Token>,
}
