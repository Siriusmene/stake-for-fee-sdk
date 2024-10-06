use crate::pda::*;
use crate::states::{FeeVault, FullBalanceListMetadata, StakeEscrow, TopListMetadata};
use anchor_lang::prelude::*;

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeStakeEscrow<'info> {
    #[account(mut)]
    pub vault: AccountLoader<'info, FeeVault>,

    #[account(
        init,
        seeds = [
            ESCROW_SEED,
            vault.key().as_ref(),
            owner.key().as_ref(),
        ],
        space = 8 + StakeEscrow::INIT_SPACE,
        bump,
        payer = owner
    )]
    pub escrow: AccountLoader<'info, StakeEscrow>,

    #[account(
        mut,
        has_one = vault,

    )]
    pub full_balance_list: AccountLoader<'info, FullBalanceListMetadata>,

    #[account(
        mut,
        has_one = vault,

    )]
    pub top_staker_list: AccountLoader<'info, TopListMetadata>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}
