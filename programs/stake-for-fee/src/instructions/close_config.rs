use crate::states::Config;
use anchor_lang::prelude::*;

#[derive(Accounts)]
#[event_cpi]
pub struct CloseConfig<'info> {
    #[account(
        mut,
        close = admin
    )]
    pub config: AccountLoader<'info, Config>,

    #[account(mut)]
    pub admin: Signer<'info>,
}
