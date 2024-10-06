export type StakeForFee = {
  "version": "0.1.0",
  "name": "stake_for_fee",
  "instructions": [
    {
      "name": "initializeVault",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Config account"
          ]
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Vault account"
          ]
        },
        {
          "name": "stakeTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Stake token vault"
          ]
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Token x vault"
          ]
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Token y vault"
          ]
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Top staker list account"
          ]
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Full balance list account"
          ]
        },
        {
          "name": "pool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Token a mint of the pool"
          ]
        },
        {
          "name": "tokenBMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Token b mint of the pool"
          ]
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Stake mint of the pool"
          ]
        },
        {
          "name": "lockEscrow",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "customStartClaimFeeTimestamp",
          "type": {
            "option": "i64"
          }
        }
      ]
    },
    {
      "name": "initializeStakeEscrow",
      "accounts": [
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stake",
      "accounts": [
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "smallestStakeEscrow",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "userStakeToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimFee",
      "accounts": [
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenX",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenY",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "requestUnstake",
      "accounts": [
        {
          "name": "unstake",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "unstakeAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancelUnstake",
      "accounts": [
        {
          "name": "unstake",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "smallestStakeEscrow",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "withdraw",
      "accounts": [
        {
          "name": "unstake",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userStakeToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "dummy",
      "docs": [
        "To force IDL generation for some struct for easier TS decoding later"
      ],
      "accounts": [
        {
          "name": "stakerMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakerBalance",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeConfig",
      "docs": [
        "Start of admin only endpoints *"
      ],
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "InitializeConfigParams"
          }
        }
      ]
    },
    {
      "name": "closeConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "stakerMetadataDummyAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakerMetadata",
            "type": {
              "defined": "StakerMetadata"
            }
          }
        ]
      }
    },
    {
      "name": "stakerBalanceDummyAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakerBalance",
            "type": {
              "defined": "StakerBalance"
            }
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "secondsToFullUnlock",
            "docs": [
              "Seconds for lock escrow claimed fee to be fully dripped to the top stakers"
            ],
            "type": "u64"
          },
          {
            "name": "unstakeLockDuration",
            "docs": [
              "Seconds for the requested unstake to withdraw the capital"
            ],
            "type": "u64"
          },
          {
            "name": "joinWindowDuration",
            "docs": [
              "Time window (in seconds) for staker to stake before the first lock escrow claim fee happen. This is to ensure that there's enough time for staker to join the list, so the first claim fee with huge amount will be more fairly distributed."
            ],
            "type": "u64"
          },
          {
            "name": "topListLength",
            "docs": [
              "Maximum number of stakers in the top list"
            ],
            "type": "u16"
          },
          {
            "name": "padding0",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          },
          {
            "name": "padding",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u128",
                20
              ]
            }
          }
        ]
      }
    },
    {
      "name": "fullBalanceListMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vault",
            "type": "publicKey"
          },
          {
            "name": "length",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stakeEscrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "vault",
            "type": "publicKey"
          },
          {
            "name": "fullBalanceIndex",
            "type": "u64"
          },
          {
            "name": "stakeAmount",
            "type": "u64"
          },
          {
            "name": "inTopList",
            "type": "u8"
          },
          {
            "name": "padding0",
            "type": {
              "array": [
                "u8",
                15
              ]
            }
          },
          {
            "name": "ongoingTotalPartialUnstakeAmount",
            "type": "u64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "feeXClaimedAmount",
            "type": "u128"
          },
          {
            "name": "feeYClaimedAmount",
            "type": "u128"
          },
          {
            "name": "feeXPerLiquidityCheckpoint",
            "type": "u128"
          },
          {
            "name": "feeYPerLiquidityCheckpoint",
            "type": "u128"
          },
          {
            "name": "feeXPending",
            "type": "u64"
          },
          {
            "name": "feeYPending",
            "type": "u64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u128",
                20
              ]
            }
          }
        ]
      }
    },
    {
      "name": "topListMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vault",
            "docs": [
              "Vault"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "unstake",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakeEscrow",
            "type": "publicKey"
          },
          {
            "name": "unstakeAmount",
            "type": "u64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "releaseAt",
            "type": "i64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u64",
                30
              ]
            }
          }
        ]
      }
    },
    {
      "name": "feeVault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lockEscrow",
            "docs": [
              "Lock escrow account"
            ],
            "type": "publicKey"
          },
          {
            "name": "stakeMint",
            "docs": [
              "Stake mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "pool",
            "docs": [
              "Pool"
            ],
            "type": "publicKey"
          },
          {
            "name": "stakeTokenVault",
            "docs": [
              "Stake token vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "tokenXVault",
            "docs": [
              "Token x vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "tokenYVault",
            "docs": [
              "Token y vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "topStakerList",
            "docs": [
              "Top staker list"
            ],
            "type": "publicKey"
          },
          {
            "name": "fullBalanceList",
            "docs": [
              "Full balance list"
            ],
            "type": "publicKey"
          },
          {
            "name": "metrics",
            "docs": [
              "Metrics"
            ],
            "type": {
              "defined": "Metrics"
            }
          },
          {
            "name": "configuration",
            "docs": [
              "Configuration parameters"
            ],
            "type": {
              "defined": "Configuration"
            }
          },
          {
            "name": "topStakerInfo",
            "docs": [
              "Top staker info"
            ],
            "type": {
              "defined": "TopStakerInfo"
            }
          },
          {
            "name": "creator",
            "docs": [
              "Creator"
            ],
            "type": "publicKey"
          },
          {
            "name": "createdAt",
            "docs": [
              "Created at"
            ],
            "type": "i64"
          },
          {
            "name": "bump",
            "docs": [
              "Bump"
            ],
            "type": "u8"
          },
          {
            "name": "padding0",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u8",
                7
              ]
            }
          },
          {
            "name": "padding",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u128",
                20
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitializeConfigParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "topListLength",
            "type": "u16"
          },
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "secondsToFullUnlock",
            "type": "u64"
          },
          {
            "name": "unstakeLockDuration",
            "type": "u64"
          },
          {
            "name": "joinWindowDuration",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "StakerBalance",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "balance",
            "docs": [
              "Balance"
            ],
            "type": "u64"
          },
          {
            "name": "owner",
            "docs": [
              "Owner pubkey, we dont need this for logic, but it is useful for indexing"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "StakerMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakeAmount",
            "docs": [
              "Staked amount"
            ],
            "type": "u64"
          },
          {
            "name": "fullBalanceIndex",
            "docs": [
              "Full balance list index. When it's negative, the slot is empty"
            ],
            "type": "i64"
          },
          {
            "name": "owner",
            "docs": [
              "Owner pubkey, we dont need this for logic, but it is usefull for indexing"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "Configuration",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "secondsToFullUnlock",
            "docs": [
              "Time required for locked claim fee to be fully dripped"
            ],
            "type": "u64"
          },
          {
            "name": "unstakeLockDuration",
            "docs": [
              "Unstake lock duration"
            ],
            "type": "u64"
          },
          {
            "name": "startClaimFeeTimestamp",
            "docs": [
              "Minimum time to start claim fee from lock escrow"
            ],
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "Metrics",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalStakedAmount",
            "docs": [
              "Total staked amount"
            ],
            "type": "u64"
          },
          {
            "name": "totalFeeXAmount",
            "docs": [
              "Fee x amount"
            ],
            "type": "u64"
          },
          {
            "name": "totalFeeYAmount",
            "docs": [
              "Fee y amount"
            ],
            "type": "u64"
          },
          {
            "name": "totalStakeEscrowCount",
            "docs": [
              "Total stake escrow count"
            ],
            "type": "u64"
          },
          {
            "name": "ongoingTotalPartialUnstakeAmount",
            "docs": [
              "Ongoing total partial unstake amount"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TopStakerInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "topListLength",
            "docs": [
              "Number of holder in the top list"
            ],
            "type": "u64"
          },
          {
            "name": "currentLength",
            "docs": [
              "Current length, used for resize"
            ],
            "type": "u64"
          },
          {
            "name": "effectiveStakeAmount",
            "docs": [
              "Effective stake amount. Total stake amount in the top list."
            ],
            "type": "u64"
          },
          {
            "name": "lastClaimFeeAt",
            "docs": [
              "Last claim fee at"
            ],
            "type": "i64"
          },
          {
            "name": "lastUpdatedAt",
            "docs": [
              "Last fee drip updated at"
            ],
            "type": "i64"
          },
          {
            "name": "lockedFeeX",
            "docs": [
              "Locked fee x"
            ],
            "type": "u64"
          },
          {
            "name": "lockedFeeY",
            "docs": [
              "Locked fee y"
            ],
            "type": "u64"
          },
          {
            "name": "padding",
            "docs": [
              "Padding"
            ],
            "type": "u64"
          },
          {
            "name": "cumulativeFeeXPerLiquidity",
            "docs": [
              "cumulative fee x per liquidity"
            ],
            "type": "u128"
          },
          {
            "name": "cumulativeFeeYPerLiquidity",
            "docs": [
              "cumulative fee y per liquidity"
            ],
            "type": "u128"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "VaultCreated",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenAMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenBMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "topListLength",
          "type": "u16",
          "index": false
        },
        {
          "name": "secondsToFullUnlock",
          "type": "u64",
          "index": false
        },
        {
          "name": "unstakeLockDuration",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "StakeEscrowCreated",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "escrow",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "ConfigCreated",
      "fields": [
        {
          "name": "config",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "topListLength",
          "type": "u16",
          "index": false
        },
        {
          "name": "secondsToFullUnlock",
          "type": "u64",
          "index": false
        },
        {
          "name": "unstakeLockDuration",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "ConfigClosed",
      "fields": [
        {
          "name": "config",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "UnstakeCreated",
      "fields": [
        {
          "name": "unstake",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowOngoingTotalUnstakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "startAt",
          "type": "i64",
          "index": false
        },
        {
          "name": "endAt",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "CancelUnstakeSucceed",
      "fields": [
        {
          "name": "unstake",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowOngoingTotalUnstakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "WithdrawSucceed",
      "fields": [
        {
          "name": "unstake",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowOngoingTotalUnstakeAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "ClaimFeeSucceed",
      "fields": [
        {
          "name": "stakeEscrow",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "feeXAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "totalFeeXAmount",
          "type": "u128",
          "index": false
        },
        {
          "name": "totalFeeYAmount",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "FeeEmission",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenXClaimed",
          "type": "u64",
          "index": false
        },
        {
          "name": "tokenYClaimed",
          "type": "u64",
          "index": false
        },
        {
          "name": "tokenXReleased",
          "type": "u64",
          "index": false
        },
        {
          "name": "tokenYReleased",
          "type": "u64",
          "index": false
        },
        {
          "name": "cumulativeFeeXPerLiquidity",
          "type": "u128",
          "index": false
        },
        {
          "name": "cumulativeFeeYPerLiquidity",
          "type": "u128",
          "index": false
        },
        {
          "name": "effectiveStakeAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "AddNewUserToTopHolder",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "RemoveUserFromTopHolder",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "UserStake",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "totalStakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidEscrowOwner",
      "msg": "Escrow owner is not vault"
    },
    {
      "code": 6001,
      "name": "InvalidTopListLength",
      "msg": "Invalid top list length"
    },
    {
      "code": 6002,
      "name": "InvalidSecondsToFullUnlock",
      "msg": "Invalid seconds to full unlock"
    },
    {
      "code": 6003,
      "name": "MustHaveQuoteToken",
      "msg": "Pool missing SOL/USDC token"
    },
    {
      "code": 6004,
      "name": "MissingDroppedStakeEscrow",
      "msg": "Missing dropped stake escrow"
    },
    {
      "code": 6005,
      "name": "InvalidStakeEscrow",
      "msg": "Invalid stake escrow"
    },
    {
      "code": 6006,
      "name": "FullBalanceListFull",
      "msg": "Full balance list is full"
    },
    {
      "code": 6007,
      "name": "InvalidStakeMint",
      "msg": "Invalid stake mint"
    },
    {
      "code": 6008,
      "name": "InsufficientStakeAmount",
      "msg": "Insufficient stake amount"
    },
    {
      "code": 6009,
      "name": "CannotWithdrawUnstakeAmount",
      "msg": "Unstake amount release date not reached"
    },
    {
      "code": 6010,
      "name": "InvalidAdmin",
      "msg": "Invalid admin"
    },
    {
      "code": 6011,
      "name": "InvalidUnstakeLockDuration",
      "msg": "Invalid unstake lock duration"
    },
    {
      "code": 6012,
      "name": "InvalidJoinWindowDuration",
      "msg": "Invalid join window duration"
    },
    {
      "code": 6013,
      "name": "InvalidCustomStartClaimFeeTimestamp",
      "msg": "Invalid custom start claim fee timestamp"
    },
    {
      "code": 6014,
      "name": "InvalidSmallestStakeEscrow",
      "msg": "Invalid smallest stake escrow"
    },
    {
      "code": 6015,
      "name": "MathOverflow",
      "msg": "MathOverflow"
    },
    {
      "code": 6016,
      "name": "TypeCastFailed",
      "msg": "Type casting failed"
    },
    {
      "code": 6017,
      "name": "InvalidLockEscrowRelatedAccounts",
      "msg": "Invalid lock escrow related accounts"
    },
    {
      "code": 6018,
      "name": "OnlyConstantProductPool",
      "msg": "Only constant product pool is supported"
    },
    {
      "code": 6019,
      "name": "UndeterminedError",
      "msg": "Undetermined error"
    }
  ]
};

export const IDL: StakeForFee = {
  "version": "0.1.0",
  "name": "stake_for_fee",
  "instructions": [
    {
      "name": "initializeVault",
      "accounts": [
        {
          "name": "config",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Config account"
          ]
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Vault account"
          ]
        },
        {
          "name": "stakeTokenVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Stake token vault"
          ]
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Token x vault"
          ]
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Token y vault"
          ]
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Top staker list account"
          ]
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false,
          "docs": [
            "Full balance list account"
          ]
        },
        {
          "name": "pool",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenAMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Token a mint of the pool"
          ]
        },
        {
          "name": "tokenBMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Token b mint of the pool"
          ]
        },
        {
          "name": "stakeMint",
          "isMut": false,
          "isSigner": false,
          "docs": [
            "Stake mint of the pool"
          ]
        },
        {
          "name": "lockEscrow",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "associatedTokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "customStartClaimFeeTimestamp",
          "type": {
            "option": "i64"
          }
        }
      ]
    },
    {
      "name": "initializeStakeEscrow",
      "accounts": [
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "stake",
      "accounts": [
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "smallestStakeEscrow",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "userStakeToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "amount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "claimFee",
      "accounts": [
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenX",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userTokenY",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": false,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "requestUnstake",
      "accounts": [
        {
          "name": "unstake",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "unstakeAmount",
          "type": "u64"
        }
      ]
    },
    {
      "name": "cancelUnstake",
      "accounts": [
        {
          "name": "unstake",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "smallestStakeEscrow",
          "isMut": true,
          "isSigner": false,
          "isOptional": true
        },
        {
          "name": "topStakerList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fullBalanceList",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenXVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "tokenYVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "pool",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "lockEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "escrowVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLp",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "aVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "bVaultLpMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "ammProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "vaultProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "withdraw",
      "accounts": [
        {
          "name": "unstake",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeEscrow",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "stakeTokenVault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "vault",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "userStakeToken",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "owner",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "tokenProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "dummy",
      "docs": [
        "To force IDL generation for some struct for easier TS decoding later"
      ],
      "accounts": [
        {
          "name": "stakerMetadata",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "stakerBalance",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initializeConfig",
      "docs": [
        "Start of admin only endpoints *"
      ],
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "params",
          "type": {
            "defined": "InitializeConfigParams"
          }
        }
      ]
    },
    {
      "name": "closeConfig",
      "accounts": [
        {
          "name": "config",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "admin",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "eventAuthority",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "program",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "stakerMetadataDummyAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakerMetadata",
            "type": {
              "defined": "StakerMetadata"
            }
          }
        ]
      }
    },
    {
      "name": "stakerBalanceDummyAccount",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakerBalance",
            "type": {
              "defined": "StakerBalance"
            }
          }
        ]
      }
    },
    {
      "name": "config",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "secondsToFullUnlock",
            "docs": [
              "Seconds for lock escrow claimed fee to be fully dripped to the top stakers"
            ],
            "type": "u64"
          },
          {
            "name": "unstakeLockDuration",
            "docs": [
              "Seconds for the requested unstake to withdraw the capital"
            ],
            "type": "u64"
          },
          {
            "name": "joinWindowDuration",
            "docs": [
              "Time window (in seconds) for staker to stake before the first lock escrow claim fee happen. This is to ensure that there's enough time for staker to join the list, so the first claim fee with huge amount will be more fairly distributed."
            ],
            "type": "u64"
          },
          {
            "name": "topListLength",
            "docs": [
              "Maximum number of stakers in the top list"
            ],
            "type": "u16"
          },
          {
            "name": "padding0",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u8",
                6
              ]
            }
          },
          {
            "name": "padding",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u128",
                20
              ]
            }
          }
        ]
      }
    },
    {
      "name": "fullBalanceListMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vault",
            "type": "publicKey"
          },
          {
            "name": "length",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "stakeEscrow",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "owner",
            "type": "publicKey"
          },
          {
            "name": "vault",
            "type": "publicKey"
          },
          {
            "name": "fullBalanceIndex",
            "type": "u64"
          },
          {
            "name": "stakeAmount",
            "type": "u64"
          },
          {
            "name": "inTopList",
            "type": "u8"
          },
          {
            "name": "padding0",
            "type": {
              "array": [
                "u8",
                15
              ]
            }
          },
          {
            "name": "ongoingTotalPartialUnstakeAmount",
            "type": "u64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "feeXClaimedAmount",
            "type": "u128"
          },
          {
            "name": "feeYClaimedAmount",
            "type": "u128"
          },
          {
            "name": "feeXPerLiquidityCheckpoint",
            "type": "u128"
          },
          {
            "name": "feeYPerLiquidityCheckpoint",
            "type": "u128"
          },
          {
            "name": "feeXPending",
            "type": "u64"
          },
          {
            "name": "feeYPending",
            "type": "u64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u128",
                20
              ]
            }
          }
        ]
      }
    },
    {
      "name": "topListMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "vault",
            "docs": [
              "Vault"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "unstake",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakeEscrow",
            "type": "publicKey"
          },
          {
            "name": "unstakeAmount",
            "type": "u64"
          },
          {
            "name": "createdAt",
            "type": "i64"
          },
          {
            "name": "releaseAt",
            "type": "i64"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u64",
                30
              ]
            }
          }
        ]
      }
    },
    {
      "name": "feeVault",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "lockEscrow",
            "docs": [
              "Lock escrow account"
            ],
            "type": "publicKey"
          },
          {
            "name": "stakeMint",
            "docs": [
              "Stake mint"
            ],
            "type": "publicKey"
          },
          {
            "name": "pool",
            "docs": [
              "Pool"
            ],
            "type": "publicKey"
          },
          {
            "name": "stakeTokenVault",
            "docs": [
              "Stake token vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "tokenXVault",
            "docs": [
              "Token x vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "tokenYVault",
            "docs": [
              "Token y vault"
            ],
            "type": "publicKey"
          },
          {
            "name": "topStakerList",
            "docs": [
              "Top staker list"
            ],
            "type": "publicKey"
          },
          {
            "name": "fullBalanceList",
            "docs": [
              "Full balance list"
            ],
            "type": "publicKey"
          },
          {
            "name": "metrics",
            "docs": [
              "Metrics"
            ],
            "type": {
              "defined": "Metrics"
            }
          },
          {
            "name": "configuration",
            "docs": [
              "Configuration parameters"
            ],
            "type": {
              "defined": "Configuration"
            }
          },
          {
            "name": "topStakerInfo",
            "docs": [
              "Top staker info"
            ],
            "type": {
              "defined": "TopStakerInfo"
            }
          },
          {
            "name": "creator",
            "docs": [
              "Creator"
            ],
            "type": "publicKey"
          },
          {
            "name": "createdAt",
            "docs": [
              "Created at"
            ],
            "type": "i64"
          },
          {
            "name": "bump",
            "docs": [
              "Bump"
            ],
            "type": "u8"
          },
          {
            "name": "padding0",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u8",
                7
              ]
            }
          },
          {
            "name": "padding",
            "docs": [
              "Padding"
            ],
            "type": {
              "array": [
                "u128",
                20
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitializeConfigParams",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "topListLength",
            "type": "u16"
          },
          {
            "name": "index",
            "type": "u64"
          },
          {
            "name": "secondsToFullUnlock",
            "type": "u64"
          },
          {
            "name": "unstakeLockDuration",
            "type": "u64"
          },
          {
            "name": "joinWindowDuration",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "StakerBalance",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "balance",
            "docs": [
              "Balance"
            ],
            "type": "u64"
          },
          {
            "name": "owner",
            "docs": [
              "Owner pubkey, we dont need this for logic, but it is useful for indexing"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "StakerMetadata",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "stakeAmount",
            "docs": [
              "Staked amount"
            ],
            "type": "u64"
          },
          {
            "name": "fullBalanceIndex",
            "docs": [
              "Full balance list index. When it's negative, the slot is empty"
            ],
            "type": "i64"
          },
          {
            "name": "owner",
            "docs": [
              "Owner pubkey, we dont need this for logic, but it is usefull for indexing"
            ],
            "type": "publicKey"
          }
        ]
      }
    },
    {
      "name": "Configuration",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "secondsToFullUnlock",
            "docs": [
              "Time required for locked claim fee to be fully dripped"
            ],
            "type": "u64"
          },
          {
            "name": "unstakeLockDuration",
            "docs": [
              "Unstake lock duration"
            ],
            "type": "u64"
          },
          {
            "name": "startClaimFeeTimestamp",
            "docs": [
              "Minimum time to start claim fee from lock escrow"
            ],
            "type": "i64"
          }
        ]
      }
    },
    {
      "name": "Metrics",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "totalStakedAmount",
            "docs": [
              "Total staked amount"
            ],
            "type": "u64"
          },
          {
            "name": "totalFeeXAmount",
            "docs": [
              "Fee x amount"
            ],
            "type": "u64"
          },
          {
            "name": "totalFeeYAmount",
            "docs": [
              "Fee y amount"
            ],
            "type": "u64"
          },
          {
            "name": "totalStakeEscrowCount",
            "docs": [
              "Total stake escrow count"
            ],
            "type": "u64"
          },
          {
            "name": "ongoingTotalPartialUnstakeAmount",
            "docs": [
              "Ongoing total partial unstake amount"
            ],
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "TopStakerInfo",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "topListLength",
            "docs": [
              "Number of holder in the top list"
            ],
            "type": "u64"
          },
          {
            "name": "currentLength",
            "docs": [
              "Current length, used for resize"
            ],
            "type": "u64"
          },
          {
            "name": "effectiveStakeAmount",
            "docs": [
              "Effective stake amount. Total stake amount in the top list."
            ],
            "type": "u64"
          },
          {
            "name": "lastClaimFeeAt",
            "docs": [
              "Last claim fee at"
            ],
            "type": "i64"
          },
          {
            "name": "lastUpdatedAt",
            "docs": [
              "Last fee drip updated at"
            ],
            "type": "i64"
          },
          {
            "name": "lockedFeeX",
            "docs": [
              "Locked fee x"
            ],
            "type": "u64"
          },
          {
            "name": "lockedFeeY",
            "docs": [
              "Locked fee y"
            ],
            "type": "u64"
          },
          {
            "name": "padding",
            "docs": [
              "Padding"
            ],
            "type": "u64"
          },
          {
            "name": "cumulativeFeeXPerLiquidity",
            "docs": [
              "cumulative fee x per liquidity"
            ],
            "type": "u128"
          },
          {
            "name": "cumulativeFeeYPerLiquidity",
            "docs": [
              "cumulative fee y per liquidity"
            ],
            "type": "u128"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "VaultCreated",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenAMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenBMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeMint",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "creator",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "topListLength",
          "type": "u16",
          "index": false
        },
        {
          "name": "secondsToFullUnlock",
          "type": "u64",
          "index": false
        },
        {
          "name": "unstakeLockDuration",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "StakeEscrowCreated",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "escrow",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "ConfigCreated",
      "fields": [
        {
          "name": "config",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "topListLength",
          "type": "u16",
          "index": false
        },
        {
          "name": "secondsToFullUnlock",
          "type": "u64",
          "index": false
        },
        {
          "name": "unstakeLockDuration",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "ConfigClosed",
      "fields": [
        {
          "name": "config",
          "type": "publicKey",
          "index": false
        }
      ]
    },
    {
      "name": "UnstakeCreated",
      "fields": [
        {
          "name": "unstake",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowOngoingTotalUnstakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "startAt",
          "type": "i64",
          "index": false
        },
        {
          "name": "endAt",
          "type": "i64",
          "index": false
        }
      ]
    },
    {
      "name": "CancelUnstakeSucceed",
      "fields": [
        {
          "name": "unstake",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowOngoingTotalUnstakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "WithdrawSucceed",
      "fields": [
        {
          "name": "unstake",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "amount",
          "type": "u64",
          "index": false
        },
        {
          "name": "newStakeEscrowOngoingTotalUnstakeAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "ClaimFeeSucceed",
      "fields": [
        {
          "name": "stakeEscrow",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "feeXAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "totalFeeXAmount",
          "type": "u128",
          "index": false
        },
        {
          "name": "totalFeeYAmount",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "FeeEmission",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "tokenXClaimed",
          "type": "u64",
          "index": false
        },
        {
          "name": "tokenYClaimed",
          "type": "u64",
          "index": false
        },
        {
          "name": "tokenXReleased",
          "type": "u64",
          "index": false
        },
        {
          "name": "tokenYReleased",
          "type": "u64",
          "index": false
        },
        {
          "name": "cumulativeFeeXPerLiquidity",
          "type": "u128",
          "index": false
        },
        {
          "name": "cumulativeFeeYPerLiquidity",
          "type": "u128",
          "index": false
        },
        {
          "name": "effectiveStakeAmount",
          "type": "u64",
          "index": false
        }
      ]
    },
    {
      "name": "AddNewUserToTopHolder",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "RemoveUserFromTopHolder",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    },
    {
      "name": "UserStake",
      "fields": [
        {
          "name": "pool",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "vault",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "owner",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "stakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "totalStakeAmount",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeYPending",
          "type": "u64",
          "index": false
        },
        {
          "name": "feeXPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        },
        {
          "name": "feeYPerLiquidityCheckpoint",
          "type": "u128",
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidEscrowOwner",
      "msg": "Escrow owner is not vault"
    },
    {
      "code": 6001,
      "name": "InvalidTopListLength",
      "msg": "Invalid top list length"
    },
    {
      "code": 6002,
      "name": "InvalidSecondsToFullUnlock",
      "msg": "Invalid seconds to full unlock"
    },
    {
      "code": 6003,
      "name": "MustHaveQuoteToken",
      "msg": "Pool missing SOL/USDC token"
    },
    {
      "code": 6004,
      "name": "MissingDroppedStakeEscrow",
      "msg": "Missing dropped stake escrow"
    },
    {
      "code": 6005,
      "name": "InvalidStakeEscrow",
      "msg": "Invalid stake escrow"
    },
    {
      "code": 6006,
      "name": "FullBalanceListFull",
      "msg": "Full balance list is full"
    },
    {
      "code": 6007,
      "name": "InvalidStakeMint",
      "msg": "Invalid stake mint"
    },
    {
      "code": 6008,
      "name": "InsufficientStakeAmount",
      "msg": "Insufficient stake amount"
    },
    {
      "code": 6009,
      "name": "CannotWithdrawUnstakeAmount",
      "msg": "Unstake amount release date not reached"
    },
    {
      "code": 6010,
      "name": "InvalidAdmin",
      "msg": "Invalid admin"
    },
    {
      "code": 6011,
      "name": "InvalidUnstakeLockDuration",
      "msg": "Invalid unstake lock duration"
    },
    {
      "code": 6012,
      "name": "InvalidJoinWindowDuration",
      "msg": "Invalid join window duration"
    },
    {
      "code": 6013,
      "name": "InvalidCustomStartClaimFeeTimestamp",
      "msg": "Invalid custom start claim fee timestamp"
    },
    {
      "code": 6014,
      "name": "InvalidSmallestStakeEscrow",
      "msg": "Invalid smallest stake escrow"
    },
    {
      "code": 6015,
      "name": "MathOverflow",
      "msg": "MathOverflow"
    },
    {
      "code": 6016,
      "name": "TypeCastFailed",
      "msg": "Type casting failed"
    },
    {
      "code": 6017,
      "name": "InvalidLockEscrowRelatedAccounts",
      "msg": "Invalid lock escrow related accounts"
    },
    {
      "code": 6018,
      "name": "OnlyConstantProductPool",
      "msg": "Only constant product pool is supported"
    },
    {
      "code": 6019,
      "name": "UndeterminedError",
      "msg": "Undetermined error"
    }
  ]
};
