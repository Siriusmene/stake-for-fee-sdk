import {
  Connection,
  Keypair,
  PublicKey,
  sendAndConfirmTransaction,
} from "@solana/web3.js";
import { StakeForFee } from "./stake-for-fee";

const connection = new Connection("https://api.devnet.solana.com");

async function test() {
  try {
    const feeFarm = await StakeForFee.create(
      connection,
      new PublicKey("FX8rBiRLHKoSzGM8GZbeMxMiVdNLiX4wfuR6BWLSaf1F")
    );
    console.log("🚀 ~ test ~ txId:", feeFarm);
  } catch (error) {
    console.log("🚀 ~ test ~ error:", JSON.parse(JSON.stringify(error)));
    console.log("🚀 ~ test ~ error:", error);
  }
}

test();
