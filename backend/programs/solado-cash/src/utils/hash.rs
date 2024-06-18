use solana_program::hash::hash;

pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    let mut combined_leaf_bytes = [0u8; 64];
    combined_leaf_bytes[..32].copy_from_slice(left);
    combined_leaf_bytes[32..].copy_from_slice(right);
    let combined_leaf_hash = hash(&combined_leaf_bytes);
    combined_leaf_hash.to_bytes()
}
