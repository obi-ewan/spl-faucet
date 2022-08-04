import * as anchor from "@project-serum/anchor";
import { Program } from "@project-serum/anchor";
import { BetdexSplFaucet } from "../target/types/betdex_spl_faucet";

describe("betdex-spl-faucet", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.BetdexSplFaucet as Program<BetdexSplFaucet>;

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().rpc();
    console.log("Your transaction signature", tx);
  });
});
