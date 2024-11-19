use borsh::{BorshDeserialize, BorshSerialize};
use crate::*;
use solana_program::pubkey::Pubkey;
pub const STAKER_METADATA_DUMMY_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [
    164,
    41,
    91,
    148,
    225,
    107,
    235,
    174,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakerMetadataDummyAccount {
    pub staker_metadata: StakerMetadata,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakerMetadataDummyAccountAccount(pub StakerMetadataDummyAccount);
impl StakerMetadataDummyAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKER_METADATA_DUMMY_ACCOUNT_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        STAKER_METADATA_DUMMY_ACCOUNT_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(StakerMetadataDummyAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKER_METADATA_DUMMY_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAKER_BALANCE_DUMMY_ACCOUNT_ACCOUNT_DISCM: [u8; 8] = [
    27,
    66,
    207,
    61,
    128,
    77,
    48,
    26,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakerBalanceDummyAccount {
    pub staker_balance: StakerBalance,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakerBalanceDummyAccountAccount(pub StakerBalanceDummyAccount);
impl StakerBalanceDummyAccountAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKER_BALANCE_DUMMY_ACCOUNT_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        STAKER_BALANCE_DUMMY_ACCOUNT_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(StakerBalanceDummyAccount::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKER_BALANCE_DUMMY_ACCOUNT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FULL_BALANCE_LIST_METADATA_ACCOUNT_DISCM: [u8; 8] = [
    169,
    43,
    13,
    23,
    89,
    89,
    173,
    27,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FullBalanceListMetadata {
    pub vault: Pubkey,
    pub length: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FullBalanceListMetadataAccount(pub FullBalanceListMetadata);
impl FullBalanceListMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FULL_BALANCE_LIST_METADATA_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FULL_BALANCE_LIST_METADATA_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FullBalanceListMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FULL_BALANCE_LIST_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const STAKE_ESCROW_ACCOUNT_DISCM: [u8; 8] = [115, 173, 53, 77, 43, 219, 85, 124];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct StakeEscrow {
    pub owner: Pubkey,
    pub vault: Pubkey,
    pub full_balance_index: u64,
    pub stake_amount: u64,
    pub in_top_list: u8,
    pub padding0: [u8; 15],
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
#[derive(Clone, Debug, PartialEq)]
pub struct StakeEscrowAccount(pub StakeEscrow);
impl StakeEscrowAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != STAKE_ESCROW_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        STAKE_ESCROW_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(StakeEscrow::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&STAKE_ESCROW_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const TOP_LIST_METADATA_ACCOUNT_DISCM: [u8; 8] = [
    129,
    203,
    39,
    239,
    234,
    197,
    215,
    74,
];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TopListMetadata {
    pub vault: Pubkey,
}
#[derive(Clone, Debug, PartialEq)]
pub struct TopListMetadataAccount(pub TopListMetadata);
impl TopListMetadataAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != TOP_LIST_METADATA_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        TOP_LIST_METADATA_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(TopListMetadata::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&TOP_LIST_METADATA_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const UNSTAKE_ACCOUNT_DISCM: [u8; 8] = [154, 148, 131, 67, 52, 244, 244, 19];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Unstake {
    pub stake_escrow: Pubkey,
    pub unstake_amount: u64,
    pub created_at: i64,
    pub release_at: i64,
    pub padding: [u64; 30],
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnstakeAccount(pub Unstake);
impl UnstakeAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != UNSTAKE_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UNSTAKE_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Unstake::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&UNSTAKE_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
pub const FEE_VAULT_ACCOUNT_DISCM: [u8; 8] = [192, 178, 69, 232, 58, 149, 157, 132];
#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeeVault {
    pub lock_escrow: Pubkey,
    pub stake_mint: Pubkey,
    pub quote_mint: Pubkey,
    pub pool: Pubkey,
    pub stake_token_vault: Pubkey,
    pub quote_token_vault: Pubkey,
    pub top_staker_list: Pubkey,
    pub full_balance_list: Pubkey,
    pub metrics: Metrics,
    pub configuration: Configuration,
    pub top_staker_info: TopStakerInfo,
    pub creator: Pubkey,
    pub created_at: i64,
    pub bump: u8,
    pub padding0: [u8; 7],
    pub padding: [u128; 20],
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeVaultAccount(pub FeeVault);
impl FeeVaultAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != FEE_VAULT_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FEE_VAULT_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FeeVault::deserialize(&mut reader)?))
    }
    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&FEE_VAULT_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }
    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}
