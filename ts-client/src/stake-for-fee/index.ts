import { AnchorProvider, BN, Program } from "@coral-xyz/anchor";
import {
  AccountMeta,
  Connection,
  Keypair,
  PublicKey,
  SystemProgram,
  Transaction,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  DYNAMIC_AMM_PROGRAM_ID,
  DYNAMIC_VAULT_PROGRAM_ID,
  FULL_BALANCE_LIST_HARD_LIMIT,
  STAKE_FOR_FEE_PROGRAM_ID,
} from "./constants";
import {
  deriveFeeVault,
  deriveFullBalanceList,
  deriveStakeEscrow,
  deriveStakeTokenVault,
  deriveTopStakerList,
} from "./helpers/pda";
import {
  ASSOCIATED_TOKEN_PROGRAM_ID,
  getAssociatedTokenAddressSync,
  TOKEN_PROGRAM_ID,
} from "@solana/spl-token";
import {
  DynamicPool,
  DynamicVault,
  FeeVault,
  FullBalanceListState,
  StakeForFeeProgram,
  TopStakerListState,
  DynamicVaultProgram,
  DynamicAmmProgram,
  StakerMetadata,
} from "./types";
import {
  createDynamicAmmProgram,
  createDynamicVaultProgram,
  createStakeFeeProgram,
  getOrCreateATAInstruction,
  getOrCreateStakeEscrowInstruction,
} from "./helpers/program";
import {
  decodeFullBalanceState,
  decodeTopStakerListState,
} from "./helpers/decoder";

type Opt = {
  stakeForFeeProgramId?: PublicKey;
  dynamicAmmProgramId?: PublicKey;
  dynamicVaultProgramId?: PublicKey;
};

export class StakeForFee {
  constructor(
    public connection: Connection,
    public stakeForFeeProgram: StakeForFeeProgram,
    public dynamicAmmProgram: DynamicAmmProgram,
    public dynamicVaultProgram: DynamicVaultProgram,
    public feeVaultKey: PublicKey,
    public feeVault: FeeVault,
    public fullBalanceListState: FullBalanceListState,
    public topStakerListState: TopStakerListState,
    public aVault: DynamicVault,
    public bVault: DynamicVault,
    public ammPool: DynamicPool,
    public escrowVaultKey: PublicKey
  ) {}

  static async create(
    connection: Connection,
    pool: PublicKey,
    opt?: Opt
  ): Promise<StakeForFee> {
    const stakeForFeeProgram = createStakeFeeProgram(
      connection,
      opt?.stakeForFeeProgramId ?? STAKE_FOR_FEE_PROGRAM_ID
    );

    const dynamicVaultProgram = createDynamicVaultProgram(
      connection,
      opt?.dynamicVaultProgramId ?? DYNAMIC_VAULT_PROGRAM_ID
    );

    const dynamicAmmProgram = createDynamicAmmProgram(
      connection,
      opt?.dynamicAmmProgramId ?? DYNAMIC_AMM_PROGRAM_ID
    );

    const feeVaultKey = deriveFeeVault(pool, stakeForFeeProgram.programId);
    const fullBalanceListKey = deriveFullBalanceList(
      feeVaultKey,
      stakeForFeeProgram.programId
    );
    const topStakerListKey = deriveTopStakerList(
      feeVaultKey,
      stakeForFeeProgram.programId
    );

    const [
      feeVaultAccount,
      fullBalanceListAccount,
      topStakerAccount,
      poolAccount,
    ] = await connection.getMultipleAccountsInfo([
      feeVaultKey,
      fullBalanceListKey,
      topStakerListKey,
      pool,
    ]);

    const feeVaultState = stakeForFeeProgram.coder.accounts.decode(
      "feeVault",
      feeVaultAccount.data
    );

    const fullBalanceListState = decodeFullBalanceState(
      stakeForFeeProgram,
      fullBalanceListAccount
    );

    const topStakerListState = decodeTopStakerListState(
      stakeForFeeProgram,
      feeVaultState,
      topStakerAccount
    );

    const poolState: DynamicPool = dynamicAmmProgram.coder.accounts.decode(
      "pool",
      poolAccount.data
    );

    const [aVaultAccount, bVaultAccount] =
      await connection.getMultipleAccountsInfo([
        poolState.aVault,
        poolState.bVault,
      ]);

    const aVaultState: DynamicVault = dynamicVaultProgram.coder.accounts.decode(
      "vault",
      aVaultAccount.data
    );

    const bVaultState: DynamicVault = dynamicVaultProgram.coder.accounts.decode(
      "vault",
      bVaultAccount.data
    );

    const escrowVaultKey = getAssociatedTokenAddressSync(
      poolState.lpMint,
      feeVaultKey,
      true
    );

    return new StakeForFee(
      connection,
      stakeForFeeProgram,
      dynamicAmmProgram,
      dynamicVaultProgram,
      feeVaultKey,
      feeVaultState,
      fullBalanceListState,
      topStakerListState,
      aVaultState,
      bVaultState,
      poolState,
      escrowVaultKey
    );
  }

  /**
   * Gets all unstake records for the given stake escrow.
   *
   * @param connection The connection to use.
   * @param stakeEscrow The stake escrow to get the unstake records for.
   * @returns A promise that resolves with an array of unstake records that match the given stake escrow.
   */
  static async getUnstakeByStakeEscrow(
    connection: Connection,
    stakeEscrow: PublicKey
  ) {
    const stakeForFeeProgram = createStakeFeeProgram(
      connection,
      STAKE_FOR_FEE_PROGRAM_ID
    );
    return await stakeForFeeProgram.account.unstake.all([
      {
        memcmp: {
          offset: 8,
          bytes: stakeEscrow.toBase58(),
        },
      },
    ]);
  }

  /**
   * Gets all config accounts for the given stake-for-fee program.
   * @param connection The connection to use.
   * @param programId The program id of the stake-for-fee program. Defaults to the idl program id.
   * @returns A promise that resolves with an array of config accounts.
   */
  static async getConfigs(connection: Connection, programId?: PublicKey) {
    const stakeForFeeProgram = createStakeFeeProgram(
      connection,
      programId ?? STAKE_FOR_FEE_PROGRAM_ID
    );
    return await stakeForFeeProgram.account.config.all();
  }

  /**
   * Gets all stake escrow accounts for the given owner.
   * @param connection The connection to use.
   * @param owner The owner to get the stake escrow accounts for.
   * @param programId The program id of the stake-for-fee program. Defaults to the idl program id.
   * @returns A promise that resolves with an array of stake escrow accounts.
   */
  static async getStakeEscrowsByOwner(
    connection: Connection,
    owner: PublicKey,
    programId?: PublicKey
  ) {
    const stakeForFeeProgram = createStakeFeeProgram(
      connection,
      programId ?? STAKE_FOR_FEE_PROGRAM_ID
    );

    return stakeForFeeProgram.account.stakeEscrow.all([
      { memcmp: { offset: 8, bytes: owner.toBase58() } },
    ]);
  }

  /**
   * Creates a fee vault for the given pool.
   *
   * @param connection Solana connection
   * @param pool The pool to create the fee vault for
   * @param stakeMint The mint of the stake token
   * @param lockEscrow The lock escrow account. The owner of the lock escrow must be the fee vault. Integrator must create lock escrow account before calling this instruction.
   * @param payer The payer of the transaction. Signer.
   * @param config The configuration account for the fee vault. Get from `getConfigs`
   * @param customStartClaimFeeTimestamp The custom start claim fee timestamp. If not passed, it will default to current timestamp. Else, lock escrow can only claim fee after this timestamp. Constraint: currentTimestamp <= `customStartClaimFeeTimestamp` <= currentTimestamp + configAccount.joinWindowDuration
   * @param opt Optional options
   *
   * @returns A transaction that can be signed and sent to the network
   */
  public static async createFeeVault(
    connection: Connection,
    pool: PublicKey,
    stakeMint: PublicKey,
    lockEscrow: PublicKey,
    payer: PublicKey,
    config: PublicKey,
    customStartClaimFeeTimestamp?: BN,
    opt?: Opt
  ): Promise<Transaction> {
    const stakeForFeeProgram = createStakeFeeProgram(
      connection,
      opt?.stakeForFeeProgramId ?? STAKE_FOR_FEE_PROGRAM_ID
    );

    const ammProgram = createDynamicAmmProgram(
      connection,
      opt?.dynamicAmmProgramId ?? DYNAMIC_AMM_PROGRAM_ID
    );

    const poolState = await ammProgram.account.pool.fetch(pool);

    const feeVaultKey = deriveFeeVault(pool, stakeForFeeProgram.programId);
    const stakeTokenVaultKey = deriveStakeTokenVault(
      feeVaultKey,
      stakeForFeeProgram.programId
    );
    const topStakerListKey = deriveTopStakerList(
      feeVaultKey,
      stakeForFeeProgram.programId
    );
    const fullBalanceListKey = deriveFullBalanceList(
      feeVaultKey,
      stakeForFeeProgram.programId
    );
    const tokenAVaultKey = getAssociatedTokenAddressSync(
      poolState.tokenAMint,
      feeVaultKey,
      true
    );
    const tokenBVaultKey = getAssociatedTokenAddressSync(
      poolState.tokenBMint,
      feeVaultKey,
      true
    );

    const transaction = await stakeForFeeProgram.methods
      .initializeVault(customStartClaimFeeTimestamp)
      .accounts({
        config,
        vault: feeVaultKey,
        stakeTokenVault: stakeTokenVaultKey,
        stakeMint,
        topStakerList: topStakerListKey,
        fullBalanceList: fullBalanceListKey,
        tokenAMint: poolState.tokenAMint,
        tokenBMint: poolState.tokenBMint,
        pool,
        tokenAVault: tokenAVaultKey,
        tokenBVault: tokenBVaultKey,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        lockEscrow,
        payer,
        systemProgram: SystemProgram.programId,
      })
      .transaction();

    const { blockhash, lastValidBlockHeight } =
      await connection.getLatestBlockhash("confirmed");

    return new Transaction({
      blockhash,
      lastValidBlockHeight,
      feePayer: payer,
    }).add(transaction);
  }

  /**
   * Initializes a stake escrow for the given owner.
   *
   * @param connection Solana connection
   * @param owner The owner of the stake escrow. Signer.
   *
   * @returns A transaction that can be signed and sent to the network
   */
  public async initializeStakeEscrow(owner: PublicKey): Promise<Transaction> {
    const stakeEscrowKey = deriveStakeEscrow(
      this.feeVaultKey,
      owner,
      this.stakeForFeeProgram.programId
    );

    const transaction = await this.stakeForFeeProgram.methods
      .initializeStakeEscrow()
      .accounts({
        vault: this.feeVaultKey,
        fullBalanceList: this.feeVault.fullBalanceList,
        topStakerList: this.feeVault.topStakerList,
        escrow: stakeEscrowKey,
        owner,
        systemProgram: SystemProgram.programId,
      })
      .transaction();

    const { blockhash, lastValidBlockHeight } =
      await this.connection.getLatestBlockhash("confirmed");

    return new Transaction({
      blockhash,
      lastValidBlockHeight,
      feePayer: owner,
    }).add(transaction);
  }

  findSmallestStakeEscrowInFullBalanceList(
    skipOwner: PublicKey
  ): PublicKey | null {
    const endIdx = FULL_BALANCE_LIST_HARD_LIMIT.toNumber() - 1;
    let smallestBalance = new BN("18446744073709551615");
    let smallestOwner: PublicKey = null;

    for (let i = endIdx; i >= 0; i--) {
      const staker = this.fullBalanceListState.stakers[i];
      if (staker.owner.equals(skipOwner)) {
        continue;
      }

      if (staker.balance.isZero()) {
        smallestOwner = staker.owner;
        break;
      }

      if (staker.balance.lt(smallestBalance)) {
        smallestOwner = staker.owner;
        smallestBalance = staker.balance;
      }
    }

    return (
      smallestOwner ??
      deriveStakeEscrow(
        this.feeVaultKey,
        smallestOwner,
        this.stakeForFeeProgram.programId
      )
    );
  }

  findClosestStakeByBalanceInFullBalanceList(
    lookupNumber: number,
    stakeAmount: BN,
    skipIdx: number
  ) {
    const closestStakers: Array<{ idx: BN; balance: BN; owner: PublicKey }> =
      [];

    const fnFindNextClosestStakersWithSlice = (
      startIdx: number,
      endIdx: number
    ) => {
      for (let i = startIdx; i < endIdx; i++) {
        const staker = this.fullBalanceListState.stakers[i];
        if (staker.balance.gte(stakeAmount)) {
          if (closestStakers.length < lookupNumber) {
            closestStakers.push({
              idx: new BN(i),
              balance: staker.balance,
              owner: staker.owner,
            });
          } else {
            const biggestClosestStakers = closestStakers[lookupNumber - 1];
            if (staker.balance.lt(biggestClosestStakers.balance)) {
              closestStakers.pop();
              closestStakers.push({
                idx: new BN(i),
                balance: staker.balance,
                owner: staker.owner,
              });
            }
          }
          closestStakers.sort((a, b) => {
            if (a.balance.eq(b.balance)) {
              return a.idx.cmp(b.idx);
            } else {
              return a.balance.cmp(b.balance);
            }
          });
        }
      }
    };

    fnFindNextClosestStakersWithSlice(0, skipIdx);
    fnFindNextClosestStakersWithSlice(
      skipIdx + 1,
      this.fullBalanceListState.stakers.length
    );

    return closestStakers.map((s) =>
      deriveStakeEscrow(
        this.feeVaultKey,
        s.owner,
        this.stakeForFeeProgram.programId
      )
    );
  }

  findReplaceableTopStaker(lookupNumber: number) {
    const smallestStakers: Array<StakerMetadata> = [];

    for (const staker of this.topStakerListState.stakers) {
      if (staker.fullBalanceIndex.isNeg()) {
        continue;
      }

      if (smallestStakers.length < lookupNumber) {
        smallestStakers.push(staker);
        smallestStakers.sort((a, b) => {
          if (a.stakeAmount.eq(b.stakeAmount)) {
            return a.fullBalanceIndex.cmp(b.fullBalanceIndex);
          } else {
            return b.stakeAmount.cmp(a.stakeAmount);
          }
        });
      }

      const biggestStakers = smallestStakers[lookupNumber - 1];
      if (staker.stakeAmount.lt(biggestStakers.stakeAmount)) {
        smallestStakers.pop();
        smallestStakers.push(staker);
        smallestStakers.sort((a, b) => {
          if (a.stakeAmount.eq(b.stakeAmount)) {
            return a.fullBalanceIndex.cmp(b.fullBalanceIndex);
          } else {
            return b.stakeAmount.cmp(a.stakeAmount);
          }
        });
      }
    }

    return smallestStakers.map((s) =>
      deriveStakeEscrow(
        this.feeVaultKey,
        s.owner,
        this.stakeForFeeProgram.programId
      )
    );
  }

  /**
   * Withdraws the tokens from the given unstake key and sends them to the given owner.
   * @param unstakeKey The public key of the unstake account to withdraw from.
   * @param owner The public key of the account to send the withdrawn tokens to.
   * @returns A transaction that can be signed and sent to the network.
   */
  public async withdraw(
    unstakeKey: PublicKey,
    owner: PublicKey
  ): Promise<Transaction> {
    const stakeEscrowKey = deriveStakeEscrow(
      this.feeVaultKey,
      owner,
      this.stakeForFeeProgram.programId
    );

    const preInstructions = [];

    const { ataPubKey: userStakeToken, ix: initializeUserStakeTokenIx } =
      await getOrCreateATAInstruction(
        this.connection,
        this.feeVault.stakeMint,
        owner
      );

    initializeUserStakeTokenIx &&
      preInstructions.push(initializeUserStakeTokenIx);

    const transaction = await this.stakeForFeeProgram.methods
      .withdraw()
      .accounts({
        unstake: unstakeKey,
        stakeEscrow: stakeEscrowKey,
        stakeTokenVault: this.feeVault.stakeTokenVault,
        vault: this.feeVaultKey,
        userStakeToken,
        tokenProgram: TOKEN_PROGRAM_ID,
        owner,
      })
      .preInstructions(preInstructions)
      .transaction();

    const { blockhash, lastValidBlockHeight } =
      await this.connection.getLatestBlockhash("confirmed");

    return new Transaction({
      blockhash,
      lastValidBlockHeight,
      feePayer: owner,
    }).add(transaction);
  }

  /**
   * Requests to unstake a given amount of tokens from the vault.
   * Creates a new `Unstake` account and initializes it to the given owner.
   *
   * @param amount The amount of tokens to unstake
   * @param unstakeKeypair The new unstake account. Arbitrarily generated. Signer.
   * @param owner The owner of the stake. Signer.
   * @returns The transaction to execute the unstake instruction
   */
  public async unstake(
    amount: BN,
    unstakeKeypair: PublicKey,
    owner: PublicKey
  ): Promise<Transaction> {
    const stakeEscrowKey = deriveStakeEscrow(
      this.feeVaultKey,
      owner,
      this.stakeForFeeProgram.programId
    );

    const stakeEscrowState =
      await this.stakeForFeeProgram.account.stakeEscrow.fetch(stakeEscrowKey);

    const remainingAccounts: Array<AccountMeta> = [];

    if (Boolean(stakeEscrowState.inTopList)) {
      const closestStakeEscrows: Array<AccountMeta> =
        this.findClosestStakeByBalanceInFullBalanceList(
          3,
          stakeEscrowState.stakeAmount.sub(amount),
          stakeEscrowState.fullBalanceIndex.toNumber()
        ).map((key) => {
          return {
            pubkey: key,
            isSigner: false,
            isWritable: true,
          };
        });

      remainingAccounts.push(...closestStakeEscrows);
    }

    const transaction = await this.stakeForFeeProgram.methods
      .requestUnstake(amount)
      .accounts({
        unstake: unstakeKeypair,
        vault: this.feeVaultKey,
        topStakerList: this.feeVault.topStakerList,
        fullBalanceList: this.feeVault.fullBalanceList,
        stakeEscrow: stakeEscrowKey,
        tokenAVault: this.feeVault.tokenAVault,
        tokenBVault: this.feeVault.tokenBVault,
        owner,
        pool: this.feeVault.pool,
        lpMint: this.ammPool.lpMint,
        lockEscrow: this.feeVault.lockEscrow,
        escrowVault: this.escrowVaultKey,
        aTokenVault: this.aVault.tokenVault,
        bTokenVault: this.bVault.tokenVault,
        aVault: this.ammPool.aVault,
        bVault: this.ammPool.bVault,
        aVaultLp: this.ammPool.aVaultLp,
        bVaultLp: this.ammPool.bVaultLp,
        aVaultLpMint: this.aVault.lpMint,
        bVaultLpMint: this.bVault.lpMint,
        ammProgram: this.dynamicAmmProgram.programId,
        vaultProgram: this.dynamicVaultProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .remainingAccounts(remainingAccounts)
      .transaction();

    const { blockhash, lastValidBlockHeight } =
      await this.connection.getLatestBlockhash("confirmed");

    return new Transaction({
      blockhash,
      lastValidBlockHeight,
      feePayer: owner,
    }).add(transaction);
  }

  /**
   * @description
   * Claim fee from stake escrow.
   *
   * @param owner Owner of stake escrow.
   * @returns Transaction
   */
  public async claimFee(owner: PublicKey): Promise<Transaction> {
    const stakeEscrowKey = deriveStakeEscrow(
      this.feeVaultKey,
      owner,
      this.stakeForFeeProgram.programId
    );

    const preInstructions = [];

    const [
      { ataPubKey: userTokenA, ix: initializeUserTokenAIx },
      { ataPubKey: userTokenB, ix: initializeUserTokenBIx },
    ] = await Promise.all([
      getOrCreateATAInstruction(
        this.connection,
        this.ammPool.tokenAMint,
        owner
      ),
      getOrCreateATAInstruction(
        this.connection,
        this.ammPool.tokenBMint,
        owner
      ),
    ]);

    initializeUserTokenAIx && preInstructions.push(initializeUserTokenAIx);
    initializeUserTokenBIx && preInstructions.push(initializeUserTokenBIx);

    const transaction = await this.stakeForFeeProgram.methods
      .claimFee()
      .accounts({
        userTokenA,
        userTokenB,
        vault: this.feeVaultKey,
        topStakerList: this.feeVault.topStakerList,
        stakeEscrow: stakeEscrowKey,
        tokenAVault: this.feeVault.tokenAVault,
        tokenBVault: this.feeVault.tokenBVault,
        owner,
        pool: this.feeVault.pool,
        lpMint: this.ammPool.lpMint,
        lockEscrow: this.feeVault.lockEscrow,
        escrowVault: this.escrowVaultKey,
        aTokenVault: this.aVault.tokenVault,
        bTokenVault: this.bVault.tokenVault,
        aVault: this.ammPool.aVault,
        bVault: this.ammPool.bVault,
        aVaultLp: this.ammPool.aVaultLp,
        bVaultLp: this.ammPool.bVaultLp,
        aVaultLpMint: this.aVault.lpMint,
        bVaultLpMint: this.bVault.lpMint,
        ammProgram: this.dynamicAmmProgram.programId,
        vaultProgram: this.dynamicVaultProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .preInstructions(preInstructions)
      .transaction();

    const { blockhash, lastValidBlockHeight } =
      await this.connection.getLatestBlockhash("confirmed");

    return new Transaction({
      blockhash,
      lastValidBlockHeight,
      feePayer: owner,
    }).add(transaction);
  }

  /**
   * Stake tokens in the vault for the given owner.
   *
   * Creates a new stake escrow if one doesn't exist.
   *
   * @param amount The amount of tokens to stake
   * @param owner The owner of the stake. Signer.
   * @returns The transaction to execute the stake instruction
   */
  public async stake(amount: BN, owner: PublicKey): Promise<Transaction> {
    const preInstructions: Array<TransactionInstruction> = [];
    const { stakeEscrowKey, ix: initializeStakeEscrowIx } =
      await getOrCreateStakeEscrowInstruction(
        this.connection,
        this.feeVaultKey,
        owner,
        this.stakeForFeeProgram.programId
      );

    initializeStakeEscrowIx && preInstructions.push(initializeStakeEscrowIx);

    const { ataPubKey: userStakeTokenKey, ix: initializeUserStakeTokenIx } =
      await getOrCreateATAInstruction(
        this.connection,
        this.feeVault.stakeMint,
        owner
      );

    initializeUserStakeTokenIx &&
      preInstructions.push(initializeUserStakeTokenIx);

    const remainingAccounts: Array<AccountMeta> = [];
    if (!initializeStakeEscrowIx) {
      const stakeEscrowState =
        await this.stakeForFeeProgram.account.stakeEscrow.fetch(stakeEscrowKey);

      if (!Boolean(stakeEscrowState.inTopList)) {
        const smallestStakeEscrows: Array<AccountMeta> =
          this.findReplaceableTopStaker(3).map((key) => {
            return {
              pubkey: key,
              isWritable: true,
              isSigner: false,
            };
          });

        remainingAccounts.push(...smallestStakeEscrows);
      }
    }

    const smallestStakeEscrow =
      this.findSmallestStakeEscrowInFullBalanceList(owner);

    const transaction = await this.stakeForFeeProgram.methods
      .stake(amount)
      .accounts({
        vault: this.feeVaultKey,
        stakeTokenVault: this.feeVault.stakeTokenVault,
        topStakerList: this.feeVault.topStakerList,
        fullBalanceList: this.feeVault.fullBalanceList,
        stakeEscrow: stakeEscrowKey,
        smallestStakeEscrow,
        userStakeToken: userStakeTokenKey,
        tokenAVault: this.feeVault.tokenAVault,
        tokenBVault: this.feeVault.tokenBVault,
        owner,
        pool: this.feeVault.pool,
        lpMint: this.ammPool.lpMint,
        lockEscrow: this.feeVault.lockEscrow,
        escrowVault: this.escrowVaultKey,
        aTokenVault: this.aVault.tokenVault,
        bTokenVault: this.bVault.tokenVault,
        aVault: this.ammPool.aVault,
        bVault: this.ammPool.bVault,
        aVaultLp: this.ammPool.aVaultLp,
        bVaultLp: this.ammPool.bVaultLp,
        aVaultLpMint: this.aVault.lpMint,
        bVaultLpMint: this.bVault.lpMint,
        ammProgram: this.dynamicAmmProgram.programId,
        vaultProgram: this.dynamicVaultProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .preInstructions(preInstructions)
      .remainingAccounts(remainingAccounts)
      .transaction();

    const { blockhash, lastValidBlockHeight } =
      await this.connection.getLatestBlockhash("confirmed");

    return new Transaction({
      blockhash,
      lastValidBlockHeight,
      feePayer: owner,
    }).add(transaction);
  }

  /**
   * Cancels an unstake request, given the public key of the unstake account.
   * @param unstakeKey The public key of the unstake account to cancel.
   * @param owner The public key of the owner of the stake.
   * @returns A transaction to execute the cancel unstake instruction.
   */
  public async cancelUnstake(
    unstakeKey: PublicKey,
    owner: PublicKey
  ): Promise<Transaction> {
    const stakeEscrowKey = deriveStakeEscrow(
      this.feeVaultKey,
      owner,
      this.stakeForFeeProgram.programId
    );

    const remainingAccounts: Array<AccountMeta> = [];
    const stakeEscrowState =
      await this.stakeForFeeProgram.account.stakeEscrow.fetch(stakeEscrowKey);

    if (!Boolean(stakeEscrowState.inTopList)) {
      const smallestStakeEscrows: Array<AccountMeta> =
        this.findReplaceableTopStaker(3).map((key) => {
          return {
            pubkey: key,
            isWritable: true,
            isSigner: false,
          };
        });

      remainingAccounts.push(...smallestStakeEscrows);
    }

    const smallestStakeEscrow =
      this.findSmallestStakeEscrowInFullBalanceList(owner);

    const transaction = await this.stakeForFeeProgram.methods
      .cancelUnstake()
      .accounts({
        unstake: unstakeKey,
        vault: this.feeVaultKey,
        topStakerList: this.feeVault.topStakerList,
        fullBalanceList: this.feeVault.fullBalanceList,
        stakeEscrow: stakeEscrowKey,
        smallestStakeEscrow,
        tokenAVault: this.feeVault.tokenAVault,
        tokenBVault: this.feeVault.tokenBVault,
        owner,
        pool: this.feeVault.pool,
        lpMint: this.ammPool.lpMint,
        lockEscrow: this.feeVault.lockEscrow,
        escrowVault: this.escrowVaultKey,
        aTokenVault: this.aVault.tokenVault,
        bTokenVault: this.bVault.tokenVault,
        aVault: this.ammPool.aVault,
        bVault: this.ammPool.bVault,
        aVaultLp: this.ammPool.aVaultLp,
        bVaultLp: this.ammPool.bVaultLp,
        aVaultLpMint: this.aVault.lpMint,
        bVaultLpMint: this.bVault.lpMint,
        ammProgram: this.dynamicAmmProgram.programId,
        vaultProgram: this.dynamicVaultProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
      })
      .remainingAccounts(remainingAccounts)
      .transaction();

    const { blockhash, lastValidBlockHeight } =
      await this.connection.getLatestBlockhash("confirmed");

    return new Transaction({
      blockhash,
      lastValidBlockHeight,
      feePayer: owner,
    }).add(transaction);
  }
}
