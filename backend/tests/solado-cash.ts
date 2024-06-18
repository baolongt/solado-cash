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
import { hashString } from "./lib/hash";
import * as bs58 from 'bs58';

describe("solado-cash", () => {
  // Configure the client to use the local cluster.
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.SoladoCash as Program<SoladoCash>;
  const usdcMintKp = anchor.web3.Keypair.generate();
  console.log("USDC mint pubkey", usdcMintKp.publicKey.toBase58());

  const user = loadKeypairFromFile();
  let userTokenAccount: anchor.web3.PublicKey;
  let mtAccoountPubKey: anchor.web3.PublicKey;

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

    mtAccoountPubKey = anchor.web3.PublicKey.findProgramAddressSync(
      [
        Buffer.from("pool_vault"),
        Buffer.from("1000_USDC"),
        usdcMintKp.publicKey.toBytes(),
      ],
      program.programId
    )[0];

    console.log("mmmAccount", mtAccoountPubKey.toBase58());

  })

  it("Is initialized!", async () => {
    const poolTokenAccount = getAssociatedTokenAddressSync(
      usdcMintKp.publicKey,
      mtAccoountPubKey,
      true
    );

    try {
      // Add your test here.
      const tx = await program.methods.initialize().accounts({
        admin: provider.publicKey,
        poolToken: usdcMintKp.publicKey,
        merkleTreeAccount: mtAccoountPubKey,
        poolTokenAccount: poolTokenAccount,
        systemProgram: anchor.web3.SystemProgram.programId,
        tokenProgram: TOKEN_PROGRAM_ID,
        associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
      })
        .rpc();
      console.log("init tx", tx);

    }
    catch (e) {
      console.log("error", e);
    }

    const merkleTreeAccount = await program.account.merkleTree.fetch(mtAccoountPubKey);

    expect(merkleTreeAccount.levels).to.equal(8);
    expect(merkleTreeAccount.currentLeafIndex.toNumber()).to.equal(1);

  });

  it("Should deposit successfully", async () => {
    const poolTokenAccount = getAssociatedTokenAddressSync(
      usdcMintKp.publicKey,
      mtAccoountPubKey,
      true
    );
    const txs = []
    const proofs = {};


    for (let i = 0; i < 20; i++) {
      try {
        console.log("depositing", i);
        const merkleTreeAccount = await program.account.merkleTree.fetch(mtAccoountPubKey);

        const noteAccountPubKey = anchor.web3.PublicKey.findProgramAddressSync(
          [
            Buffer.from("note"),
            user.publicKey.toBytes(),
            mtAccoountPubKey.toBytes(),
            Buffer.from(merkleTreeAccount.currentLeafIndex.toArray("le", 8))
          ],
          program.programId
        )[0];
        console.log("noteAccountPubKey", noteAccountPubKey.toBase58());
        const depositAmount = new BN(1000 * 10 ** 6);

        const tx = await program.methods.deposit(depositAmount).accounts({
          userTokenAccount: userTokenAccount,
          noteAccount: noteAccountPubKey,
          user: user.publicKey,
          merkleTreeAccount: mtAccoountPubKey,
          poolTokenAccount: poolTokenAccount,
          poolToken: usdcMintKp.publicKey,
          tokenProgram: TOKEN_PROGRAM_ID,
          associatedTokenProgram: ASSOCIATED_TOKEN_PROGRAM_ID,
        })
          .signers([user])
          .rpc();


        txs.push(tx);


        const noteAccount = await program.account.note.fetch(noteAccountPubKey);
        proofs[noteAccountPubKey.toBase58()] = {
          proof: noteAccount.proof,
          root: merkleTreeAccount.root,
        };


        expect(noteAccount.proof).to.be.not.null;
      } catch (e) {
        console.log("error", e);

      }
    }

    console.table(txs);
    console.table(proofs);
  });


});
