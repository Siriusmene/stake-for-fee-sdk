import { ComputeBudgetProgram } from "@solana/web3.js";

export const computeUnitIx = (units?: number) => {
  return ComputeBudgetProgram.setComputeUnitLimit({
    units: units ?? 1_400_000,
  });
};
