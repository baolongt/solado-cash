use solana_program::{ keccak, msg };

pub fn merkle_verify(mut proof: Vec<[u8; 32]>, root: [u8; 32], leaf: [u8; 32]) -> bool {
    msg!("leaf: {:?}", leaf);
    let mut computed_hash = leaf;

    for proof_element in proof.into_iter() {
        msg!("proof_element: {:?}", proof_element);
        if computed_hash <= proof_element {
            computed_hash = keccak::hashv(&[&computed_hash, &proof_element]).0;
        } else {
            computed_hash = keccak::hashv(&[&proof_element, &computed_hash]).0;
        }
        msg!("computed_hash: {:?}", computed_hash);
    }

    msg!("root: {:?}", root);

    computed_hash == root
}
