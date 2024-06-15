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
import { BN } from "bn.js";

describe("solado-cash", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SoladoCash as Program<SoladoCash>;
  const usdcMintKp = anchor.web3.Keypair.generate();
  console.log("USDC mint pubkey", usdcMintKp.publicKey.toBase58());

  const user = loadKeypairFromFile();
  let userTokenAccount: anchor.web3.PublicKey;
  let mmrAccount: anchor.web3.PublicKey;

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

      const createUserTokenAccountIx = createAssociatedTokenAccountInstruction(
        user.publicKey,
        userTokenAccount,
        user.publicKey,
        usdcMintKp.publicKey
      );


      const mintToUserIx = createMintToInstruction(
        usdcMintKp.publicKey,
        userTokenAccount,
        provider.publicKey,
        100000 * 10 ** 6,
        []
      );


      tx.add(
        ...[
          createMintIx,
          initMintIx,
          createUserTokenAccountIx,
          mintToUserIx,
        ]
      );

      console.log("Creating USDC mint...")

      const ts = await provider.sendAndConfirm(tx, [usdcMintKp, user]);
      console.log("send and mint tx", ts);

    }




    mmrAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("pool_vault"),
        Buffer.from("1000_USDC"),
        usdcMintKp.publicKey.toBytes(),
      ],
      program.programId
    )[0];

    console.log("mmmAccount", mmrAccount.toBase58());

  })

  it("Is initialized!", async () => {
    mmrAccount = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("pool_vault"),
        Buffer.from("1000_USDC"),
        usdcMintKp.publicKey.toBytes(),
      ],
      program.programId
    )[0];

    const poolTokenAccount = getAssociatedTokenAddressSync(
      usdcMintKp.publicKey,
      mmrAccount,
      true
    );
    // Add your test here.
    const tx = await program.methods.initialize().accounts({
      admin: provider.publicKey,
      poolToken: usdcMintKp.publicKey,
      mmrAccount: mmrAccount,
      poolTokenAccount: poolTokenAccount,
      systemProgram: anchor.web3.SystemProgram.programId,
      tokenProgram: TOKEN_PROGRAM_ID,
      associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
    })
      .rpc();

    console.log("init tx", tx);
    const merkleTreeAccount = await program.account.merkleMountainRange.fetch(mmrAccount);

    expect(merkleTreeAccount.nodes).to.eql([]);
    expect(merkleTreeAccount.peaks).to.eql([]);
  });

  it("Should deposit successfully", async () => {
    const poolTokenAccount = getAssociatedTokenAddressSync(
      usdcMintKp.publicKey,
      mmrAccount,
      true
    );
    const depositAmount = new BN(1000 * 10 ** 6);
    const txs = []

    for (let i = 0; i < 10; i++) {

      const tx = await program.rpc.deposit(
        depositAmount,
        {
          accounts: {
            user: user.publicKey,
            poolToken: usdcMintKp.publicKey,
            userTokenAccount: userTokenAccount,
            mmrAccount: mmrAccount,
            poolTokenAccount: poolTokenAccount,
            systemProgram: anchor.web3.SystemProgram.programId,
            tokenProgram: TOKEN_PROGRAM_ID,
            associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
          },
          signers: [user]
        }
      );

      txs.push(tx);
    }

    console.table(txs);

  });
});
