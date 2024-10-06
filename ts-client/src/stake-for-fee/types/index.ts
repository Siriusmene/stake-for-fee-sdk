import { IdlAccounts, IdlTypes, Program } from "@coral-xyz/anchor";
import { StakeForFee } from "../idls/stake_for_fee";
import { Amm } from "../idls/dynamic_amm";
import { Vault } from "../idls/dynamic_vault";

export type StakeForFeeProgram = Program<StakeForFee>;
export type FeeVault = IdlAccounts<StakeForFee>["feeVault"];
export type FullBalanceListMetadata =
  IdlAccounts<StakeForFee>["fullBalanceListMetadata"];
export type TopListMetadata = IdlAccounts<StakeForFee>["topListMetadata"];
export type StakerMetadata = IdlTypes<StakeForFee>["StakerMetadata"];
export type StakerBalance = IdlTypes<StakeForFee>["StakerBalance"];

export type DynamicAmmProgram = Program<Amm>;
export type DynamicPool = IdlAccounts<Amm>["pool"];

export type DynamicVaultProgram = Program<Vault>;
export type DynamicVault = IdlAccounts<Vault>["vault"];

export interface FullBalanceListState {
  metadata: FullBalanceListMetadata;
  stakers: Array<StakerBalance>;
}

export interface TopStakerListState {
  metadata: TopListMetadata;
  stakers: Array<StakerMetadata>;
}
