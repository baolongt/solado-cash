use solana_program::{ msg, keccak };

pub fn hash_pair(left: &[u8; 32], right: &[u8; 32]) -> [u8; 32] {
    if left <= right {
        let hash = keccak::hashv(&[left, right]).0;
        hash
    } else {
        let hash = keccak::hashv(&[right, left]).0;
        hash
    }
}
