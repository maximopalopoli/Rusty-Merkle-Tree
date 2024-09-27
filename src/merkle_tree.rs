use sha2::{Digest, Sha256};


pub struct MerkleTree {
    tree: Vec<String>,
    depth: u8,
    amount: u8,
}

impl MerkleTree {
    pub fn new () -> Self {
        let tree = Vec::new();
        MerkleTree { tree, depth:0, amount: 0}
    }

    pub fn add_raw(&mut self, raw_text: String) {
        let mut hasher = Sha256::new();
        hasher.update(raw_text);
        let hashed: [u8; 32] = hasher.finalize().into();
        let hashed_string = hex::encode(hashed);

        self.add(hashed_string);
    }

    pub fn add(&mut self, hashed_string: String) {
        self.resize_tree();

        self.tree.push(hashed_string);
        
        self.rehash_tree(0);

        self.amount += 1;
    }

    fn resize_tree(&mut self) {
        // Keep it simple, could not afford more than 8 elements
        if (self.amount + 1) == 1 {
            self.depth = 1;
            self.tree.insert(0, "".to_string());
        }
        if (self.amount + 1) == 3 {
            self.depth = 2;
            self.tree.insert(1, "".to_string());
            self.tree.insert(1, "".to_string());

        }
        if (self.amount + 1) == 5 {
            self.depth = 3;
            self.tree.insert(3, "".to_string());
            self.tree.insert(3, "".to_string());
            self.tree.insert(3, "".to_string());
            self.tree.insert(3, "".to_string());

        }
        //El proximo sería 9
    }

    fn rehash_tree(&mut self, pos: usize) {
        if let None = self.tree.get(pos) {
            return;
        }
        self.rehash_tree(pos + 1);
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

        self.tree[pos] = result;
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

    fn calculate_hash(encoding_str: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(encoding_str);
        let hashed: [u8; 32] = hasher.finalize().into();
        hex::encode(hashed)
    }

    fn calculate_hash_double(hashed_string_0: String, hashed_string_1: String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(hashed_string_0);
        hasher.update(hashed_string_1);
        let hashed: [u8; 32] = hasher.finalize().into();
        hex::encode(hashed)
    }

    #[test]
    fn test_01() {
        // Create a MerkleTree and begins with a vec with an empty string and an initial depth of 1
        let tree = MerkleTree::new();
        assert_eq!(0, tree.depth);
        assert_eq!(0, tree.amount);
    }

    #[test]
    fn test_02 () {
        // Add a raw text to the tree, grows depth and tree now contains the hash
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        
        let hashed_string_root = calculate_hash("Merkle Tree");

        assert_eq!(1, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952
    }

    #[test]
    fn test_03 () {
        // Add a two raw texts to the tree, depth is two and tree root is result of hashing both
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());

        let hashed_string_0 = calculate_hash("Merkle Tree");
        let hashed_string_1 = calculate_hash("Ralph Merkle");

        let hashed_string_root = calculate_hash_double(hashed_string_0, hashed_string_1);
        
        assert_eq!(1, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 5a13e205575dc3d9a374dfe32941511e62f8cf900fb9df59cae9c17bd8b8ce15
    }

    #[test]
    fn test_04 () {
        // Add a three raw texts to the tree, depth is two and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());

        let hashed_string_00 = calculate_hash("Merkle Tree");
        let hashed_string_01 = calculate_hash("Ralph Merkle");
        
        let hashed_string_0 = calculate_hash_double(hashed_string_00, hashed_string_01);

        let hashed_string_1 = calculate_hash("Game of Life");

        let hashed_string_root = calculate_hash_double(hashed_string_0, hashed_string_1);
        
        assert_eq!(2, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // d28d8deea9f793a014e668ea4050f34dc669cfc6084cd7bf3ba9ccdf62901cbf
    }

    #[test]
    fn test_05 () {
        // Add a four raw texts to the tree, depth is two and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());
        tree.add_raw("John Conway".to_string());

        let hashed_string_00 = calculate_hash("Merkle Tree");
        let hashed_string_01 = calculate_hash("Ralph Merkle");
        
        let hashed_string_10 = calculate_hash("Game of Life");
        let hashed_string_11 = calculate_hash("John Conway");

        let hashed_string_0 = calculate_hash_double(hashed_string_00, hashed_string_01);
        let hashed_string_1 = calculate_hash_double(hashed_string_10, hashed_string_11);

        let hashed_string_root = calculate_hash_double(hashed_string_0, hashed_string_1);

        assert_eq!(2, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 8b63c8eebf3c438a9e6aff8c860febfda5d28ab473faa6c6375a01009920b91d
    }

}
