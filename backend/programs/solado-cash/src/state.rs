use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct DepositInfo {
    pub note: Pubkey,

    pub pool_token: Pubkey,

    pub amount: f64,
}

#[account]
#[derive(InitSpace)]
pub struct MerkleMountainRange {
    #[max_len(32, 32)]
    pub nodes: Vec<[u8; 32]>, // Use Vec for variable-length array
    #[max_len(32, 32)]
    pub peaks: Vec<u32>, // Track peak positions
}
