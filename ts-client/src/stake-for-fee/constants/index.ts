import { PublicKey } from "@solana/web3.js";
import { BN } from "bn.js";

export const MAX_COMPUTE_UNITS = 1_400_000;

export const STAKE_FOR_FEE_PROGRAM_ID = new PublicKey(
  "FEESngU3neckdwib9X3KWqdL7Mjmqk9XNp3uh5JbP4KP"
);

export const DYNAMIC_AMM_PROGRAM_ID = new PublicKey(
  "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"
);

export const DYNAMIC_VAULT_PROGRAM_ID = new PublicKey(
  "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"
);

export const FULL_BALANCE_LIST_HARD_LIMIT = new BN(10000);
export const U64_MAX = new BN("18446744073709551615");

export const VAULT_WITH_NON_PDA_BASED_LP_MINTS: Map<string, PublicKey> =
  new Map([
    [
      "BFJP6RYDxJa4FmFtBpPDYcrPozjC98CELrXqVL7rGMVW",
      new PublicKey("5CuhvouXVx6t5XPiyhRkrfgK5omAf8XnqY1ef6CLjw7o"),
    ],
    [
      "AzrUPWWyT9ZoAuMTgGHxYCnnWD2veh98FsCcMknVjg3Q",
      new PublicKey("9MSsSzDKq8VzokicRom6ciYPLhhZf65bCCBQLjnC7jUH"),
    ],
    [
      "GGQfASSnFaqPu83jWrL1DMJBJEzG3rdwsDARDGt6Gxmj",
      new PublicKey("4da9saTYgDs37wRSuS8mnFoiWzSYeRtvSWaFRe8rtkFc"),
    ],
    [
      "GofttAxULhp5NE9faWNngsnDM1iJiL75AJ2AkSaaC2CC",
      new PublicKey("Bma9RZx1AjNGcojNJpstGe9Wcytxz17YA6rd2Lq1UirT"),
    ],
    [
      "671JaLe2zDgBeXK3UtFHBiid7WFCHAKZTmLqAaQxx7cL",
      new PublicKey("9NywobBSCyntrPSZxEZpUbJXLfgUzKbUF2ZqBBkJLEgB"),
    ],
    [
      "2dH3aSpt5aEwhoeSaThKRNtNppEpg2DhGKGa1C5Wecc1",
      new PublicKey("Afe5fiLmbKw7aBi1VgWZb9hEY8nRYtib6LNr5RGUJibP"),
    ],
    [
      "BVJACEffKRHvKbQT9VfEqoxrUWJN2UVdonTKYB2c4MgK",
      new PublicKey("FFmYsMk5xQq3zQf1r4A6Yyf3kaKd3LUQokeVa776rKWH"),
    ],
    [
      "5XCP3oD3JAuQyDpfBFFVUxsBxNjPQojpKuL4aVhHsDok",
      new PublicKey("EZun6G5514FeqYtUv26cBHWLqXjAEdjGuoX6ThBpBtKj"),
    ],
    [
      "mPWBpKzzchEjitz7x4Q2d7cbQ3fHibF2BHWbWk8YGnH",
      new PublicKey("4nCGSVN8ZGuewX36TznzisceaNYzURWPesxyGtDvA2iP"),
    ],
    [
      "8p1VKP45hhqq5iZG5fNGoi7ucme8nFLeChoDWNy7rWFm",
      new PublicKey("28KR3goEditLnzBZShRk2H7xvgzc176EoFwMogjdfSkn"),
    ],
    [
      "FERjPVNEa7Udq8CEv68h6tPL46Tq7ieE49HrE2wea3XT",
      new PublicKey("FZN7QZ8ZUUAxMPfxYEYkH3cXUASzH8EqA6B4tyCL8f1j"),
    ],
    [
      "3ESUFCnRNgZ7Mn2mPPUMmXYaKU8jpnV9VtA17M7t2mHQ",
      new PublicKey("3RpEekjLE5cdcG15YcXJUpxSepemvq2FpmMcgo342BwC"),
    ],
  ]);
