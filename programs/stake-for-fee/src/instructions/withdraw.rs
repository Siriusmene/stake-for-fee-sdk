use crate::states::{FeeVault, StakeEscrow, Unstake};
use anchor_lang::prelude::*;
use anchor_spl::token::{Token, TokenAccount};

#[derive(Accounts)]
#[event_cpi]
pub struct Withdraw<'info> {
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
    pub stake_token_vault: Box<Account<'info, TokenAccount>>,

    #[account(
        mut,
        has_one = stake_token_vault,
    )]
    pub vault: AccountLoader<'info, FeeVault>,

    #[account(mut)]
    pub user_stake_token: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub token_program: Program<'info, Token>,
}
