use sha2::{Digest, Sha256};

/// This structure represents a Merkle Tree, with a Vector
pub struct MerkleTree {
    /// I've chosen a vector temporarily bc it was the simpler way to do it
    tree: Vec<String>,
    /// How deep reaches the table from the root to the leaves
    depth: u8,
    /// Ammount of elements
    amount: usize,
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

impl MerkleTree {
    pub fn new() -> Self {
        let tree = Vec::new();
        MerkleTree {
            tree,
            depth: 0,
            amount: 0,
        }
    }

    fn hash_raw(raw_text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(raw_text);
        let hashed: [u8; 32] = hasher.finalize().into();
        hex::encode(hashed)
    }

    fn combine_hashes(hash_left: &str, hash_right: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(hash_left);
        hasher.update(hash_right);
        let hashed: [u8; 32] = hasher.finalize().into();
        hex::encode(hashed)
    }

    pub fn add_raw(&mut self, raw_text: String) {
        let hashed_string = MerkleTree::hash_raw(&raw_text);

        self.add(hashed_string);
    }

    /// The logic is: first resize if needed, second insert the element, and then recalculate the middle and root hashes
    pub fn add(&mut self, hashed_string: String) {
        self.resize_tree();

        self.insert_hash(hashed_string);

        self.rehash_tree(0);
    }

    /// When depth increase is needed, then insert the middle hashes required to calculate all the leaf hashes of the level
    fn resize_tree(&mut self) {
        if self.amount == 0 {
            self.depth = 1;
            self.tree.insert(0, "".to_string());
        }
        // Needed this bc 1 is power of two and should not execute the logic that is inside the if
        if self.amount == 1 {
            return;
        }
        if MerkleTree::number_si_power_of_two(self.amount as f32) {
            self.depth += 1;
            for _ in 0..self.amount {
                self.tree.insert(self.amount - 1, "".to_string());
            }
        }
    }

    fn number_si_power_of_two(num: f32) -> bool {
        if num <= 0.0 {
            return false;
        }

        let log = num.log(2.0);
        log.fract() == 0.0
    }

    /// Decided to insert all the copies to the tree when needed to fill spaces
    fn insert_hash(&mut self, hashed_string: String){
        let gap = 2_i8.pow(self.depth as u32) - (self.amount + 1) as i8;

        let non_leaf_nodes = 2_i8.pow(self.depth as u32) as usize - 1;
        let amount_of_copies = self.tree.len() - self.amount - non_leaf_nodes;

        if gap > 0 && amount_of_copies == 0 {
        // When i do insert and there are spaces left
            for _ in 0..gap {
                self.tree.push(hashed_string.clone());
            }
            self.tree.push(hashed_string.clone());
        } else if gap <= 0 {
        // When i replace the last copy element placed to fill the elements
            self.tree.pop();
            self.tree.push(hashed_string);
        } else if amount_of_copies > 0 {
        // When i replace copy element placed to fill the elements but it's not the last
            self.tree.pop();
            self.tree
                .insert(non_leaf_nodes + self.amount, hashed_string);
        }

        self.amount += 1;
    }

    /// The logic is: First, insert the element, and then recalculate the middle hashes
    fn rehash_tree(&mut self, pos: usize) {
        if self.tree.get(pos).is_none() {
            return;
        }
        self.rehash_tree(pos + 1);
        let pos_hash = self.tree[pos].clone();
        let result = match self.tree.get(2 * pos + 1) {
            Some(hashed_left) => match self.tree.get(2 * pos + 2) {
                Some(hashed_right) => {
                    MerkleTree::combine_hashes(hashed_left, hashed_right)
                }
                None => hashed_left.to_string(),
            },
            None => pos_hash,
        };

        self.tree[pos] = result;
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleTree;

    #[test]
    fn test_01() {
        // Create a MerkleTree and begins with a vec with an empty string and an initial depth of 1
        let tree = MerkleTree::new();
        assert_eq!(0, tree.depth);
        assert_eq!(0, tree.amount);
    }

    #[test]
    fn test_02() {
        // Add a raw text to the tree, grows depth and tree now contains the hash
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());

        let hased_string_0 = MerkleTree::hash_raw("Merkle Tree");
        let hased_string_1 = MerkleTree::hash_raw("Merkle Tree");
        let hashed_string_root = MerkleTree::combine_hashes(&hased_string_0, &hased_string_1);
        
        assert_eq!(1, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // e92a2fd865f0aada3a9b81de2ca576ae627c025dd282fc2be754f9dee4e234fd
    }

    #[test]
    fn test_03() {
        // Adds two raw texts to the tree, depth is two and tree root is result of hashing both
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());

        let hashed_string_0 = MerkleTree::hash_raw("Merkle Tree");
        let hashed_string_1 = MerkleTree::hash_raw("Ralph Merkle");

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(1, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 5a13e205575dc3d9a374dfe32941511e62f8cf900fb9df59cae9c17bd8b8ce15
    }

    #[test]
    fn test_04() {
        // Adds three raw texts to the tree, depth is two and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());

        let hashed_string_00 = MerkleTree::hash_raw("Merkle Tree");
        let hashed_string_01 = MerkleTree::hash_raw("Ralph Merkle");

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);

        let hashed_string_10 = MerkleTree::hash_raw("Game of Life");
        let hashed_string_11 = MerkleTree::hash_raw("Game of Life");

        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(2, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // d28d8deea9f793a014e668ea4050f34dc669cfc6084cd7bf3ba9ccdf62901cbf
    }

    #[test]
    fn test_05() {
        // Adds four raw texts to the tree, depth is two and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());
        tree.add_raw("John Conway".to_string());

        let hashed_string_00 = MerkleTree::hash_raw("Merkle Tree");
        let hashed_string_01 = MerkleTree::hash_raw("Ralph Merkle");

        let hashed_string_10 = MerkleTree::hash_raw("Game of Life");
        let hashed_string_11 = MerkleTree::hash_raw("John Conway");

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(2, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 8b63c8eebf3c438a9e6aff8c860febfda5d28ab473faa6c6375a01009920b91d
    }

    #[test]
    fn test_06() {
        // Adds five raw texts to the tree, depth is three and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());
        tree.add_raw("John Conway".to_string());

        tree.add_raw("Tetris".to_string());

        let hashed_string_000 = MerkleTree::hash_raw("Merkle Tree");
        let hashed_string_001 = MerkleTree::hash_raw("Ralph Merkle");

        let hashed_string_010 = MerkleTree::hash_raw("Game of Life");
        let hashed_string_011 = MerkleTree::hash_raw("John Conway");

        let hashed_string_00 = MerkleTree::combine_hashes(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = MerkleTree::combine_hashes(&hashed_string_010, &hashed_string_011);

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);

        let hashed_string_100 = MerkleTree::hash_raw("Tetris");
        let hashed_string_101 = MerkleTree::hash_raw("Tetris");
        let hashed_string_110 = MerkleTree::hash_raw("Tetris");
        let hashed_string_111 = MerkleTree::hash_raw("Tetris");

        let hashed_string_10 = MerkleTree::combine_hashes(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = MerkleTree::combine_hashes(&hashed_string_110, &hashed_string_111);

        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);
        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(3, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 8b63c8eebf3c438a9e6aff8c860febfda5d28ab473faa6c6375a01009920b91d
    }

    #[test]
    fn test_07() {
        // Adds eight raw texts to the tree, depth is three and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());
        tree.add_raw("John Conway".to_string());

        tree.add_raw("Tetris1".to_string());
        tree.add_raw("Tetris2".to_string());
        tree.add_raw("Tetris3".to_string());
        tree.add_raw("Tetris4".to_string());

        let hashed_string_000 = MerkleTree::hash_raw("Merkle Tree");
        let hashed_string_001 = MerkleTree::hash_raw("Ralph Merkle");
        let hashed_string_010 = MerkleTree::hash_raw("Game of Life");
        let hashed_string_011 = MerkleTree::hash_raw("John Conway");
        let hashed_string_100 = MerkleTree::hash_raw("Tetris1");
        let hashed_string_101 = MerkleTree::hash_raw("Tetris2");
        let hashed_string_110 = MerkleTree::hash_raw("Tetris3");
        let hashed_string_111 = MerkleTree::hash_raw("Tetris4");

        let hashed_string_00 = MerkleTree::combine_hashes(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = MerkleTree::combine_hashes(&hashed_string_010, &hashed_string_011);
        let hashed_string_10 = MerkleTree::combine_hashes(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = MerkleTree::combine_hashes(&hashed_string_110, &hashed_string_111);

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(3, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 584d46bf1bfe774bca9d4f620d127a87a2f78a341001f5f644a2f5f153c82cad
    }

    #[test]
    fn test_08() {
        // Adds nine raw texts to the tree, depth is four and tree root is result of hashing all
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

        let hashed_string_0000 = MerkleTree::hash_raw("Merkle Tree");
        let hashed_string_0001 = MerkleTree::hash_raw("Ralph Merkle");
        let hashed_string_0010 = MerkleTree::hash_raw("Game of Life");
        let hashed_string_0011 = MerkleTree::hash_raw("John Conway");
        let hashed_string_0100 = MerkleTree::hash_raw("Tetris1");
        let hashed_string_0101 = MerkleTree::hash_raw("Tetris2");
        let hashed_string_0110 = MerkleTree::hash_raw("Tetris3");
        let hashed_string_0111 = MerkleTree::hash_raw("Tetris4");

        let hashed_string_1000 = MerkleTree::hash_raw("Tetris5");
        let hashed_string_1001 = MerkleTree::hash_raw("Tetris5");
        let hashed_string_1010 = MerkleTree::hash_raw("Tetris5");
        let hashed_string_1011 = MerkleTree::hash_raw("Tetris5");
        let hashed_string_1100 = MerkleTree::hash_raw("Tetris5");
        let hashed_string_1101 = MerkleTree::hash_raw("Tetris5");
        let hashed_string_1110 = MerkleTree::hash_raw("Tetris5");
        let hashed_string_1111 = MerkleTree::hash_raw("Tetris5");

        let hashed_string_000 = MerkleTree::combine_hashes(&hashed_string_0000, &hashed_string_0001);
        let hashed_string_001 = MerkleTree::combine_hashes(&hashed_string_0010, &hashed_string_0011);
        let hashed_string_010 = MerkleTree::combine_hashes(&hashed_string_0100, &hashed_string_0101);
        let hashed_string_011 = MerkleTree::combine_hashes(&hashed_string_0110, &hashed_string_0111);

        let hashed_string_100 = MerkleTree::combine_hashes(&hashed_string_1000, &hashed_string_1001);
        let hashed_string_101 = MerkleTree::combine_hashes(&hashed_string_1010, &hashed_string_1011);
        let hashed_string_110 = MerkleTree::combine_hashes(&hashed_string_1100, &hashed_string_1101);
        let hashed_string_111 = MerkleTree::combine_hashes(&hashed_string_1110, &hashed_string_1111);

        let hashed_string_00 = MerkleTree::combine_hashes(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = MerkleTree::combine_hashes(&hashed_string_010, &hashed_string_011);

        let hashed_string_10 = MerkleTree::combine_hashes(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = MerkleTree::combine_hashes(&hashed_string_110, &hashed_string_111);

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(4, tree.depth);
        assert_eq!(hashed_string_root, tree.tree[0]);
        // 7d6aca7ece41a33246a1fe3d13dcf074b701aa43717a19a93047553fc38294b0
    }

    #[test]
    fn test_9() {
        // Assert that hash function works correctly
        assert_eq!(MerkleTree::hash_raw("Merkle Tree"), "cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952");
        assert_eq!(MerkleTree::hash_raw("Merkle Root"), "09b4b6987df5353bfe0055491ac474539691011d0e95ecdaf8ad06906504308b");
        assert_eq!(MerkleTree::hash_raw("Ralph Merkle"), "5a93dda4ddfe626b84b6ffdb6f4ee27da108a28762247359b9d25310c6f00736");
    }

    #[test]
    fn test_10() {
        // Assert that the combine hashes function works as expected
        let hash_left = MerkleTree::hash_raw("Merkle Tree");
        let hash_right = MerkleTree::hash_raw("Merkle Root");
        assert_eq!(MerkleTree::combine_hashes(&hash_left, &hash_right), "c4f431efc6c50e3b703e11233dd219eaef584c24e4a4b76da22487eb74ec9258");
        assert_eq!(MerkleTree::combine_hashes(&hash_right, &hash_left), "39d978a783e10f39b039ff6a022d7761f8bf74104d663717037e4825d86da10b");
    }

    #[test]
    fn test_11() {
        assert!(MerkleTree::number_si_power_of_two(1.));
        assert!(MerkleTree::number_si_power_of_two(2.));
        assert!(MerkleTree::number_si_power_of_two(8.));
        assert!(MerkleTree::number_si_power_of_two(64.));
        assert!(MerkleTree::number_si_power_of_two(128.));
        assert!(MerkleTree::number_si_power_of_two(512.));
        assert!(MerkleTree::number_si_power_of_two(2048.));
    }



}
