use crate::pda::*;
use crate::states::TopListMetadata;
use crate::states::{Config, FeeVault, FullBalanceListMetadata};
use anchor_lang::prelude::*;
use anchor_spl::associated_token::AssociatedToken;
use anchor_spl::token::{Mint, Token, TokenAccount};

#[event_cpi]
#[derive(Accounts)]
pub struct InitializeVault<'info> {
    /// Config account
    pub config: AccountLoader<'info, Config>,

    /// Vault account
    #[account(
        init,
        seeds = [
            VAULT_SEED,
            pool.key().as_ref(),
        ],
        bump,
        space = 8 + FeeVault::INIT_SPACE,
        payer = payer,
    )]
    pub vault: AccountLoader<'info, FeeVault>,

    /// Stake token vault
    #[account(
        init,
        seeds = [
            STAKE_VAULT_SEED,
            vault.key().as_ref()
        ],
        bump,
        payer = payer,
        token::mint = stake_mint,
        token::authority = vault
    )]
    pub stake_token_vault: Box<Account<'info, TokenAccount>>,

    /// Token a vault
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_a_mint,
        associated_token::authority = vault
    )]
    pub token_a_vault: Box<Account<'info, TokenAccount>>,

    /// Token b vault
    #[account(
        init_if_needed,
        payer = payer,
        associated_token::mint = token_b_mint,
        associated_token::authority = vault
    )]
    pub token_b_vault: Box<Account<'info, TokenAccount>>,

    /// Top staker list account
    #[account(
        init,
        seeds = [
            LIST_SEED,
            vault.key().as_ref(),
        ],
        bump,
        space = 8 + TopListMetadata::INIT_SPACE,
        payer = payer
    )]
    pub top_staker_list: AccountLoader<'info, TopListMetadata>,

    /// Full balance list account
    #[account(
        init,
        seeds = [
            BALANCE_SEED,
            vault.key().as_ref(),
        ],
        bump,
        space = 8 + FullBalanceListMetadata::INIT_SPACE,
        payer = payer,
    )]
    pub full_balance_list: AccountLoader<'info, FullBalanceListMetadata>,

    /// CHECK: Dynamic AMM Pool
    pub pool: UncheckedAccount<'info>,

    /// Token a mint of the pool
    pub token_a_mint: Box<Account<'info, Mint>>,

    /// Token b mint of the pool
    pub token_b_mint: Box<Account<'info, Mint>>,

    /// Stake mint of the pool
    pub stake_mint: Box<Account<'info, Mint>>,

    /// CHECK: Dynamic AMM LockEscrow
    pub lock_escrow: UncheckedAccount<'info>,

    #[account(mut)]
    pub payer: Signer<'info>,

    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}
