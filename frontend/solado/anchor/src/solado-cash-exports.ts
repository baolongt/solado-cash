// Here we export some useful types and functions for interacting with the Anchor program.
import { AnchorProvider, Program } from '@coral-xyz/anchor';
import { PublicKey } from '@solana/web3.js';
import type { SoladoCash } from '../target/types/solado_cash';
import { IDL as SoladoCashIDL } from '../target/types/solado_cash';

// Re-export the generated IDL and type
export { SoladoCash, SoladoCashIDL };

// The programId is imported from the program IDL.
export const SOLADO_CASH_PROGRAM_ID = new PublicKey(SoladoCashIDL.address)

// This is a helper function to get the SoladoCash Anchor program.
export function getSoladoCashProgram(provider: AnchorProvider) {
  return new Program(SoladoCashIDL as SoladoCash, provider);
}
