use merkle_tree::MerkleTree;
pub mod merkle_tree;
use sha2::{Digest, Sha256};

fn calculate_hash(encoding_str: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(encoding_str);
    let hashed: [u8; 32] = hasher.finalize().into();
    hex::encode(hashed)
}

fn calculate_hash_double(hashed_string_0: &str, hashed_string_1: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(hashed_string_0);
    hasher.update(hashed_string_1);
    let hashed: [u8; 32] = hasher.finalize().into();
    hex::encode(hashed)
}

fn main() {
    let mut tree = MerkleTree::new();
    tree.add_raw("Merkle Tree".to_string());
    tree.add_raw("Ralph Merkle".to_string());
    tree.add_raw("Game of Life".to_string());
    tree.add_raw("John Conway".to_string());

    tree.add_raw("Tetris1".to_string());
    tree.add_raw("Tetris2".to_string());
    tree.add_raw("Tetris3".to_string());
    tree.add_raw("Tetris4".to_string());
    tree.add_raw("Tetris5".to_string());

    let hashed_string_0000 = calculate_hash("Merkle Tree");
    let hashed_string_0001 = calculate_hash("Ralph Merkle");
    let hashed_string_0010 = calculate_hash("Game of Life");
    let hashed_string_0011 = calculate_hash("John Conway");
    let hashed_string_0100 = calculate_hash("Tetris1");
    let hashed_string_0101 = calculate_hash("Tetris2");
    let hashed_string_0110 = calculate_hash("Tetris3");
    let hashed_string_0111 = calculate_hash("Tetris4");

    let hashed_string_1000 = calculate_hash("Tetris5");
    let hashed_string_1001 = calculate_hash("Tetris5");
    let hashed_string_1010 = calculate_hash("Tetris5");
    let hashed_string_1011 = calculate_hash("Tetris5");
    let hashed_string_1100 = calculate_hash("Tetris5");
    let hashed_string_1101 = calculate_hash("Tetris5");
    let hashed_string_1110 = calculate_hash("Tetris5");
    let hashed_string_1111 = calculate_hash("Tetris5");

    let hashed_string_000 = calculate_hash_double(&hashed_string_0000, &hashed_string_0001);
    let hashed_string_001 = calculate_hash_double(&hashed_string_0010, &hashed_string_0011);
    let hashed_string_010 = calculate_hash_double(&hashed_string_0100, &hashed_string_0101);
    let hashed_string_011 = calculate_hash_double(&hashed_string_0110, &hashed_string_0111);

    let hashed_string_100 = calculate_hash_double(&hashed_string_1000, &hashed_string_1001);
    let hashed_string_101 = calculate_hash_double(&hashed_string_1010, &hashed_string_1011);
    let hashed_string_110 = calculate_hash_double(&hashed_string_1100, &hashed_string_1101);
    let hashed_string_111 = calculate_hash_double(&hashed_string_1110, &hashed_string_1111);

    let hashed_string_00 = calculate_hash_double(&hashed_string_000, &hashed_string_001);
    let hashed_string_01 = calculate_hash_double(&hashed_string_010, &hashed_string_011);

    let hashed_string_10 = calculate_hash_double(&hashed_string_100, &hashed_string_101);
    let hashed_string_11 = calculate_hash_double(&hashed_string_110, &hashed_string_111);

    let hashed_string_0 = calculate_hash_double(&hashed_string_00, &hashed_string_01);
    let hashed_string_1 = calculate_hash_double(&hashed_string_10, &hashed_string_11);

    let hashed_string_root = calculate_hash_double(&hashed_string_0, &hashed_string_1);

    println!("{hashed_string_root}");
}
