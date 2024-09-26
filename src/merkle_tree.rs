use sha2::{Digest, Sha256};
use std::f32;


pub struct MerkleTree {
    tree: Vec<String>,
    depth: u8,
}

impl MerkleTree {
    pub fn new () -> Self {
        let mut tree = Vec::new();
        tree.push("".to_string());
        MerkleTree { tree, depth:0}
    }

    pub fn add_raw(&mut self, raw_text: String) {
        let mut hasher = Sha256::new();
        hasher.update(raw_text);
        let hashed: [u8; 32] = hasher.finalize().into();
        let hashed_string = hex::encode(hashed);
        println!("{:?}", hashed_string);

        self.add(hashed_string);
    }

    pub fn add(&mut self, hashed_string: String) {

        self.tree.push(hashed_string);
        
        self.rehash_tree(0);

        // Keep it simple, could not afford more than 4 elements
        let tree_len = self.tree.len();
        self.depth = if tree_len == 1 {
            0
        } else if (2..4).contains(&tree_len) {
            1
        } else {
            2
        };
    }

    fn rehash_tree(&mut self, pos: usize) {
        if let None = self.tree.get(pos) {
            return;
        }
        let pos_hash = self.tree[pos].clone();
        let result = match self.tree.get(2*pos + 1) {
            Some(hashed_left) => {
                match self.tree.get(2*pos + 2) {
                    Some(hashed_right) => {
                        MerkleTree::combine_hashes(hashed_left.to_string(), hashed_right.to_string())
                    },
                    None => {
                        hashed_left.to_string()            
                    },
                }
            },
            None => {
                pos_hash      
            },
        };
        println!("{result}");
        self.tree[pos] = result;
        self.rehash_tree(pos + 1);
    }

    fn combine_hashes (hash_left: String, hash_right: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(hash_left);
        hasher.update(hash_right);
        let hashed: [u8; 32] = hasher.finalize().into();
        hex::encode(hashed)
    }
}


#[cfg(test)]
mod tests {
    use sha2::{Digest, Sha256};

    use super::MerkleTree;

    #[test]
    fn test_01() {
        // Create a MerkleTree and begins with an empty vec and an initial depth of 1
        let tree = MerkleTree::new();
        assert_eq!(0, tree.depth);
        assert_eq!(1, tree.tree.len());
    }

    #[test]
    fn test_02 () {
        // Add a raw text to the tree, grows depth and tree now contains the hash
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());  
        
        assert_eq!(1, tree.depth);
        assert_eq!("cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952", tree.tree[0]);
    }

    #[test]
    fn test_03 () {
        // Add a two raw texts to the tree, depth is two and tree root is result of hashing both
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        println!("{:?} ", tree.tree);
        tree.add_raw("Ralph Merkle".to_string());
        println!("{:?} ", tree.tree);

        let mut hasher = Sha256::new();
        hasher.update("Merkle Tree");
        let hashed: [u8; 32] = hasher.finalize().into();
        let hashed_string_1 = hex::encode(hashed);
        let mut hasher = Sha256::new();
        hasher.update("Ralph Merkle");
        let hashed: [u8; 32] = hasher.finalize().into();
        let hashed_string_2 = hex::encode(hashed);
        println!("Hashed_string 2: {} ", hashed_string_2);

        let mut hasher = Sha256::new();
        hasher.update(hashed_string_1);
        hasher.update(hashed_string_2);
        let hashed: [u8; 32] = hasher.finalize().into();
        let hashed_string_root = hex::encode(hashed);
        
        assert_eq!(1, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
    }

}
