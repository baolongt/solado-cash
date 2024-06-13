import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { SoladoCash } from "../target/types/solado_cash";
import {
  createAssociatedTokenAccountInstruction,
  getAssociatedTokenAddressSync,
  createInitializeMint2Instruction,
  getMinimumBalanceForRentExemptMint,
  TOKEN_PROGRAM_ID,
  MINT_SIZE,
  ASSOCIATED_TOKEN_PROGRAM_ID,
  createMintToInstruction,
  getAccount,
} from "@solana/spl-token";
import { loadKeypairFromFile } from "./lib/helper";
import { expect } from "chai";
describe("solado-cash", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SoladoCash as Program<SoladoCash>;
  const usdcMintKp = anchor.web3.Keypair.generate();
  console.log("USDC mint pubkey", usdcMintKp.publicKey.toBase58());

  const user = loadKeypairFromFile();
  let userTokenAccount: anchor.web3.PublicKey;
  let poolVault: anchor.web3.PublicKey;

  before(async () => {
    // init staker
    // create USDC-fake mint
    {
      const tx = new anchor.web3.Transaction();

      const lamports = await getMinimumBalanceForRentExemptMint(
        provider.connection
      );

      const createMintIx = anchor.web3.SystemProgram.createAccount({
        fromPubkey: provider.publicKey,
        newAccountPubkey: usdcMintKp.publicKey,
        space: MINT_SIZE,
        lamports,
        programId: TOKEN_PROGRAM_ID,
      });

      const initMintIx = createInitializeMint2Instruction(
        usdcMintKp.publicKey,
        6,
        provider.publicKey,
        provider.publicKey,
        TOKEN_PROGRAM_ID
      );


      userTokenAccount = getAssociatedTokenAddressSync(
        usdcMintKp.publicKey,
        user.publicKey
      );

      const createStakerTokenAccountIx =
        createAssociatedTokenAccountInstruction(
          user.publicKey,
          userTokenAccount,
          user.publicKey,
          usdcMintKp.publicKey
        );

      console.log("user token account", userTokenAccount.toBase58());


      const mintToUserIx = createMintToInstruction(
        usdcMintKp.publicKey,
        userTokenAccount,
        provider.publicKey,
        1000 * 10 ** 6,
        []
      );


      tx.add(
        ...[
          createMintIx,
          initMintIx,
          createStakerTokenAccountIx,
          mintToUserIx,
        ]
      );

      console.log("Creating USDC mint...")

      const ts = await provider.sendAndConfirm(tx, [usdcMintKp, user]);
      console.log("send and mint tx", ts);

    }


    poolVault = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("pool_vault"),
        Buffer.from("1000_USDC"),
        usdcMintKp.publicKey.toBytes()
      ],
      program.programId
    )[0];

    console.log("Pool vault", poolVault.toBase58());

  })

  it("Is initialized!", async () => {
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      admin: provider.publicKey,
      poolToken: usdcMintKp.publicKey,
      poolVault: poolVault,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
    })
      .rpc();

    console.log("init tx", tx);
    const poolAccount = await getAccount(provider.connection, poolVault);
    console.log("pool account address", poolAccount);

    expect(poolAccount.address.toBase58()).to.equal(
      poolVault.toBase58()
    );
    expect(poolAccount.mint.toBase58()).to.equal(
      usdcMintKp.publicKey.toBase58()
    );
    expect(poolAccount.amount).to.equal(BigInt(0));
  });
});
