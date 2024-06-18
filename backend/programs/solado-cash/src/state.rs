use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use crate::utils::{ hash_pair, zero };
use bs58;

#[account]
#[derive(InitSpace)]
pub struct DepositInfo {
    pub note: Pubkey,

    pub pool_token: Pubkey,

    pub amount: f64,
}

#[account]
#[derive(InitSpace)]
pub struct MerkleTree {
    pub levels: u8, // Number of levels in the tree
    #[max_len(256, 32)]
    pub nodes: Vec<[u8; 32]>, // Nodes of the Merkle tree
    pub root: [u8; 32], // Current root of the Merkle tree
    pub current_leaf_index: u64, // Index for the next deposit
}

#[account]
#[derive(InitSpace)]
pub struct Note {
    #[max_len(1000)]
    pub proof: String,
    pub owner: Pubkey, // Account that can withdraw
    pub root: [u8; 32], // Root of the Merkle tree
}

impl MerkleTree {
    pub fn insert(&mut self, leaf: [u8; 32]) -> ProgramResult {
        let leaf_index = self.current_leaf_index;
        if leaf_index >= (2u64).pow(self.levels as u32) {
            // Check if the tree is full (1-indexed)
            return Err(ProgramError::InvalidArgument);
        }

        // 1. Store the new leaf
        self.nodes[leaf_index as usize] = leaf; // Store at the current leaf index

        // 2. Update hash (main logic)
        let mut current_index = leaf_index;
        for current_level in 0..self.levels {
            // Iterate directly over levels
            if current_index == 0 {
                break; // Root reached
            }

            let (left_index, right_index) = if current_index % 2 == 0 {
                (current_index as usize, (current_index + 1) as usize) // Leaf is left child
            } else {
                ((current_index - 1) as usize, current_index as usize) // Leaf is right child
            };

            let right = if (right_index as u64) < (self.nodes.len() as u64) {
                self.nodes[right_index]
            } else {
                zero(current_level.into()) // Directly call zero function
            };

            self.nodes[((current_index - 1) / 2) as usize] = hash_pair(
                &self.nodes[left_index],
                &right
            );
            current_index = (current_index - 1) / 2;
        }

        // 3. Update root and leaf index
        self.root = self.nodes[0];
        self.current_leaf_index += 1;

        Ok(())
    }

    pub fn generate_proof(&self, leaf_index: u64) -> String {
        let mut proof_str = String::new();
        let mut current_index = leaf_index + (2u64).pow(self.levels as u32) - 1; // Leaf node index

        while current_index > 0 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };
            if sibling_index < (self.nodes.len() as u64) {
                proof_str += bs58
                    ::encode(self.nodes[sibling_index as usize])
                    .into_string()
                    .as_str();
            } else {
                proof_str += bs58
                    ::encode([0u8; 32])
                    .into_string()
                    .as_str();
            }
            current_index = (current_index - 1) / 2; // Move up to the parent
        }

        proof_str
    }
}
