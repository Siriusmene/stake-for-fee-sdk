use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::AccountInfo, entrypoint::ProgramResult,
    instruction::{AccountMeta, Instruction},
    program::{invoke, invoke_signed},
    pubkey::Pubkey, program_error::ProgramError,
};
use std::io::Read;
use crate::*;
#[derive(Clone, Debug, PartialEq)]
pub enum StakeForFeeProgramIx {
    InitializeVault(InitializeVaultIxArgs),
    InitializeStakeEscrow,
    Stake(StakeIxArgs),
    ClaimFee(ClaimFeeIxArgs),
    RequestUnstake(RequestUnstakeIxArgs),
    CancelUnstake,
    Withdraw,
    ClaimFeeCrank,
    UpdateUnstakeLockDuration(UpdateUnstakeLockDurationIxArgs),
    UpdateSecondsToFullUnlock(UpdateSecondsToFullUnlockIxArgs),
    Dummy,
}
impl StakeForFeeProgramIx {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        match maybe_discm {
            INITIALIZE_VAULT_IX_DISCM => {
                Ok(
                    Self::InitializeVault(
                        InitializeVaultIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            INITIALIZE_STAKE_ESCROW_IX_DISCM => Ok(Self::InitializeStakeEscrow),
            STAKE_IX_DISCM => Ok(Self::Stake(StakeIxArgs::deserialize(&mut reader)?)),
            CLAIM_FEE_IX_DISCM => {
                Ok(Self::ClaimFee(ClaimFeeIxArgs::deserialize(&mut reader)?))
            }
            REQUEST_UNSTAKE_IX_DISCM => {
                Ok(Self::RequestUnstake(RequestUnstakeIxArgs::deserialize(&mut reader)?))
            }
            CANCEL_UNSTAKE_IX_DISCM => Ok(Self::CancelUnstake),
            WITHDRAW_IX_DISCM => Ok(Self::Withdraw),
            CLAIM_FEE_CRANK_IX_DISCM => Ok(Self::ClaimFeeCrank),
            UPDATE_UNSTAKE_LOCK_DURATION_IX_DISCM => {
                Ok(
                    Self::UpdateUnstakeLockDuration(
                        UpdateUnstakeLockDurationIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            UPDATE_SECONDS_TO_FULL_UNLOCK_IX_DISCM => {
                Ok(
                    Self::UpdateSecondsToFullUnlock(
                        UpdateSecondsToFullUnlockIxArgs::deserialize(&mut reader)?,
                    ),
                )
            }
            DUMMY_IX_DISCM => Ok(Self::Dummy),
            _ => {
                Err(
                    std::io::Error::new(
                        std::io::ErrorKind::Other,
                        format!("discm {:?} not found", maybe_discm),
                    ),
                )
            }
        }
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        match self {
            Self::InitializeVault(args) => {
                writer.write_all(&INITIALIZE_VAULT_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::InitializeStakeEscrow => {
                writer.write_all(&INITIALIZE_STAKE_ESCROW_IX_DISCM)
            }
            Self::Stake(args) => {
                writer.write_all(&STAKE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::ClaimFee(args) => {
                writer.write_all(&CLAIM_FEE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::RequestUnstake(args) => {
                writer.write_all(&REQUEST_UNSTAKE_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::CancelUnstake => writer.write_all(&CANCEL_UNSTAKE_IX_DISCM),
            Self::Withdraw => writer.write_all(&WITHDRAW_IX_DISCM),
            Self::ClaimFeeCrank => writer.write_all(&CLAIM_FEE_CRANK_IX_DISCM),
            Self::UpdateUnstakeLockDuration(args) => {
                writer.write_all(&UPDATE_UNSTAKE_LOCK_DURATION_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::UpdateSecondsToFullUnlock(args) => {
                writer.write_all(&UPDATE_SECONDS_TO_FULL_UNLOCK_IX_DISCM)?;
                args.serialize(&mut writer)
            }
            Self::Dummy => writer.write_all(&DUMMY_IX_DISCM),
        }
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
fn invoke_instruction<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke(ix, &account_info)
}
fn invoke_instruction_signed<'info, A: Into<[AccountInfo<'info>; N]>, const N: usize>(
    ix: &Instruction,
    accounts: A,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let account_info: [AccountInfo<'info>; N] = accounts.into();
    invoke_signed(ix, &account_info, seeds)
}
pub const INITIALIZE_VAULT_IX_ACCOUNTS_LEN: usize = 15;
#[derive(Copy, Clone, Debug)]
pub struct InitializeVaultAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub stake_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub top_staker_list: &'me AccountInfo<'info>,
    pub full_balance_list: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub stake_mint: &'me AccountInfo<'info>,
    pub quote_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub associated_token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeVaultKeys {
    pub vault: Pubkey,
    pub stake_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub top_staker_list: Pubkey,
    pub full_balance_list: Pubkey,
    pub pool: Pubkey,
    pub stake_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub token_program: Pubkey,
    pub associated_token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeVaultAccounts<'_, '_>> for InitializeVaultKeys {
    fn from(accounts: InitializeVaultAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            stake_token_vault: *accounts.stake_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            top_staker_list: *accounts.top_staker_list.key,
            full_balance_list: *accounts.full_balance_list.key,
            pool: *accounts.pool.key,
            stake_mint: *accounts.stake_mint.key,
            quote_mint: *accounts.quote_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            token_program: *accounts.token_program.key,
            associated_token_program: *accounts.associated_token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeVaultKeys> for [AccountMeta; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeVaultKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.top_staker_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.full_balance_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.stake_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.quote_mint,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.associated_token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]> for InitializeVaultKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            stake_token_vault: pubkeys[1],
            quote_token_vault: pubkeys[2],
            top_staker_list: pubkeys[3],
            full_balance_list: pubkeys[4],
            pool: pubkeys[5],
            stake_mint: pubkeys[6],
            quote_mint: pubkeys[7],
            lock_escrow: pubkeys[8],
            payer: pubkeys[9],
            system_program: pubkeys[10],
            token_program: pubkeys[11],
            associated_token_program: pubkeys[12],
            event_authority: pubkeys[13],
            program: pubkeys[14],
        }
    }
}
impl<'info> From<InitializeVaultAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeVaultAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.stake_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.top_staker_list.clone(),
            accounts.full_balance_list.clone(),
            accounts.pool.clone(),
            accounts.stake_mint.clone(),
            accounts.quote_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.token_program.clone(),
            accounts.associated_token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]>
for InitializeVaultAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; INITIALIZE_VAULT_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            stake_token_vault: &arr[1],
            quote_token_vault: &arr[2],
            top_staker_list: &arr[3],
            full_balance_list: &arr[4],
            pool: &arr[5],
            stake_mint: &arr[6],
            quote_mint: &arr[7],
            lock_escrow: &arr[8],
            payer: &arr[9],
            system_program: &arr[10],
            token_program: &arr[11],
            associated_token_program: &arr[12],
            event_authority: &arr[13],
            program: &arr[14],
        }
    }
}
pub const INITIALIZE_VAULT_IX_DISCM: [u8; 8] = [48, 191, 163, 44, 71, 129, 63, 164];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct InitializeVaultIxArgs {
    pub params: InitializeVaultParams,
}
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeVaultIxData(pub InitializeVaultIxArgs);
impl From<InitializeVaultIxArgs> for InitializeVaultIxData {
    fn from(args: InitializeVaultIxArgs) -> Self {
        Self(args)
    }
}
impl InitializeVaultIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_VAULT_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_VAULT_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(InitializeVaultIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_VAULT_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_vault_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeVaultKeys,
    args: InitializeVaultIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_VAULT_IX_ACCOUNTS_LEN] = keys.into();
    let data: InitializeVaultIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn initialize_vault_ix(
    keys: InitializeVaultKeys,
    args: InitializeVaultIxArgs,
) -> std::io::Result<Instruction> {
    initialize_vault_ix_with_program_id(crate::ID, keys, args)
}
pub fn initialize_vault_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
) -> ProgramResult {
    let keys: InitializeVaultKeys = accounts.into();
    let ix = initialize_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_vault_invoke(
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
) -> ProgramResult {
    initialize_vault_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn initialize_vault_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeVaultKeys = accounts.into();
    let ix = initialize_vault_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_vault_invoke_signed(
    accounts: InitializeVaultAccounts<'_, '_>,
    args: InitializeVaultIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_vault_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn initialize_vault_verify_account_keys(
    accounts: InitializeVaultAccounts<'_, '_>,
    keys: InitializeVaultKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.stake_token_vault.key, keys.stake_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (*accounts.top_staker_list.key, keys.top_staker_list),
        (*accounts.full_balance_list.key, keys.full_balance_list),
        (*accounts.pool.key, keys.pool),
        (*accounts.stake_mint.key, keys.stake_mint),
        (*accounts.quote_mint.key, keys.quote_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.associated_token_program.key, keys.associated_token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_writable_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.stake_token_vault,
        accounts.quote_token_vault,
        accounts.top_staker_list,
        accounts.full_balance_list,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_signer_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_vault_verify_account_privileges<'me, 'info>(
    accounts: InitializeVaultAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_vault_verify_writable_privileges(accounts)?;
    initialize_vault_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct InitializeStakeEscrowAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub escrow: &'me AccountInfo<'info>,
    pub full_balance_list: &'me AccountInfo<'info>,
    pub top_staker_list: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub payer: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct InitializeStakeEscrowKeys {
    pub vault: Pubkey,
    pub escrow: Pubkey,
    pub full_balance_list: Pubkey,
    pub top_staker_list: Pubkey,
    pub owner: Pubkey,
    pub payer: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<InitializeStakeEscrowAccounts<'_, '_>> for InitializeStakeEscrowKeys {
    fn from(accounts: InitializeStakeEscrowAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            escrow: *accounts.escrow.key,
            full_balance_list: *accounts.full_balance_list.key,
            top_staker_list: *accounts.top_staker_list.key,
            owner: *accounts.owner.key,
            payer: *accounts.payer.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<InitializeStakeEscrowKeys>
for [AccountMeta; INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(keys: InitializeStakeEscrowKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.full_balance_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.top_staker_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.payer,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN]>
for InitializeStakeEscrowKeys {
    fn from(pubkeys: [Pubkey; INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            escrow: pubkeys[1],
            full_balance_list: pubkeys[2],
            top_staker_list: pubkeys[3],
            owner: pubkeys[4],
            payer: pubkeys[5],
            system_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<InitializeStakeEscrowAccounts<'_, 'info>>
for [AccountInfo<'info>; INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN] {
    fn from(accounts: InitializeStakeEscrowAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.escrow.clone(),
            accounts.full_balance_list.clone(),
            accounts.top_staker_list.clone(),
            accounts.owner.clone(),
            accounts.payer.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN]>
for InitializeStakeEscrowAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            escrow: &arr[1],
            full_balance_list: &arr[2],
            top_staker_list: &arr[3],
            owner: &arr[4],
            payer: &arr[5],
            system_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const INITIALIZE_STAKE_ESCROW_IX_DISCM: [u8; 8] = [
    67,
    237,
    111,
    110,
    218,
    214,
    29,
    153,
];
#[derive(Clone, Debug, PartialEq)]
pub struct InitializeStakeEscrowIxData;
impl InitializeStakeEscrowIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != INITIALIZE_STAKE_ESCROW_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        INITIALIZE_STAKE_ESCROW_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&INITIALIZE_STAKE_ESCROW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn initialize_stake_escrow_ix_with_program_id(
    program_id: Pubkey,
    keys: InitializeStakeEscrowKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; INITIALIZE_STAKE_ESCROW_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: InitializeStakeEscrowIxData.try_to_vec()?,
    })
}
pub fn initialize_stake_escrow_ix(
    keys: InitializeStakeEscrowKeys,
) -> std::io::Result<Instruction> {
    initialize_stake_escrow_ix_with_program_id(crate::ID, keys)
}
pub fn initialize_stake_escrow_invoke_with_program_id(
    program_id: Pubkey,
    accounts: InitializeStakeEscrowAccounts<'_, '_>,
) -> ProgramResult {
    let keys: InitializeStakeEscrowKeys = accounts.into();
    let ix = initialize_stake_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn initialize_stake_escrow_invoke(
    accounts: InitializeStakeEscrowAccounts<'_, '_>,
) -> ProgramResult {
    initialize_stake_escrow_invoke_with_program_id(crate::ID, accounts)
}
pub fn initialize_stake_escrow_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: InitializeStakeEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: InitializeStakeEscrowKeys = accounts.into();
    let ix = initialize_stake_escrow_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn initialize_stake_escrow_invoke_signed(
    accounts: InitializeStakeEscrowAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    initialize_stake_escrow_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn initialize_stake_escrow_verify_account_keys(
    accounts: InitializeStakeEscrowAccounts<'_, '_>,
    keys: InitializeStakeEscrowKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.escrow.key, keys.escrow),
        (*accounts.full_balance_list.key, keys.full_balance_list),
        (*accounts.top_staker_list.key, keys.top_staker_list),
        (*accounts.owner.key, keys.owner),
        (*accounts.payer.key, keys.payer),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn initialize_stake_escrow_verify_writable_privileges<'me, 'info>(
    accounts: InitializeStakeEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.escrow,
        accounts.full_balance_list,
        accounts.top_staker_list,
        accounts.payer,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn initialize_stake_escrow_verify_signer_privileges<'me, 'info>(
    accounts: InitializeStakeEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.payer] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn initialize_stake_escrow_verify_account_privileges<'me, 'info>(
    accounts: InitializeStakeEscrowAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    initialize_stake_escrow_verify_writable_privileges(accounts)?;
    initialize_stake_escrow_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const STAKE_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct StakeAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub stake_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub top_staker_list: &'me AccountInfo<'info>,
    pub full_balance_list: &'me AccountInfo<'info>,
    pub stake_escrow: &'me AccountInfo<'info>,
    pub smallest_stake_escrow: &'me AccountInfo<'info>,
    pub user_stake_token: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct StakeKeys {
    pub vault: Pubkey,
    pub stake_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub top_staker_list: Pubkey,
    pub full_balance_list: Pubkey,
    pub stake_escrow: Pubkey,
    pub smallest_stake_escrow: Pubkey,
    pub user_stake_token: Pubkey,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub amm_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<StakeAccounts<'_, '_>> for StakeKeys {
    fn from(accounts: StakeAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            stake_token_vault: *accounts.stake_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            top_staker_list: *accounts.top_staker_list.key,
            full_balance_list: *accounts.full_balance_list.key,
            stake_escrow: *accounts.stake_escrow.key,
            smallest_stake_escrow: *accounts.smallest_stake_escrow.key,
            user_stake_token: *accounts.user_stake_token.key,
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            amm_program: *accounts.amm_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<StakeKeys> for [AccountMeta; STAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: StakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.top_staker_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.full_balance_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smallest_stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stake_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; STAKE_IX_ACCOUNTS_LEN]> for StakeKeys {
    fn from(pubkeys: [Pubkey; STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            stake_token_vault: pubkeys[1],
            quote_token_vault: pubkeys[2],
            top_staker_list: pubkeys[3],
            full_balance_list: pubkeys[4],
            stake_escrow: pubkeys[5],
            smallest_stake_escrow: pubkeys[6],
            user_stake_token: pubkeys[7],
            owner: pubkeys[8],
            pool: pubkeys[9],
            lp_mint: pubkeys[10],
            lock_escrow: pubkeys[11],
            escrow_vault: pubkeys[12],
            a_token_vault: pubkeys[13],
            b_token_vault: pubkeys[14],
            a_vault: pubkeys[15],
            b_vault: pubkeys[16],
            a_vault_lp: pubkeys[17],
            b_vault_lp: pubkeys[18],
            a_vault_lp_mint: pubkeys[19],
            b_vault_lp_mint: pubkeys[20],
            amm_program: pubkeys[21],
            vault_program: pubkeys[22],
            token_program: pubkeys[23],
            event_authority: pubkeys[24],
            program: pubkeys[25],
        }
    }
}
impl<'info> From<StakeAccounts<'_, 'info>>
for [AccountInfo<'info>; STAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: StakeAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.stake_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.top_staker_list.clone(),
            accounts.full_balance_list.clone(),
            accounts.stake_escrow.clone(),
            accounts.smallest_stake_escrow.clone(),
            accounts.user_stake_token.clone(),
            accounts.owner.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.amm_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; STAKE_IX_ACCOUNTS_LEN]>
for StakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; STAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            stake_token_vault: &arr[1],
            quote_token_vault: &arr[2],
            top_staker_list: &arr[3],
            full_balance_list: &arr[4],
            stake_escrow: &arr[5],
            smallest_stake_escrow: &arr[6],
            user_stake_token: &arr[7],
            owner: &arr[8],
            pool: &arr[9],
            lp_mint: &arr[10],
            lock_escrow: &arr[11],
            escrow_vault: &arr[12],
            a_token_vault: &arr[13],
            b_token_vault: &arr[14],
            a_vault: &arr[15],
            b_vault: &arr[16],
            a_vault_lp: &arr[17],
            b_vault_lp: &arr[18],
            a_vault_lp_mint: &arr[19],
            b_vault_lp_mint: &arr[20],
            amm_program: &arr[21],
            vault_program: &arr[22],
            token_program: &arr[23],
            event_authority: &arr[24],
            program: &arr[25],
        }
    }
}
pub const STAKE_IX_DISCM: [u8; 8] = [206, 176, 202, 18, 200, 209, 179, 108];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeIxArgs {
    pub amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeIxData(pub StakeIxArgs);
impl From<StakeIxArgs> for StakeIxData {
    fn from(args: StakeIxArgs) -> Self {
        Self(args)
    }
}
impl StakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        STAKE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(StakeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn stake_ix_with_program_id(
    program_id: Pubkey,
    keys: StakeKeys,
    args: StakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; STAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: StakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn stake_ix(keys: StakeKeys, args: StakeIxArgs) -> std::io::Result<Instruction> {
    stake_ix_with_program_id(crate::ID, keys, args)
}
pub fn stake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
) -> ProgramResult {
    let keys: StakeKeys = accounts.into();
    let ix = stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn stake_invoke(
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
) -> ProgramResult {
    stake_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn stake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: StakeKeys = accounts.into();
    let ix = stake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn stake_invoke_signed(
    accounts: StakeAccounts<'_, '_>,
    args: StakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    stake_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn stake_verify_account_keys(
    accounts: StakeAccounts<'_, '_>,
    keys: StakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.stake_token_vault.key, keys.stake_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (*accounts.top_staker_list.key, keys.top_staker_list),
        (*accounts.full_balance_list.key, keys.full_balance_list),
        (*accounts.stake_escrow.key, keys.stake_escrow),
        (*accounts.smallest_stake_escrow.key, keys.smallest_stake_escrow),
        (*accounts.user_stake_token.key, keys.user_stake_token),
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn stake_verify_writable_privileges<'me, 'info>(
    accounts: StakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.stake_token_vault,
        accounts.quote_token_vault,
        accounts.top_staker_list,
        accounts.full_balance_list,
        accounts.stake_escrow,
        accounts.smallest_stake_escrow,
        accounts.user_stake_token,
        accounts.pool,
        accounts.lp_mint,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn stake_verify_signer_privileges<'me, 'info>(
    accounts: StakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn stake_verify_account_privileges<'me, 'info>(
    accounts: StakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    stake_verify_writable_privileges(accounts)?;
    stake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub top_staker_list: &'me AccountInfo<'info>,
    pub full_balance_list: &'me AccountInfo<'info>,
    pub stake_escrow: &'me AccountInfo<'info>,
    pub smallest_stake_escrow: &'me AccountInfo<'info>,
    pub user_quote_token: &'me AccountInfo<'info>,
    pub stake_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeKeys {
    pub vault: Pubkey,
    pub top_staker_list: Pubkey,
    pub full_balance_list: Pubkey,
    pub stake_escrow: Pubkey,
    pub smallest_stake_escrow: Pubkey,
    pub user_quote_token: Pubkey,
    pub stake_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub amm_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimFeeAccounts<'_, '_>> for ClaimFeeKeys {
    fn from(accounts: ClaimFeeAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            top_staker_list: *accounts.top_staker_list.key,
            full_balance_list: *accounts.full_balance_list.key,
            stake_escrow: *accounts.stake_escrow.key,
            smallest_stake_escrow: *accounts.smallest_stake_escrow.key,
            user_quote_token: *accounts.user_quote_token.key,
            stake_token_vault: *accounts.stake_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            amm_program: *accounts.amm_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimFeeKeys> for [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.top_staker_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.full_balance_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smallest_stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_quote_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]> for ClaimFeeKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            top_staker_list: pubkeys[1],
            full_balance_list: pubkeys[2],
            stake_escrow: pubkeys[3],
            smallest_stake_escrow: pubkeys[4],
            user_quote_token: pubkeys[5],
            stake_token_vault: pubkeys[6],
            quote_token_vault: pubkeys[7],
            owner: pubkeys[8],
            pool: pubkeys[9],
            lp_mint: pubkeys[10],
            lock_escrow: pubkeys[11],
            escrow_vault: pubkeys[12],
            a_token_vault: pubkeys[13],
            b_token_vault: pubkeys[14],
            a_vault: pubkeys[15],
            b_vault: pubkeys[16],
            a_vault_lp: pubkeys[17],
            b_vault_lp: pubkeys[18],
            a_vault_lp_mint: pubkeys[19],
            b_vault_lp_mint: pubkeys[20],
            amm_program: pubkeys[21],
            vault_program: pubkeys[22],
            token_program: pubkeys[23],
            event_authority: pubkeys[24],
            program: pubkeys[25],
        }
    }
}
impl<'info> From<ClaimFeeAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.top_staker_list.clone(),
            accounts.full_balance_list.clone(),
            accounts.stake_escrow.clone(),
            accounts.smallest_stake_escrow.clone(),
            accounts.user_quote_token.clone(),
            accounts.stake_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.owner.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.amm_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]>
for ClaimFeeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            top_staker_list: &arr[1],
            full_balance_list: &arr[2],
            stake_escrow: &arr[3],
            smallest_stake_escrow: &arr[4],
            user_quote_token: &arr[5],
            stake_token_vault: &arr[6],
            quote_token_vault: &arr[7],
            owner: &arr[8],
            pool: &arr[9],
            lp_mint: &arr[10],
            lock_escrow: &arr[11],
            escrow_vault: &arr[12],
            a_token_vault: &arr[13],
            b_token_vault: &arr[14],
            a_vault: &arr[15],
            b_vault: &arr[16],
            a_vault_lp: &arr[17],
            b_vault_lp: &arr[18],
            a_vault_lp_mint: &arr[19],
            b_vault_lp_mint: &arr[20],
            amm_program: &arr[21],
            vault_program: &arr[22],
            token_program: &arr[23],
            event_authority: &arr[24],
            program: &arr[25],
        }
    }
}
pub const CLAIM_FEE_IX_DISCM: [u8; 8] = [169, 32, 79, 137, 136, 232, 70, 137];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ClaimFeeIxArgs {
    pub max_fee: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeIxData(pub ClaimFeeIxArgs);
impl From<ClaimFeeIxArgs> for ClaimFeeIxData {
    fn from(args: ClaimFeeIxArgs) -> Self {
        Self(args)
    }
}
impl ClaimFeeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_FEE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimFeeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeeKeys,
    args: ClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_IX_ACCOUNTS_LEN] = keys.into();
    let data: ClaimFeeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn claim_fee_ix(
    keys: ClaimFeeKeys,
    args: ClaimFeeIxArgs,
) -> std::io::Result<Instruction> {
    claim_fee_ix_with_program_id(crate::ID, keys, args)
}
pub fn claim_fee_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_invoke(
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
) -> ProgramResult {
    claim_fee_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn claim_fee_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeKeys = accounts.into();
    let ix = claim_fee_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_invoke_signed(
    accounts: ClaimFeeAccounts<'_, '_>,
    args: ClaimFeeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn claim_fee_verify_account_keys(
    accounts: ClaimFeeAccounts<'_, '_>,
    keys: ClaimFeeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.top_staker_list.key, keys.top_staker_list),
        (*accounts.full_balance_list.key, keys.full_balance_list),
        (*accounts.stake_escrow.key, keys.stake_escrow),
        (*accounts.smallest_stake_escrow.key, keys.smallest_stake_escrow),
        (*accounts.user_quote_token.key, keys.user_quote_token),
        (*accounts.stake_token_vault.key, keys.stake_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.top_staker_list,
        accounts.full_balance_list,
        accounts.stake_escrow,
        accounts.smallest_stake_escrow,
        accounts.user_quote_token,
        accounts.stake_token_vault,
        accounts.quote_token_vault,
        accounts.pool,
        accounts.lp_mint,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_signer_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn claim_fee_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee_verify_writable_privileges(accounts)?;
    claim_fee_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const REQUEST_UNSTAKE_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct RequestUnstakeAccounts<'me, 'info> {
    pub unstake: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub top_staker_list: &'me AccountInfo<'info>,
    pub full_balance_list: &'me AccountInfo<'info>,
    pub stake_escrow: &'me AccountInfo<'info>,
    pub stake_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub system_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct RequestUnstakeKeys {
    pub unstake: Pubkey,
    pub vault: Pubkey,
    pub top_staker_list: Pubkey,
    pub full_balance_list: Pubkey,
    pub stake_escrow: Pubkey,
    pub stake_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub amm_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub system_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<RequestUnstakeAccounts<'_, '_>> for RequestUnstakeKeys {
    fn from(accounts: RequestUnstakeAccounts) -> Self {
        Self {
            unstake: *accounts.unstake.key,
            vault: *accounts.vault.key,
            top_staker_list: *accounts.top_staker_list.key,
            full_balance_list: *accounts.full_balance_list.key,
            stake_escrow: *accounts.stake_escrow.key,
            stake_token_vault: *accounts.stake_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            amm_program: *accounts.amm_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            system_program: *accounts.system_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<RequestUnstakeKeys> for [AccountMeta; REQUEST_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: RequestUnstakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.unstake,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.top_staker_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.full_balance_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.system_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; REQUEST_UNSTAKE_IX_ACCOUNTS_LEN]> for RequestUnstakeKeys {
    fn from(pubkeys: [Pubkey; REQUEST_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstake: pubkeys[0],
            vault: pubkeys[1],
            top_staker_list: pubkeys[2],
            full_balance_list: pubkeys[3],
            stake_escrow: pubkeys[4],
            stake_token_vault: pubkeys[5],
            quote_token_vault: pubkeys[6],
            owner: pubkeys[7],
            pool: pubkeys[8],
            lp_mint: pubkeys[9],
            lock_escrow: pubkeys[10],
            escrow_vault: pubkeys[11],
            a_token_vault: pubkeys[12],
            b_token_vault: pubkeys[13],
            a_vault: pubkeys[14],
            b_vault: pubkeys[15],
            a_vault_lp: pubkeys[16],
            b_vault_lp: pubkeys[17],
            a_vault_lp_mint: pubkeys[18],
            b_vault_lp_mint: pubkeys[19],
            amm_program: pubkeys[20],
            vault_program: pubkeys[21],
            token_program: pubkeys[22],
            system_program: pubkeys[23],
            event_authority: pubkeys[24],
            program: pubkeys[25],
        }
    }
}
impl<'info> From<RequestUnstakeAccounts<'_, 'info>>
for [AccountInfo<'info>; REQUEST_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: RequestUnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.unstake.clone(),
            accounts.vault.clone(),
            accounts.top_staker_list.clone(),
            accounts.full_balance_list.clone(),
            accounts.stake_escrow.clone(),
            accounts.stake_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.owner.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.amm_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.system_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; REQUEST_UNSTAKE_IX_ACCOUNTS_LEN]>
for RequestUnstakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; REQUEST_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstake: &arr[0],
            vault: &arr[1],
            top_staker_list: &arr[2],
            full_balance_list: &arr[3],
            stake_escrow: &arr[4],
            stake_token_vault: &arr[5],
            quote_token_vault: &arr[6],
            owner: &arr[7],
            pool: &arr[8],
            lp_mint: &arr[9],
            lock_escrow: &arr[10],
            escrow_vault: &arr[11],
            a_token_vault: &arr[12],
            b_token_vault: &arr[13],
            a_vault: &arr[14],
            b_vault: &arr[15],
            a_vault_lp: &arr[16],
            b_vault_lp: &arr[17],
            a_vault_lp_mint: &arr[18],
            b_vault_lp_mint: &arr[19],
            amm_program: &arr[20],
            vault_program: &arr[21],
            token_program: &arr[22],
            system_program: &arr[23],
            event_authority: &arr[24],
            program: &arr[25],
        }
    }
}
pub const REQUEST_UNSTAKE_IX_DISCM: [u8; 8] = [44, 154, 110, 253, 160, 202, 54, 34];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct RequestUnstakeIxArgs {
    pub unstake_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RequestUnstakeIxData(pub RequestUnstakeIxArgs);
impl From<RequestUnstakeIxArgs> for RequestUnstakeIxData {
    fn from(args: RequestUnstakeIxArgs) -> Self {
        Self(args)
    }
}
impl RequestUnstakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != REQUEST_UNSTAKE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REQUEST_UNSTAKE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RequestUnstakeIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&REQUEST_UNSTAKE_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn request_unstake_ix_with_program_id(
    program_id: Pubkey,
    keys: RequestUnstakeKeys,
    args: RequestUnstakeIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; REQUEST_UNSTAKE_IX_ACCOUNTS_LEN] = keys.into();
    let data: RequestUnstakeIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn request_unstake_ix(
    keys: RequestUnstakeKeys,
    args: RequestUnstakeIxArgs,
) -> std::io::Result<Instruction> {
    request_unstake_ix_with_program_id(crate::ID, keys, args)
}
pub fn request_unstake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: RequestUnstakeAccounts<'_, '_>,
    args: RequestUnstakeIxArgs,
) -> ProgramResult {
    let keys: RequestUnstakeKeys = accounts.into();
    let ix = request_unstake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn request_unstake_invoke(
    accounts: RequestUnstakeAccounts<'_, '_>,
    args: RequestUnstakeIxArgs,
) -> ProgramResult {
    request_unstake_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn request_unstake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: RequestUnstakeAccounts<'_, '_>,
    args: RequestUnstakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: RequestUnstakeKeys = accounts.into();
    let ix = request_unstake_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn request_unstake_invoke_signed(
    accounts: RequestUnstakeAccounts<'_, '_>,
    args: RequestUnstakeIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    request_unstake_invoke_signed_with_program_id(crate::ID, accounts, args, seeds)
}
pub fn request_unstake_verify_account_keys(
    accounts: RequestUnstakeAccounts<'_, '_>,
    keys: RequestUnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.unstake.key, keys.unstake),
        (*accounts.vault.key, keys.vault),
        (*accounts.top_staker_list.key, keys.top_staker_list),
        (*accounts.full_balance_list.key, keys.full_balance_list),
        (*accounts.stake_escrow.key, keys.stake_escrow),
        (*accounts.stake_token_vault.key, keys.stake_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.system_program.key, keys.system_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn request_unstake_verify_writable_privileges<'me, 'info>(
    accounts: RequestUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.unstake,
        accounts.vault,
        accounts.top_staker_list,
        accounts.full_balance_list,
        accounts.stake_escrow,
        accounts.stake_token_vault,
        accounts.quote_token_vault,
        accounts.owner,
        accounts.pool,
        accounts.lp_mint,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn request_unstake_verify_signer_privileges<'me, 'info>(
    accounts: RequestUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.unstake, accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn request_unstake_verify_account_privileges<'me, 'info>(
    accounts: RequestUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    request_unstake_verify_writable_privileges(accounts)?;
    request_unstake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CANCEL_UNSTAKE_IX_ACCOUNTS_LEN: usize = 26;
#[derive(Copy, Clone, Debug)]
pub struct CancelUnstakeAccounts<'me, 'info> {
    pub unstake: &'me AccountInfo<'info>,
    pub stake_escrow: &'me AccountInfo<'info>,
    pub smallest_stake_escrow: &'me AccountInfo<'info>,
    pub top_staker_list: &'me AccountInfo<'info>,
    pub full_balance_list: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub stake_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct CancelUnstakeKeys {
    pub unstake: Pubkey,
    pub stake_escrow: Pubkey,
    pub smallest_stake_escrow: Pubkey,
    pub top_staker_list: Pubkey,
    pub full_balance_list: Pubkey,
    pub vault: Pubkey,
    pub stake_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub owner: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub amm_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<CancelUnstakeAccounts<'_, '_>> for CancelUnstakeKeys {
    fn from(accounts: CancelUnstakeAccounts) -> Self {
        Self {
            unstake: *accounts.unstake.key,
            stake_escrow: *accounts.stake_escrow.key,
            smallest_stake_escrow: *accounts.smallest_stake_escrow.key,
            top_staker_list: *accounts.top_staker_list.key,
            full_balance_list: *accounts.full_balance_list.key,
            vault: *accounts.vault.key,
            stake_token_vault: *accounts.stake_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            owner: *accounts.owner.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            amm_program: *accounts.amm_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<CancelUnstakeKeys> for [AccountMeta; CANCEL_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(keys: CancelUnstakeKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.unstake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.smallest_stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.top_staker_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.full_balance_list,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CANCEL_UNSTAKE_IX_ACCOUNTS_LEN]> for CancelUnstakeKeys {
    fn from(pubkeys: [Pubkey; CANCEL_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstake: pubkeys[0],
            stake_escrow: pubkeys[1],
            smallest_stake_escrow: pubkeys[2],
            top_staker_list: pubkeys[3],
            full_balance_list: pubkeys[4],
            vault: pubkeys[5],
            stake_token_vault: pubkeys[6],
            quote_token_vault: pubkeys[7],
            owner: pubkeys[8],
            pool: pubkeys[9],
            lp_mint: pubkeys[10],
            lock_escrow: pubkeys[11],
            escrow_vault: pubkeys[12],
            a_token_vault: pubkeys[13],
            b_token_vault: pubkeys[14],
            a_vault: pubkeys[15],
            b_vault: pubkeys[16],
            a_vault_lp: pubkeys[17],
            b_vault_lp: pubkeys[18],
            a_vault_lp_mint: pubkeys[19],
            b_vault_lp_mint: pubkeys[20],
            amm_program: pubkeys[21],
            vault_program: pubkeys[22],
            token_program: pubkeys[23],
            event_authority: pubkeys[24],
            program: pubkeys[25],
        }
    }
}
impl<'info> From<CancelUnstakeAccounts<'_, 'info>>
for [AccountInfo<'info>; CANCEL_UNSTAKE_IX_ACCOUNTS_LEN] {
    fn from(accounts: CancelUnstakeAccounts<'_, 'info>) -> Self {
        [
            accounts.unstake.clone(),
            accounts.stake_escrow.clone(),
            accounts.smallest_stake_escrow.clone(),
            accounts.top_staker_list.clone(),
            accounts.full_balance_list.clone(),
            accounts.vault.clone(),
            accounts.stake_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.owner.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.amm_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CANCEL_UNSTAKE_IX_ACCOUNTS_LEN]>
for CancelUnstakeAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CANCEL_UNSTAKE_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstake: &arr[0],
            stake_escrow: &arr[1],
            smallest_stake_escrow: &arr[2],
            top_staker_list: &arr[3],
            full_balance_list: &arr[4],
            vault: &arr[5],
            stake_token_vault: &arr[6],
            quote_token_vault: &arr[7],
            owner: &arr[8],
            pool: &arr[9],
            lp_mint: &arr[10],
            lock_escrow: &arr[11],
            escrow_vault: &arr[12],
            a_token_vault: &arr[13],
            b_token_vault: &arr[14],
            a_vault: &arr[15],
            b_vault: &arr[16],
            a_vault_lp: &arr[17],
            b_vault_lp: &arr[18],
            a_vault_lp_mint: &arr[19],
            b_vault_lp_mint: &arr[20],
            amm_program: &arr[21],
            vault_program: &arr[22],
            token_program: &arr[23],
            event_authority: &arr[24],
            program: &arr[25],
        }
    }
}
pub const CANCEL_UNSTAKE_IX_DISCM: [u8; 8] = [64, 65, 53, 227, 125, 153, 3, 167];
#[derive(Clone, Debug, PartialEq)]
pub struct CancelUnstakeIxData;
impl CancelUnstakeIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CANCEL_UNSTAKE_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CANCEL_UNSTAKE_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CANCEL_UNSTAKE_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn cancel_unstake_ix_with_program_id(
    program_id: Pubkey,
    keys: CancelUnstakeKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CANCEL_UNSTAKE_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: CancelUnstakeIxData.try_to_vec()?,
    })
}
pub fn cancel_unstake_ix(keys: CancelUnstakeKeys) -> std::io::Result<Instruction> {
    cancel_unstake_ix_with_program_id(crate::ID, keys)
}
pub fn cancel_unstake_invoke_with_program_id(
    program_id: Pubkey,
    accounts: CancelUnstakeAccounts<'_, '_>,
) -> ProgramResult {
    let keys: CancelUnstakeKeys = accounts.into();
    let ix = cancel_unstake_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn cancel_unstake_invoke(accounts: CancelUnstakeAccounts<'_, '_>) -> ProgramResult {
    cancel_unstake_invoke_with_program_id(crate::ID, accounts)
}
pub fn cancel_unstake_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: CancelUnstakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: CancelUnstakeKeys = accounts.into();
    let ix = cancel_unstake_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn cancel_unstake_invoke_signed(
    accounts: CancelUnstakeAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    cancel_unstake_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn cancel_unstake_verify_account_keys(
    accounts: CancelUnstakeAccounts<'_, '_>,
    keys: CancelUnstakeKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.unstake.key, keys.unstake),
        (*accounts.stake_escrow.key, keys.stake_escrow),
        (*accounts.smallest_stake_escrow.key, keys.smallest_stake_escrow),
        (*accounts.top_staker_list.key, keys.top_staker_list),
        (*accounts.full_balance_list.key, keys.full_balance_list),
        (*accounts.vault.key, keys.vault),
        (*accounts.stake_token_vault.key, keys.stake_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (*accounts.owner.key, keys.owner),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn cancel_unstake_verify_writable_privileges<'me, 'info>(
    accounts: CancelUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.unstake,
        accounts.stake_escrow,
        accounts.smallest_stake_escrow,
        accounts.top_staker_list,
        accounts.full_balance_list,
        accounts.vault,
        accounts.stake_token_vault,
        accounts.quote_token_vault,
        accounts.owner,
        accounts.pool,
        accounts.lp_mint,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn cancel_unstake_verify_signer_privileges<'me, 'info>(
    accounts: CancelUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn cancel_unstake_verify_account_privileges<'me, 'info>(
    accounts: CancelUnstakeAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    cancel_unstake_verify_writable_privileges(accounts)?;
    cancel_unstake_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const WITHDRAW_IX_ACCOUNTS_LEN: usize = 9;
#[derive(Copy, Clone, Debug)]
pub struct WithdrawAccounts<'me, 'info> {
    pub unstake: &'me AccountInfo<'info>,
    pub stake_escrow: &'me AccountInfo<'info>,
    pub stake_token_vault: &'me AccountInfo<'info>,
    pub vault: &'me AccountInfo<'info>,
    pub user_stake_token: &'me AccountInfo<'info>,
    pub owner: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct WithdrawKeys {
    pub unstake: Pubkey,
    pub stake_escrow: Pubkey,
    pub stake_token_vault: Pubkey,
    pub vault: Pubkey,
    pub user_stake_token: Pubkey,
    pub owner: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<WithdrawAccounts<'_, '_>> for WithdrawKeys {
    fn from(accounts: WithdrawAccounts) -> Self {
        Self {
            unstake: *accounts.unstake.key,
            stake_escrow: *accounts.stake_escrow.key,
            stake_token_vault: *accounts.stake_token_vault.key,
            vault: *accounts.vault.key,
            user_stake_token: *accounts.user_stake_token.key,
            owner: *accounts.owner.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<WithdrawKeys> for [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(keys: WithdrawKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.unstake,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.user_stake_token,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.owner,
                is_signer: true,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]> for WithdrawKeys {
    fn from(pubkeys: [Pubkey; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstake: pubkeys[0],
            stake_escrow: pubkeys[1],
            stake_token_vault: pubkeys[2],
            vault: pubkeys[3],
            user_stake_token: pubkeys[4],
            owner: pubkeys[5],
            token_program: pubkeys[6],
            event_authority: pubkeys[7],
            program: pubkeys[8],
        }
    }
}
impl<'info> From<WithdrawAccounts<'_, 'info>>
for [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN] {
    fn from(accounts: WithdrawAccounts<'_, 'info>) -> Self {
        [
            accounts.unstake.clone(),
            accounts.stake_escrow.clone(),
            accounts.stake_token_vault.clone(),
            accounts.vault.clone(),
            accounts.user_stake_token.clone(),
            accounts.owner.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]>
for WithdrawAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; WITHDRAW_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            unstake: &arr[0],
            stake_escrow: &arr[1],
            stake_token_vault: &arr[2],
            vault: &arr[3],
            user_stake_token: &arr[4],
            owner: &arr[5],
            token_program: &arr[6],
            event_authority: &arr[7],
            program: &arr[8],
        }
    }
}
pub const WITHDRAW_IX_DISCM: [u8; 8] = [183, 18, 70, 156, 148, 109, 161, 34];
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawIxData;
impl WithdrawIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != WITHDRAW_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&WITHDRAW_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn withdraw_ix_with_program_id(
    program_id: Pubkey,
    keys: WithdrawKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; WITHDRAW_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: WithdrawIxData.try_to_vec()?,
    })
}
pub fn withdraw_ix(keys: WithdrawKeys) -> std::io::Result<Instruction> {
    withdraw_ix_with_program_id(crate::ID, keys)
}
pub fn withdraw_invoke_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn withdraw_invoke(accounts: WithdrawAccounts<'_, '_>) -> ProgramResult {
    withdraw_invoke_with_program_id(crate::ID, accounts)
}
pub fn withdraw_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: WithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: WithdrawKeys = accounts.into();
    let ix = withdraw_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn withdraw_invoke_signed(
    accounts: WithdrawAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    withdraw_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn withdraw_verify_account_keys(
    accounts: WithdrawAccounts<'_, '_>,
    keys: WithdrawKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.unstake.key, keys.unstake),
        (*accounts.stake_escrow.key, keys.stake_escrow),
        (*accounts.stake_token_vault.key, keys.stake_token_vault),
        (*accounts.vault.key, keys.vault),
        (*accounts.user_stake_token.key, keys.user_stake_token),
        (*accounts.owner.key, keys.owner),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn withdraw_verify_writable_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.unstake,
        accounts.stake_escrow,
        accounts.stake_token_vault,
        accounts.vault,
        accounts.user_stake_token,
        accounts.owner,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn withdraw_verify_signer_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.owner] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn withdraw_verify_account_privileges<'me, 'info>(
    accounts: WithdrawAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    withdraw_verify_writable_privileges(accounts)?;
    withdraw_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN: usize = 20;
#[derive(Copy, Clone, Debug)]
pub struct ClaimFeeCrankAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub stake_token_vault: &'me AccountInfo<'info>,
    pub quote_token_vault: &'me AccountInfo<'info>,
    pub pool: &'me AccountInfo<'info>,
    pub lp_mint: &'me AccountInfo<'info>,
    pub lock_escrow: &'me AccountInfo<'info>,
    pub escrow_vault: &'me AccountInfo<'info>,
    pub a_token_vault: &'me AccountInfo<'info>,
    pub b_token_vault: &'me AccountInfo<'info>,
    pub a_vault: &'me AccountInfo<'info>,
    pub b_vault: &'me AccountInfo<'info>,
    pub a_vault_lp: &'me AccountInfo<'info>,
    pub b_vault_lp: &'me AccountInfo<'info>,
    pub a_vault_lp_mint: &'me AccountInfo<'info>,
    pub b_vault_lp_mint: &'me AccountInfo<'info>,
    pub amm_program: &'me AccountInfo<'info>,
    pub vault_program: &'me AccountInfo<'info>,
    pub token_program: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ClaimFeeCrankKeys {
    pub vault: Pubkey,
    pub stake_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub pool: Pubkey,
    pub lp_mint: Pubkey,
    pub lock_escrow: Pubkey,
    pub escrow_vault: Pubkey,
    pub a_token_vault: Pubkey,
    pub b_token_vault: Pubkey,
    pub a_vault: Pubkey,
    pub b_vault: Pubkey,
    pub a_vault_lp: Pubkey,
    pub b_vault_lp: Pubkey,
    pub a_vault_lp_mint: Pubkey,
    pub b_vault_lp_mint: Pubkey,
    pub amm_program: Pubkey,
    pub vault_program: Pubkey,
    pub token_program: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<ClaimFeeCrankAccounts<'_, '_>> for ClaimFeeCrankKeys {
    fn from(accounts: ClaimFeeCrankAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            stake_token_vault: *accounts.stake_token_vault.key,
            quote_token_vault: *accounts.quote_token_vault.key,
            pool: *accounts.pool.key,
            lp_mint: *accounts.lp_mint.key,
            lock_escrow: *accounts.lock_escrow.key,
            escrow_vault: *accounts.escrow_vault.key,
            a_token_vault: *accounts.a_token_vault.key,
            b_token_vault: *accounts.b_token_vault.key,
            a_vault: *accounts.a_vault.key,
            b_vault: *accounts.b_vault.key,
            a_vault_lp: *accounts.a_vault_lp.key,
            b_vault_lp: *accounts.b_vault_lp.key,
            a_vault_lp_mint: *accounts.a_vault_lp_mint.key,
            b_vault_lp_mint: *accounts.b_vault_lp_mint.key,
            amm_program: *accounts.amm_program.key,
            vault_program: *accounts.vault_program.key,
            token_program: *accounts.token_program.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<ClaimFeeCrankKeys> for [AccountMeta; CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN] {
    fn from(keys: ClaimFeeCrankKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.stake_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.quote_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.pool,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.lock_escrow,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.escrow_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_token_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.a_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.b_vault_lp_mint,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.amm_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.vault_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.token_program,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN]> for ClaimFeeCrankKeys {
    fn from(pubkeys: [Pubkey; CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            stake_token_vault: pubkeys[1],
            quote_token_vault: pubkeys[2],
            pool: pubkeys[3],
            lp_mint: pubkeys[4],
            lock_escrow: pubkeys[5],
            escrow_vault: pubkeys[6],
            a_token_vault: pubkeys[7],
            b_token_vault: pubkeys[8],
            a_vault: pubkeys[9],
            b_vault: pubkeys[10],
            a_vault_lp: pubkeys[11],
            b_vault_lp: pubkeys[12],
            a_vault_lp_mint: pubkeys[13],
            b_vault_lp_mint: pubkeys[14],
            amm_program: pubkeys[15],
            vault_program: pubkeys[16],
            token_program: pubkeys[17],
            event_authority: pubkeys[18],
            program: pubkeys[19],
        }
    }
}
impl<'info> From<ClaimFeeCrankAccounts<'_, 'info>>
for [AccountInfo<'info>; CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN] {
    fn from(accounts: ClaimFeeCrankAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.stake_token_vault.clone(),
            accounts.quote_token_vault.clone(),
            accounts.pool.clone(),
            accounts.lp_mint.clone(),
            accounts.lock_escrow.clone(),
            accounts.escrow_vault.clone(),
            accounts.a_token_vault.clone(),
            accounts.b_token_vault.clone(),
            accounts.a_vault.clone(),
            accounts.b_vault.clone(),
            accounts.a_vault_lp.clone(),
            accounts.b_vault_lp.clone(),
            accounts.a_vault_lp_mint.clone(),
            accounts.b_vault_lp_mint.clone(),
            accounts.amm_program.clone(),
            accounts.vault_program.clone(),
            accounts.token_program.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN]>
for ClaimFeeCrankAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: &arr[0],
            stake_token_vault: &arr[1],
            quote_token_vault: &arr[2],
            pool: &arr[3],
            lp_mint: &arr[4],
            lock_escrow: &arr[5],
            escrow_vault: &arr[6],
            a_token_vault: &arr[7],
            b_token_vault: &arr[8],
            a_vault: &arr[9],
            b_vault: &arr[10],
            a_vault_lp: &arr[11],
            b_vault_lp: &arr[12],
            a_vault_lp_mint: &arr[13],
            b_vault_lp_mint: &arr[14],
            amm_program: &arr[15],
            vault_program: &arr[16],
            token_program: &arr[17],
            event_authority: &arr[18],
            program: &arr[19],
        }
    }
}
pub const CLAIM_FEE_CRANK_IX_DISCM: [u8; 8] = [202, 199, 147, 2, 255, 199, 1, 222];
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeCrankIxData;
impl ClaimFeeCrankIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != CLAIM_FEE_CRANK_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_FEE_CRANK_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&CLAIM_FEE_CRANK_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn claim_fee_crank_ix_with_program_id(
    program_id: Pubkey,
    keys: ClaimFeeCrankKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; CLAIM_FEE_CRANK_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: ClaimFeeCrankIxData.try_to_vec()?,
    })
}
pub fn claim_fee_crank_ix(keys: ClaimFeeCrankKeys) -> std::io::Result<Instruction> {
    claim_fee_crank_ix_with_program_id(crate::ID, keys)
}
pub fn claim_fee_crank_invoke_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeCrankAccounts<'_, '_>,
) -> ProgramResult {
    let keys: ClaimFeeCrankKeys = accounts.into();
    let ix = claim_fee_crank_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn claim_fee_crank_invoke(accounts: ClaimFeeCrankAccounts<'_, '_>) -> ProgramResult {
    claim_fee_crank_invoke_with_program_id(crate::ID, accounts)
}
pub fn claim_fee_crank_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: ClaimFeeCrankAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: ClaimFeeCrankKeys = accounts.into();
    let ix = claim_fee_crank_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn claim_fee_crank_invoke_signed(
    accounts: ClaimFeeCrankAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    claim_fee_crank_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn claim_fee_crank_verify_account_keys(
    accounts: ClaimFeeCrankAccounts<'_, '_>,
    keys: ClaimFeeCrankKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.stake_token_vault.key, keys.stake_token_vault),
        (*accounts.quote_token_vault.key, keys.quote_token_vault),
        (*accounts.pool.key, keys.pool),
        (*accounts.lp_mint.key, keys.lp_mint),
        (*accounts.lock_escrow.key, keys.lock_escrow),
        (*accounts.escrow_vault.key, keys.escrow_vault),
        (*accounts.a_token_vault.key, keys.a_token_vault),
        (*accounts.b_token_vault.key, keys.b_token_vault),
        (*accounts.a_vault.key, keys.a_vault),
        (*accounts.b_vault.key, keys.b_vault),
        (*accounts.a_vault_lp.key, keys.a_vault_lp),
        (*accounts.b_vault_lp.key, keys.b_vault_lp),
        (*accounts.a_vault_lp_mint.key, keys.a_vault_lp_mint),
        (*accounts.b_vault_lp_mint.key, keys.b_vault_lp_mint),
        (*accounts.amm_program.key, keys.amm_program),
        (*accounts.vault_program.key, keys.vault_program),
        (*accounts.token_program.key, keys.token_program),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn claim_fee_crank_verify_writable_privileges<'me, 'info>(
    accounts: ClaimFeeCrankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [
        accounts.vault,
        accounts.stake_token_vault,
        accounts.quote_token_vault,
        accounts.pool,
        accounts.lp_mint,
        accounts.lock_escrow,
        accounts.escrow_vault,
        accounts.a_token_vault,
        accounts.b_token_vault,
        accounts.a_vault,
        accounts.b_vault,
        accounts.a_vault_lp,
        accounts.b_vault_lp,
        accounts.a_vault_lp_mint,
        accounts.b_vault_lp_mint,
    ] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn claim_fee_crank_verify_account_privileges<'me, 'info>(
    accounts: ClaimFeeCrankAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    claim_fee_crank_verify_writable_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN: usize = 4;
#[derive(Copy, Clone, Debug)]
pub struct UpdateUnstakeLockDurationAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateUnstakeLockDurationKeys {
    pub vault: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateUnstakeLockDurationAccounts<'_, '_>> for UpdateUnstakeLockDurationKeys {
    fn from(accounts: UpdateUnstakeLockDurationAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateUnstakeLockDurationKeys>
for [AccountMeta; UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateUnstakeLockDurationKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN]>
for UpdateUnstakeLockDurationKeys {
    fn from(pubkeys: [Pubkey; UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            admin: pubkeys[1],
            event_authority: pubkeys[2],
            program: pubkeys[3],
        }
    }
}
impl<'info> From<UpdateUnstakeLockDurationAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateUnstakeLockDurationAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN]>
for UpdateUnstakeLockDurationAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            admin: &arr[1],
            event_authority: &arr[2],
            program: &arr[3],
        }
    }
}
pub const UPDATE_UNSTAKE_LOCK_DURATION_IX_DISCM: [u8; 8] = [
    83,
    195,
    196,
    224,
    200,
    81,
    70,
    96,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateUnstakeLockDurationIxArgs {
    pub unstake_lock_duration: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUnstakeLockDurationIxData(pub UpdateUnstakeLockDurationIxArgs);
impl From<UpdateUnstakeLockDurationIxArgs> for UpdateUnstakeLockDurationIxData {
    fn from(args: UpdateUnstakeLockDurationIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateUnstakeLockDurationIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_UNSTAKE_LOCK_DURATION_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_UNSTAKE_LOCK_DURATION_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateUnstakeLockDurationIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_UNSTAKE_LOCK_DURATION_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_unstake_lock_duration_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateUnstakeLockDurationKeys,
    args: UpdateUnstakeLockDurationIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_UNSTAKE_LOCK_DURATION_IX_ACCOUNTS_LEN] = keys.into();
    let data: UpdateUnstakeLockDurationIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_unstake_lock_duration_ix(
    keys: UpdateUnstakeLockDurationKeys,
    args: UpdateUnstakeLockDurationIxArgs,
) -> std::io::Result<Instruction> {
    update_unstake_lock_duration_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_unstake_lock_duration_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUnstakeLockDurationAccounts<'_, '_>,
    args: UpdateUnstakeLockDurationIxArgs,
) -> ProgramResult {
    let keys: UpdateUnstakeLockDurationKeys = accounts.into();
    let ix = update_unstake_lock_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_unstake_lock_duration_invoke(
    accounts: UpdateUnstakeLockDurationAccounts<'_, '_>,
    args: UpdateUnstakeLockDurationIxArgs,
) -> ProgramResult {
    update_unstake_lock_duration_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_unstake_lock_duration_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateUnstakeLockDurationAccounts<'_, '_>,
    args: UpdateUnstakeLockDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateUnstakeLockDurationKeys = accounts.into();
    let ix = update_unstake_lock_duration_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_unstake_lock_duration_invoke_signed(
    accounts: UpdateUnstakeLockDurationAccounts<'_, '_>,
    args: UpdateUnstakeLockDurationIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_unstake_lock_duration_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_unstake_lock_duration_verify_account_keys(
    accounts: UpdateUnstakeLockDurationAccounts<'_, '_>,
    keys: UpdateUnstakeLockDurationKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_unstake_lock_duration_verify_writable_privileges<'me, 'info>(
    accounts: UpdateUnstakeLockDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_unstake_lock_duration_verify_signer_privileges<'me, 'info>(
    accounts: UpdateUnstakeLockDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_unstake_lock_duration_verify_account_privileges<'me, 'info>(
    accounts: UpdateUnstakeLockDurationAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_unstake_lock_duration_verify_writable_privileges(accounts)?;
    update_unstake_lock_duration_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN: usize = 5;
#[derive(Copy, Clone, Debug)]
pub struct UpdateSecondsToFullUnlockAccounts<'me, 'info> {
    pub vault: &'me AccountInfo<'info>,
    pub instructions_sysvar: &'me AccountInfo<'info>,
    pub admin: &'me AccountInfo<'info>,
    pub event_authority: &'me AccountInfo<'info>,
    pub program: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct UpdateSecondsToFullUnlockKeys {
    pub vault: Pubkey,
    pub instructions_sysvar: Pubkey,
    pub admin: Pubkey,
    pub event_authority: Pubkey,
    pub program: Pubkey,
}
impl From<UpdateSecondsToFullUnlockAccounts<'_, '_>> for UpdateSecondsToFullUnlockKeys {
    fn from(accounts: UpdateSecondsToFullUnlockAccounts) -> Self {
        Self {
            vault: *accounts.vault.key,
            instructions_sysvar: *accounts.instructions_sysvar.key,
            admin: *accounts.admin.key,
            event_authority: *accounts.event_authority.key,
            program: *accounts.program.key,
        }
    }
}
impl From<UpdateSecondsToFullUnlockKeys>
for [AccountMeta; UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN] {
    fn from(keys: UpdateSecondsToFullUnlockKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.vault,
                is_signer: false,
                is_writable: true,
            },
            AccountMeta {
                pubkey: keys.instructions_sysvar,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.admin,
                is_signer: true,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.event_authority,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.program,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN]>
for UpdateSecondsToFullUnlockKeys {
    fn from(pubkeys: [Pubkey; UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            vault: pubkeys[0],
            instructions_sysvar: pubkeys[1],
            admin: pubkeys[2],
            event_authority: pubkeys[3],
            program: pubkeys[4],
        }
    }
}
impl<'info> From<UpdateSecondsToFullUnlockAccounts<'_, 'info>>
for [AccountInfo<'info>; UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN] {
    fn from(accounts: UpdateSecondsToFullUnlockAccounts<'_, 'info>) -> Self {
        [
            accounts.vault.clone(),
            accounts.instructions_sysvar.clone(),
            accounts.admin.clone(),
            accounts.event_authority.clone(),
            accounts.program.clone(),
        ]
    }
}
impl<
    'me,
    'info,
> From<&'me [AccountInfo<'info>; UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN]>
for UpdateSecondsToFullUnlockAccounts<'me, 'info> {
    fn from(
        arr: &'me [AccountInfo<'info>; UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN],
    ) -> Self {
        Self {
            vault: &arr[0],
            instructions_sysvar: &arr[1],
            admin: &arr[2],
            event_authority: &arr[3],
            program: &arr[4],
        }
    }
}
pub const UPDATE_SECONDS_TO_FULL_UNLOCK_IX_DISCM: [u8; 8] = [
    22,
    2,
    54,
    36,
    74,
    146,
    7,
    141,
];
#[derive(BorshDeserialize, BorshSerialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct UpdateSecondsToFullUnlockIxArgs {
    pub seconds_to_full_unlock: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSecondsToFullUnlockIxData(pub UpdateSecondsToFullUnlockIxArgs);
impl From<UpdateSecondsToFullUnlockIxArgs> for UpdateSecondsToFullUnlockIxData {
    fn from(args: UpdateSecondsToFullUnlockIxArgs) -> Self {
        Self(args)
    }
}
impl UpdateSecondsToFullUnlockIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UPDATE_SECONDS_TO_FULL_UNLOCK_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_SECONDS_TO_FULL_UNLOCK_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateSecondsToFullUnlockIxArgs::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UPDATE_SECONDS_TO_FULL_UNLOCK_IX_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn update_seconds_to_full_unlock_ix_with_program_id(
    program_id: Pubkey,
    keys: UpdateSecondsToFullUnlockKeys,
    args: UpdateSecondsToFullUnlockIxArgs,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; UPDATE_SECONDS_TO_FULL_UNLOCK_IX_ACCOUNTS_LEN] = keys
        .into();
    let data: UpdateSecondsToFullUnlockIxData = args.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: data.try_to_vec()?,
    })
}
pub fn update_seconds_to_full_unlock_ix(
    keys: UpdateSecondsToFullUnlockKeys,
    args: UpdateSecondsToFullUnlockIxArgs,
) -> std::io::Result<Instruction> {
    update_seconds_to_full_unlock_ix_with_program_id(crate::ID, keys, args)
}
pub fn update_seconds_to_full_unlock_invoke_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSecondsToFullUnlockAccounts<'_, '_>,
    args: UpdateSecondsToFullUnlockIxArgs,
) -> ProgramResult {
    let keys: UpdateSecondsToFullUnlockKeys = accounts.into();
    let ix = update_seconds_to_full_unlock_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction(&ix, accounts)
}
pub fn update_seconds_to_full_unlock_invoke(
    accounts: UpdateSecondsToFullUnlockAccounts<'_, '_>,
    args: UpdateSecondsToFullUnlockIxArgs,
) -> ProgramResult {
    update_seconds_to_full_unlock_invoke_with_program_id(crate::ID, accounts, args)
}
pub fn update_seconds_to_full_unlock_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: UpdateSecondsToFullUnlockAccounts<'_, '_>,
    args: UpdateSecondsToFullUnlockIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: UpdateSecondsToFullUnlockKeys = accounts.into();
    let ix = update_seconds_to_full_unlock_ix_with_program_id(program_id, keys, args)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn update_seconds_to_full_unlock_invoke_signed(
    accounts: UpdateSecondsToFullUnlockAccounts<'_, '_>,
    args: UpdateSecondsToFullUnlockIxArgs,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    update_seconds_to_full_unlock_invoke_signed_with_program_id(
        crate::ID,
        accounts,
        args,
        seeds,
    )
}
pub fn update_seconds_to_full_unlock_verify_account_keys(
    accounts: UpdateSecondsToFullUnlockAccounts<'_, '_>,
    keys: UpdateSecondsToFullUnlockKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.vault.key, keys.vault),
        (*accounts.instructions_sysvar.key, keys.instructions_sysvar),
        (*accounts.admin.key, keys.admin),
        (*accounts.event_authority.key, keys.event_authority),
        (*accounts.program.key, keys.program),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
pub fn update_seconds_to_full_unlock_verify_writable_privileges<'me, 'info>(
    accounts: UpdateSecondsToFullUnlockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_writable in [accounts.vault] {
        if !should_be_writable.is_writable {
            return Err((should_be_writable, ProgramError::InvalidAccountData));
        }
    }
    Ok(())
}
pub fn update_seconds_to_full_unlock_verify_signer_privileges<'me, 'info>(
    accounts: UpdateSecondsToFullUnlockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    for should_be_signer in [accounts.admin] {
        if !should_be_signer.is_signer {
            return Err((should_be_signer, ProgramError::MissingRequiredSignature));
        }
    }
    Ok(())
}
pub fn update_seconds_to_full_unlock_verify_account_privileges<'me, 'info>(
    accounts: UpdateSecondsToFullUnlockAccounts<'me, 'info>,
) -> Result<(), (&'me AccountInfo<'info>, ProgramError)> {
    update_seconds_to_full_unlock_verify_writable_privileges(accounts)?;
    update_seconds_to_full_unlock_verify_signer_privileges(accounts)?;
    Ok(())
}
pub const DUMMY_IX_ACCOUNTS_LEN: usize = 2;
#[derive(Copy, Clone, Debug)]
pub struct DummyAccounts<'me, 'info> {
    pub staker_metadata: &'me AccountInfo<'info>,
    pub staker_balance: &'me AccountInfo<'info>,
}
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct DummyKeys {
    pub staker_metadata: Pubkey,
    pub staker_balance: Pubkey,
}
impl From<DummyAccounts<'_, '_>> for DummyKeys {
    fn from(accounts: DummyAccounts) -> Self {
        Self {
            staker_metadata: *accounts.staker_metadata.key,
            staker_balance: *accounts.staker_balance.key,
        }
    }
}
impl From<DummyKeys> for [AccountMeta; DUMMY_IX_ACCOUNTS_LEN] {
    fn from(keys: DummyKeys) -> Self {
        [
            AccountMeta {
                pubkey: keys.staker_metadata,
                is_signer: false,
                is_writable: false,
            },
            AccountMeta {
                pubkey: keys.staker_balance,
                is_signer: false,
                is_writable: false,
            },
        ]
    }
}
impl From<[Pubkey; DUMMY_IX_ACCOUNTS_LEN]> for DummyKeys {
    fn from(pubkeys: [Pubkey; DUMMY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            staker_metadata: pubkeys[0],
            staker_balance: pubkeys[1],
        }
    }
}
impl<'info> From<DummyAccounts<'_, 'info>>
for [AccountInfo<'info>; DUMMY_IX_ACCOUNTS_LEN] {
    fn from(accounts: DummyAccounts<'_, 'info>) -> Self {
        [accounts.staker_metadata.clone(), accounts.staker_balance.clone()]
    }
}
impl<'me, 'info> From<&'me [AccountInfo<'info>; DUMMY_IX_ACCOUNTS_LEN]>
for DummyAccounts<'me, 'info> {
    fn from(arr: &'me [AccountInfo<'info>; DUMMY_IX_ACCOUNTS_LEN]) -> Self {
        Self {
            staker_metadata: &arr[0],
            staker_balance: &arr[1],
        }
    }
}
pub const DUMMY_IX_DISCM: [u8; 8] = [167, 117, 211, 79, 251, 254, 47, 135];
#[derive(Clone, Debug, PartialEq)]
pub struct DummyIxData;
impl DummyIxData {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != DUMMY_IX_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        DUMMY_IX_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self)
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&DUMMY_IX_DISCM)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub fn dummy_ix_with_program_id(
    program_id: Pubkey,
    keys: DummyKeys,
) -> std::io::Result<Instruction> {
    let metas: [AccountMeta; DUMMY_IX_ACCOUNTS_LEN] = keys.into();
    Ok(Instruction {
        program_id,
        accounts: Vec::from(metas),
        data: DummyIxData.try_to_vec()?,
    })
}
pub fn dummy_ix(keys: DummyKeys) -> std::io::Result<Instruction> {
    dummy_ix_with_program_id(crate::ID, keys)
}
pub fn dummy_invoke_with_program_id(
    program_id: Pubkey,
    accounts: DummyAccounts<'_, '_>,
) -> ProgramResult {
    let keys: DummyKeys = accounts.into();
    let ix = dummy_ix_with_program_id(program_id, keys)?;
    invoke_instruction(&ix, accounts)
}
pub fn dummy_invoke(accounts: DummyAccounts<'_, '_>) -> ProgramResult {
    dummy_invoke_with_program_id(crate::ID, accounts)
}
pub fn dummy_invoke_signed_with_program_id(
    program_id: Pubkey,
    accounts: DummyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    let keys: DummyKeys = accounts.into();
    let ix = dummy_ix_with_program_id(program_id, keys)?;
    invoke_instruction_signed(&ix, accounts, seeds)
}
pub fn dummy_invoke_signed(
    accounts: DummyAccounts<'_, '_>,
    seeds: &[&[&[u8]]],
) -> ProgramResult {
    dummy_invoke_signed_with_program_id(crate::ID, accounts, seeds)
}
pub fn dummy_verify_account_keys(
    accounts: DummyAccounts<'_, '_>,
    keys: DummyKeys,
) -> Result<(), (Pubkey, Pubkey)> {
    for (actual, expected) in [
        (*accounts.staker_metadata.key, keys.staker_metadata),
        (*accounts.staker_balance.key, keys.staker_balance),
    ] {
        if actual != expected {
            return Err((actual, expected));
        }
    }
    Ok(())
}
