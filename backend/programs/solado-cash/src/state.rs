use anchor_lang::prelude::*;
use solana_program::entrypoint::ProgramResult;
use crate::{ errors::AppError, utils::{ hash_pair, zero } };
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
    pub leaf: [u8; 32], // Leaf of the Merkle tree
}

impl MerkleTree {
    // pub fn insert(&mut self, leaf: [u8; 32]) -> ProgramResult {
    //     let leaf_index = self.current_leaf_index;
    //     if leaf_index >= (2u64).pow(self.levels as u32) {
    //         // Check if the tree is full (1-indexed)
    //         return Err(ProgramError::InvalidArgument);
    //     }

    //     // 1. Store the new leaf
    //     self.nodes[leaf_index as usize] = leaf; // Store at the current leaf index

    //     // 2. Update hash (main logic)
    //     let mut current_index = leaf_index;
    //     for current_level in 0..self.levels {
    //         // Iterate directly over levels
    //         if current_index == 0 {
    //             break; // Root reached
    //         }

    //         let (left_index, right_index) = if current_index % 2 == 0 {
    //             (current_index as usize, (current_index + 1) as usize) // Leaf is left child
    //         } else {
    //             ((current_index - 1) as usize, current_index as usize) // Leaf is right child
    //         };

    //         let right = if (right_index as u64) < (self.nodes.len() as u64) {
    //             self.nodes[right_index]
    //         } else {
    //             zero(current_level.into()) // Directly call zero function
    //         };

    //         self.nodes[((current_index - 1) / 2) as usize] = hash_pair(
    //             &self.nodes[left_index],
    //             &right
    //         );

    //         msg!("inserted: {:?}", self.nodes[((current_index - 1) / 2) as usize]);
    //         current_index = (current_index - 1) / 2;
    //     }

    //     // 3. Update root and leaf index
    //     self.root = self.nodes[0];
    //     self.current_leaf_index += 1;

    //     Ok(())
    // }

    pub fn insert(&mut self, leaf: [u8; 32]) -> ProgramResult {
        let leaf_index = self.current_leaf_index;
        if leaf_index >= (2u64).pow(self.levels as u32) {
            return Err(ProgramError::InvalidArgument);
        }

        // 1. Store the new leaf
        self.nodes[leaf_index as usize] = leaf;

        // 2. Update hashes
        let mut current_index = leaf_index;
        let mut computed_hash = leaf;
        for _ in 0..self.levels {
            if current_index == 0 {
                self.nodes[0] = hash_pair(&self.nodes[1], &self.nodes[2]);
                break;
            }
            if current_index % 2 == 0 {
                // Leaf is left child
                let right_index = current_index + 1;
                let right = if (right_index as u64) < (self.nodes.len() as u64) {
                    self.nodes[right_index as usize]
                } else {
                    zero((self.levels - 1).into()) // Use the appropriate zero value for the hash function
                };

                self.nodes[(current_index as usize) / 2] = hash_pair(&computed_hash, &right);
                computed_hash = self.nodes[(current_index as usize) / 2];
            } else {
                // Leaf is right child
                let left_index = current_index - 1;
                if current_index > 2 {
                    self.nodes[(current_index as usize) / 2] = hash_pair(
                        &self.nodes[left_index as usize],
                        &computed_hash
                    );
                    computed_hash = self.nodes[(current_index as usize) / 2];
                }
            }

            current_index /= 2;
        }

        // 3. Update root and leaf index
        self.root = self.nodes[0];
        self.current_leaf_index += 1;

        Ok(())
    }

    pub fn generate_proof(&self, leaf_index: u64) -> String {
        let mut proof_str = String::new();
        let mut current_index = leaf_index; // Leaf node index

        while current_index > 0 {
            let sibling_index = if current_index % 2 == 0 {
                current_index + 1
            } else {
                current_index - 1
            };

            msg!("sibling_index: {:?}", sibling_index);

            if sibling_index == 0 {
                proof_str += &bs58::encode(self.nodes[2]).into_string();
                break;
            } else if sibling_index == 1 {
                proof_str += &bs58::encode(self.nodes[1]).into_string();
                break;
            }

            if sibling_index < (self.nodes.len() as u64) {
                let hash = bs58::encode(self.nodes[sibling_index as usize]);
                let hash_str = hash.into_string();
                proof_str += &hash_str;
            } else {
                let hash = bs58::encode([0u8; 44]);
                let hash_str = hash.into_string();

                proof_str += &hash_str;
            }
            current_index = current_index / 2; // Move up to the parent
        }

        proof_str
    }

    pub fn decode_proof(&self, proof_str: &str) -> Vec<[u8; 32]> {
        let mut hashes = Vec::new();

        // Each Base58-encoded hash is 44 characters long
        for chunk in proof_str.as_bytes().chunks(44) {
            let hash_str = std::str::from_utf8(chunk).expect("Invalid UTF-8");
            let hash = bs58::decode(hash_str).into_vec().expect("Invalid Base58 string");

            let mut hash_array = [0u8; 32];
            hash_array.copy_from_slice(&hash[0..32]);

            hashes.push(hash_array);
        }

        hashes
    }
}
