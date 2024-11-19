use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const VAULT_CREATED_EVENT_DISCM: [u8; 8] = [117, 25, 120, 254, 75, 236, 78, 115];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct VaultCreated {
    pool: Pubkey,
    token_a_mint: Pubkey,
    token_b_mint: Pubkey,
    vault: Pubkey,
    stake_mint: Pubkey,
    quote_mint: Pubkey,
    creator: Pubkey,
    top_list_length: u16,
    seconds_to_full_unlock: u64,
    unstake_lock_duration: u64,
    start_fee_distribute_timestamp: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct VaultCreatedEvent(pub VaultCreated);
impl BorshSerialize for VaultCreatedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        VAULT_CREATED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl VaultCreatedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != VAULT_CREATED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        VAULT_CREATED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(VaultCreated::deserialize(buf)?))
    }
}
pub const STAKE_ESCROW_CREATED_EVENT_DISCM: [u8; 8] = [
    30,
    193,
    6,
    249,
    124,
    123,
    229,
    152,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct StakeEscrowCreated {
    pool: Pubkey,
    vault: Pubkey,
    escrow: Pubkey,
    owner: Pubkey,
    full_balance_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct StakeEscrowCreatedEvent(pub StakeEscrowCreated);
impl BorshSerialize for StakeEscrowCreatedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        STAKE_ESCROW_CREATED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl StakeEscrowCreatedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != STAKE_ESCROW_CREATED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        STAKE_ESCROW_CREATED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(StakeEscrowCreated::deserialize(buf)?))
    }
}
pub const CONFIG_CREATED_EVENT_DISCM: [u8; 8] = [195, 73, 104, 161, 166, 245, 4, 120];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ConfigCreated {
    config: Pubkey,
    index: u64,
    seconds_to_full_unlock: u64,
    unstake_lock_duration: u64,
    join_window_duration: u64,
    top_list_length: u16,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigCreatedEvent(pub ConfigCreated);
impl BorshSerialize for ConfigCreatedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CONFIG_CREATED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ConfigCreatedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CONFIG_CREATED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CONFIG_CREATED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ConfigCreated::deserialize(buf)?))
    }
}
pub const CONFIG_CLOSED_EVENT_DISCM: [u8; 8] = [4, 138, 208, 218, 204, 236, 118, 199];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ConfigClosed {
    config: Pubkey,
    index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ConfigClosedEvent(pub ConfigClosed);
impl BorshSerialize for ConfigClosedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CONFIG_CLOSED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ConfigClosedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CONFIG_CLOSED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CONFIG_CLOSED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ConfigClosed::deserialize(buf)?))
    }
}
pub const UNSTAKE_CREATED_EVENT_DISCM: [u8; 8] = [8, 148, 18, 227, 107, 164, 235, 112];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UnstakeCreated {
    unstake: Pubkey,
    pool: Pubkey,
    vault: Pubkey,
    owner: Pubkey,
    amount: u64,
    new_stake_escrow_amount: u64,
    new_stake_escrow_ongoing_total_unstake_amount: u64,
    fee_a_pending: u64,
    fee_b_pending: u64,
    fee_a_per_liquidity_checkpoint: u128,
    fee_b_per_liquidity_checkpoint: u128,
    start_at: i64,
    end_at: i64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UnstakeCreatedEvent(pub UnstakeCreated);
impl BorshSerialize for UnstakeCreatedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UNSTAKE_CREATED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UnstakeCreatedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UNSTAKE_CREATED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UNSTAKE_CREATED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UnstakeCreated::deserialize(buf)?))
    }
}
pub const CANCEL_UNSTAKE_SUCCEED_EVENT_DISCM: [u8; 8] = [
    82,
    107,
    69,
    158,
    67,
    109,
    78,
    147,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct CancelUnstakeSucceed {
    unstake: Pubkey,
    pool: Pubkey,
    vault: Pubkey,
    owner: Pubkey,
    amount: u64,
    new_stake_escrow_amount: u64,
    new_stake_escrow_ongoing_total_unstake_amount: u64,
    fee_a_pending: u64,
    fee_b_pending: u64,
    fee_a_per_liquidity_checkpoint: u128,
    fee_b_per_liquidity_checkpoint: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct CancelUnstakeSucceedEvent(pub CancelUnstakeSucceed);
impl BorshSerialize for CancelUnstakeSucceedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CANCEL_UNSTAKE_SUCCEED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl CancelUnstakeSucceedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CANCEL_UNSTAKE_SUCCEED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CANCEL_UNSTAKE_SUCCEED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(CancelUnstakeSucceed::deserialize(buf)?))
    }
}
pub const WITHDRAW_SUCCEED_EVENT_DISCM: [u8; 8] = [14, 37, 122, 205, 115, 39, 159, 28];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct WithdrawSucceed {
    unstake: Pubkey,
    pool: Pubkey,
    vault: Pubkey,
    owner: Pubkey,
    amount: u64,
    new_stake_escrow_ongoing_total_unstake_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct WithdrawSucceedEvent(pub WithdrawSucceed);
impl BorshSerialize for WithdrawSucceedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        WITHDRAW_SUCCEED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl WithdrawSucceedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != WITHDRAW_SUCCEED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        WITHDRAW_SUCCEED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(WithdrawSucceed::deserialize(buf)?))
    }
}
pub const CLAIM_FEE_SUCCEED_EVENT_DISCM: [u8; 8] = [254, 25, 29, 83, 115, 189, 144, 18];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ClaimFeeSucceed {
    stake_escrow: Pubkey,
    pool: Pubkey,
    vault: Pubkey,
    owner: Pubkey,
    fee_a_amount: u64,
    fee_b_amount: u64,
    total_fee_a_amount: u128,
    total_fee_b_amount: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ClaimFeeSucceedEvent(pub ClaimFeeSucceed);
impl BorshSerialize for ClaimFeeSucceedEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        CLAIM_FEE_SUCCEED_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ClaimFeeSucceedEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != CLAIM_FEE_SUCCEED_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        CLAIM_FEE_SUCCEED_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ClaimFeeSucceed::deserialize(buf)?))
    }
}
pub const FEE_EMISSION_EVENT_DISCM: [u8; 8] = [109, 105, 68, 86, 142, 4, 115, 27];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct FeeEmission {
    pool: Pubkey,
    vault: Pubkey,
    token_a_claimed: u64,
    token_b_claimed: u64,
    token_a_released: u64,
    token_b_released: u64,
    cumulative_fee_a_per_liquidity: u128,
    cumulative_fee_b_per_liquidity: u128,
    effective_stake_amount: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct FeeEmissionEvent(pub FeeEmission);
impl BorshSerialize for FeeEmissionEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        FEE_EMISSION_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl FeeEmissionEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != FEE_EMISSION_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        FEE_EMISSION_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(FeeEmission::deserialize(buf)?))
    }
}
pub const ADD_NEW_USER_TO_TOP_HOLDER_EVENT_DISCM: [u8; 8] = [
    215,
    17,
    200,
    127,
    224,
    180,
    114,
    200,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct AddNewUserToTopHolder {
    pool: Pubkey,
    vault: Pubkey,
    owner: Pubkey,
    stake_amount: u64,
    fee_a_pending: u64,
    fee_b_pending: u64,
    fee_a_per_liquidity_checkpoint: u128,
    fee_b_per_liquidity_checkpoint: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct AddNewUserToTopHolderEvent(pub AddNewUserToTopHolder);
impl BorshSerialize for AddNewUserToTopHolderEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        ADD_NEW_USER_TO_TOP_HOLDER_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl AddNewUserToTopHolderEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != ADD_NEW_USER_TO_TOP_HOLDER_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        ADD_NEW_USER_TO_TOP_HOLDER_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(AddNewUserToTopHolder::deserialize(buf)?))
    }
}
pub const REMOVE_USER_FROM_TOP_HOLDER_EVENT_DISCM: [u8; 8] = [
    244,
    213,
    191,
    20,
    241,
    134,
    37,
    235,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct RemoveUserFromTopHolder {
    pool: Pubkey,
    vault: Pubkey,
    owner: Pubkey,
    stake_amount: u64,
    fee_a_pending: u64,
    fee_b_pending: u64,
    fee_a_per_liquidity_checkpoint: u128,
    fee_b_per_liquidity_checkpoint: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct RemoveUserFromTopHolderEvent(pub RemoveUserFromTopHolder);
impl BorshSerialize for RemoveUserFromTopHolderEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        REMOVE_USER_FROM_TOP_HOLDER_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl RemoveUserFromTopHolderEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != REMOVE_USER_FROM_TOP_HOLDER_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        REMOVE_USER_FROM_TOP_HOLDER_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(RemoveUserFromTopHolder::deserialize(buf)?))
    }
}
pub const USER_STAKE_EVENT_DISCM: [u8; 8] = [195, 190, 70, 231, 232, 75, 51, 151];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UserStake {
    pool: Pubkey,
    vault: Pubkey,
    owner: Pubkey,
    stake_amount: u64,
    total_stake_amount: u64,
    fee_a_pending: u64,
    fee_b_pending: u64,
    fee_a_per_liquidity_checkpoint: u128,
    fee_b_per_liquidity_checkpoint: u128,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UserStakeEvent(pub UserStake);
impl BorshSerialize for UserStakeEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        USER_STAKE_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UserStakeEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != USER_STAKE_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        USER_STAKE_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UserStake::deserialize(buf)?))
    }
}
pub const RECLAIM_INDEX_EVENT_DISCM: [u8; 8] = [134, 152, 42, 196, 107, 132, 35, 222];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct ReclaimIndex {
    vault: Pubkey,
    in_owner: Pubkey,
    in_owner_balance: u64,
    out_owner: Pubkey,
    out_owner_balance: u64,
    reclaim_index: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct ReclaimIndexEvent(pub ReclaimIndex);
impl BorshSerialize for ReclaimIndexEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        RECLAIM_INDEX_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl ReclaimIndexEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != RECLAIM_INDEX_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        RECLAIM_INDEX_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(ReclaimIndex::deserialize(buf)?))
    }
}
pub const UPDATE_UNSTAKE_LOCK_DURATION_EVENT_DISCM: [u8; 8] = [
    117,
    134,
    16,
    232,
    145,
    24,
    175,
    95,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateUnstakeLockDuration {
    vault: Pubkey,
    old_value: u64,
    new_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateUnstakeLockDurationEvent(pub UpdateUnstakeLockDuration);
impl BorshSerialize for UpdateUnstakeLockDurationEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_UNSTAKE_LOCK_DURATION_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateUnstakeLockDurationEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_UNSTAKE_LOCK_DURATION_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_UNSTAKE_LOCK_DURATION_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateUnstakeLockDuration::deserialize(buf)?))
    }
}
pub const UPDATE_SECONDS_TO_FULL_UNLOCK_EVENT_DISCM: [u8; 8] = [
    82,
    125,
    66,
    63,
    78,
    209,
    125,
    196,
];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct UpdateSecondsToFullUnlock {
    vault: Pubkey,
    old_value: u64,
    new_value: u64,
}
#[derive(Clone, Debug, PartialEq)]
pub struct UpdateSecondsToFullUnlockEvent(pub UpdateSecondsToFullUnlock);
impl BorshSerialize for UpdateSecondsToFullUnlockEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        UPDATE_SECONDS_TO_FULL_UNLOCK_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl UpdateSecondsToFullUnlockEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != UPDATE_SECONDS_TO_FULL_UNLOCK_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        UPDATE_SECONDS_TO_FULL_UNLOCK_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(UpdateSecondsToFullUnlock::deserialize(buf)?))
    }
}
