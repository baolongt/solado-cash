use anchor_lang::prelude::*;
use crate::{ errors::AppError, Deposit };

pub fn generate_proof(ctx: &Context<Deposit>, leaf_index: u32) -> Result<String> {
    let mmr_account = &ctx.accounts.mmr_account;

    // Check leaf_index validity
    if leaf_index >= (mmr_account.nodes.len() as u32) {
        return err!(AppError::InvalidLeafIndex);
    }

    let mut proof = Vec::new();
    let mut current_index = leaf_index;

    while current_index > 0 {
        let sibling_index = if current_index % 2 == 0 {
            current_index + 1
        } else {
            current_index - 1
        };
        let sibling_within_bounds = sibling_index < (mmr_account.nodes.len() as u32);

        if sibling_within_bounds {
            proof.push(mmr_account.nodes[sibling_index as usize]);
        }

        // Check if current node is a peak
        if mmr_account.peaks.contains(&current_index) {
            // If it is a peak, but it has a sibling on the right, we should continue traversing up
            if sibling_within_bounds && !mmr_account.peaks.contains(&sibling_index) {
                current_index = current_index / 2;
                continue;
            } else {
                // If it's a peak with no right sibling, we've reached the root
                break;
            }
        }

        current_index = current_index / 2;
    }

    // Convert proof elements to a hex string (unchanged from your original code)
    let mut string = String::new();
    proof.iter().for_each(|item| {
        item.iter().for_each(|byte| {
            string.push_str(&format!("{:02x}", byte));
        });
    });

    Ok(string)
}
