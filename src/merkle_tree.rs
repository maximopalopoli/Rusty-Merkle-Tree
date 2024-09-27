use sha2::{Digest, Sha256};


pub struct MerkleTree {
    tree: Vec<String>,
    depth: u8,
    amount: usize,
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

        let gap = 2_i8.pow(self.depth as u32) - (self.amount + 1) as i8;

        let non_leaf_nodes = 2_i8.pow(self.depth as u32) as usize - 1;
        let amount_of_copies = self.tree.len() - self.amount - non_leaf_nodes;

        if gap > 0 && amount_of_copies <= 0 {
            for _ in 0..gap {
                self.tree.push(hashed_string.clone());
            }
            self.tree.push(hashed_string.clone());
        } else if gap <= 0 {
            self.tree.pop();
            self.tree.push(hashed_string);
        } else if amount_of_copies > 0 {
            self.tree.pop();
            self.tree.insert(non_leaf_nodes + self.amount, hashed_string);
        }
        
        self.amount += 1;
        self.rehash_tree(0);
    }

    fn resize_tree(&mut self) {
        if self.amount == 0 {
            self.depth = 1;
            self.tree.insert(0, "".to_string());
        }
        if self.amount == 1 {
            return;
        }
        if MerkleTree::number_is_exact_log_2(self.amount as f32){
            self.depth += 1;
            for _ in 0..self.amount {
                self.tree.insert(self.amount - 1, "".to_string());
            }
        }
    }

    fn number_is_exact_log_2(num: f32) -> bool {
        if num <= 0.0 {
            return false;
        }
    
        let log = num.log(2.0);
        log.fract() == 0.0
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

    fn calculate_hash_double(hashed_string_0: &str, hashed_string_1: &str) -> String {
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
        
        let hased_string_0 = calculate_hash("Merkle Tree");
        let hased_string_1 = calculate_hash("Merkle Tree");
        let hashed_string_root = calculate_hash_double(&hased_string_0, &hased_string_1);

        assert_eq!(1, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // e92a2fd865f0aada3a9b81de2ca576ae627c025dd282fc2be754f9dee4e234fd
    }

    #[test]
    fn test_03 () {
        // Add a two raw texts to the tree, depth is two and tree root is result of hashing both
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());

        let hashed_string_0 = calculate_hash("Merkle Tree");
        let hashed_string_1 = calculate_hash("Ralph Merkle");

        let hashed_string_root = calculate_hash_double(&hashed_string_0, &hashed_string_1);
        
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
        
        let hashed_string_0 = calculate_hash_double(&hashed_string_00, &hashed_string_01);

        let hashed_string_10 = calculate_hash("Game of Life");
        let hashed_string_11 = calculate_hash("Game of Life");

       let hashed_string_1 = calculate_hash_double(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = calculate_hash_double(&hashed_string_0, &hashed_string_1);
        
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

        let hashed_string_0 = calculate_hash_double(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = calculate_hash_double(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = calculate_hash_double(&hashed_string_0, &hashed_string_1);

        assert_eq!(2, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 8b63c8eebf3c438a9e6aff8c860febfda5d28ab473faa6c6375a01009920b91d
    }

    #[test]
    fn test_06 () {
        // Add a five raw texts to the tree, depth is three and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());
        tree.add_raw("John Conway".to_string());

        tree.add_raw("Tetris".to_string());

        let hashed_string_000 = calculate_hash("Merkle Tree");
        let hashed_string_001 = calculate_hash("Ralph Merkle");
        
        let hashed_string_010 = calculate_hash("Game of Life");
        let hashed_string_011 = calculate_hash("John Conway");

        let hashed_string_00 = calculate_hash_double(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = calculate_hash_double(&hashed_string_010, &hashed_string_011);

        let hashed_string_0 = calculate_hash_double(&hashed_string_00, &hashed_string_01);

        let hashed_string_100 = calculate_hash("Tetris");
        let hashed_string_101 = calculate_hash("Tetris");
        let hashed_string_110 = calculate_hash("Tetris");
        let hashed_string_111 = calculate_hash("Tetris");

        let hashed_string_10 = calculate_hash_double(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = calculate_hash_double(&hashed_string_110, &hashed_string_111);
        

        let hashed_string_1 = calculate_hash_double(&hashed_string_10, &hashed_string_11);
        let hashed_string_root = calculate_hash_double(&hashed_string_0, &hashed_string_1);

        assert_eq!(3, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 8b63c8eebf3c438a9e6aff8c860febfda5d28ab473faa6c6375a01009920b91d
    }

    #[test]
    fn test_07 () {
        // Add a eight raw texts to the tree, depth is three and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());
        tree.add_raw("John Conway".to_string());

        tree.add_raw("Tetris1".to_string());
        tree.add_raw("Tetris2".to_string());
        tree.add_raw("Tetris3".to_string());
        tree.add_raw("Tetris4".to_string());

        let hashed_string_000 = calculate_hash("Merkle Tree");
        let hashed_string_001 = calculate_hash("Ralph Merkle");
        let hashed_string_010 = calculate_hash("Game of Life");
        let hashed_string_011 = calculate_hash("John Conway");
        let hashed_string_100 = calculate_hash("Tetris1");
        let hashed_string_101 = calculate_hash("Tetris2");
        let hashed_string_110 = calculate_hash("Tetris3");
        let hashed_string_111 = calculate_hash("Tetris4");

        let hashed_string_00 = calculate_hash_double(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = calculate_hash_double(&hashed_string_010, &hashed_string_011);
        let hashed_string_10 = calculate_hash_double(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = calculate_hash_double(&hashed_string_110, &hashed_string_111);

        let hashed_string_0 = calculate_hash_double(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = calculate_hash_double(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = calculate_hash_double(&hashed_string_0, &hashed_string_1);

        assert_eq!(3, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 584d46bf1bfe774bca9d4f620d127a87a2f78a341001f5f644a2f5f153c82cad
    }

    #[test]
    fn test_08 () {
        // Add a nine raw texts to the tree, depth is four and tree root is result of hashing all
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

        assert_eq!(4, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 7d6aca7ece41a33246a1fe3d13dcf074b701aa43717a19a93047553fc38294b0
    }

}

