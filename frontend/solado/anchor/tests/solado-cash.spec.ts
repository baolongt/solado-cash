import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SoladoCash } from "../target/types/solado_cash";

describe("solado-cash", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.SoladoCash as Program<SoladoCash>;

  it("should run the program", async () => {
    // Add your test here.
    const tx = await program.methods.greet().rpc();
    console.log("Your transaction signature", tx);
  });
});
