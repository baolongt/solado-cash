use anchor_lang::prelude::*;
use solana_merkle_tree::MerkleTree;

#[account]
#[derive(InitSpace)]
pub struct DepositInfo {
    pub note: Pubkey,

    pub pool_token: Pubkey,

    pub amount: f64,
}

#[account]
pub struct MerkleTreeInfo {
    pub merkle_tree: MerkleTree,
}
