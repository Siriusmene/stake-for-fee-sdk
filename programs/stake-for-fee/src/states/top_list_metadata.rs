#![allow(dead_code)]
use anchor_lang::prelude::*;
use std::cell::RefMut;

#[account(zero_copy)]
#[derive(InitSpace, Debug, Default)]
pub struct TopListMetadata {
    /// Vault
    pub vault: Pubkey,
}

#[zero_copy]
#[derive(InitSpace, Debug, Default)]
pub struct StakerMetadata {
    /// Staked amount
    pub stake_amount: u64,
    /// Full balance list index. When it's negative, the slot is empty
    pub full_balance_index: i64,
    /// Owner pubkey, we dont need this for logic, but it is usefull for indexing
    pub owner: Pubkey,
}

#[derive(Debug)]
pub struct TopStakerList<'a> {
    pub metadata: RefMut<'a, TopListMetadata>,
    pub stakers: RefMut<'a, [StakerMetadata]>,
}

impl<'a> TopStakerList<'a> {
    pub fn from_account_loader<'info>(
        top_list_metadata_al: &'a AccountLoader<'info, TopListMetadata>,
    ) -> Result<Self> {
        top_list_metadata_account_split(top_list_metadata_al)
    }

    pub fn new(
        metadata: RefMut<'a, TopListMetadata>,
        stakers: RefMut<'a, [StakerMetadata]>,
    ) -> Self {
        Self { metadata, stakers }
    }
}

fn top_list_metadata_account_split<'a, 'info>(
    top_list_metadata_al: &'a AccountLoader<'info, TopListMetadata>,
) -> Result<TopStakerList<'a>> {
    let data = top_list_metadata_al.as_ref().try_borrow_mut_data()?;

    let (top_list_metadata, top_stakers) = RefMut::map_split(data, |data| {
        let (top_list_metadata_bytes, top_stakers_bytes) =
            data.split_at_mut(8 + TopListMetadata::INIT_SPACE);
        let top_list_metadata =
            bytemuck::from_bytes_mut::<TopListMetadata>(&mut top_list_metadata_bytes[8..]);
        let top_stakers = bytemuck::cast_slice_mut::<u8, StakerMetadata>(top_stakers_bytes);
        (top_list_metadata, top_stakers)
    });

    Ok(TopStakerList::new(top_list_metadata, top_stakers))
}
