use merkle_tree::MerkleTree;
pub mod merkle_tree;
use sha2::{Digest, Sha256};


fn calculate_hash(encoding_str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(encoding_str);
    let hashed: [u8; 32] = hasher.finalize().into();
    hex::encode(hashed)
}


fn main() {
    let mut tree = MerkleTree::new();
    tree.add_raw("Merkle Tree".to_string());
    
    let hashed_string_root = calculate_hash("Merkle Tree");

    println!("{hashed_string_root}");    
}
