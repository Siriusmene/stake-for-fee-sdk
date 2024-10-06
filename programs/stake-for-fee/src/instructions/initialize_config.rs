use crate::{pda::CONFIG_SEED, states::Config};
use anchor_lang::prelude::*;

#[derive(AnchorSerialize, AnchorDeserialize, Debug)]
pub struct InitializeConfigParams {
    pub top_list_length: u16,
    pub index: u64,
    pub seconds_to_full_unlock: u64,
    pub unstake_lock_duration: u64,
    pub join_window_duration: u64,
}

#[derive(Accounts)]
#[event_cpi]
#[instruction(params: InitializeConfigParams)]
pub struct InitializeConfig<'info> {
    #[account(
        init,
        seeds = [
            CONFIG_SEED,
            params.index.to_le_bytes().as_ref()
        ],
        bump,
        space = 8 + Config::INIT_SPACE,
        payer = admin
    )]
    pub config: AccountLoader<'info, Config>,

    #[account(mut)]
    pub admin: Signer<'info>,

    pub system_program: Program<'info, System>,
}
