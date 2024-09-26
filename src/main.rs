use merkle_tree::MerkleTree;
pub mod merkle_tree;


fn main() {
    let mut m_tree = MerkleTree::new();
    m_tree.add_raw("Merkle Tree".to_string());
}
