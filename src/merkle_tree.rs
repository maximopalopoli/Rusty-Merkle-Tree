use sha2::{Digest, Sha256};

/// This structure represents a Merkle Tree, with a Vector
pub struct MerkleTree {
    /// I've chosen a vector temporarily bc it was the simpler way to do it
    elements: Vec<String>,
    /// Ammount of inserted leaf nodes (without reapeated ones)
    inserted_elements_amount: usize,
}

impl Default for MerkleTree {
    fn default() -> Self {
        Self::new()
    }
}

impl MerkleTree {
    pub fn new() -> Self {
        let elements = Vec::new();
        MerkleTree {
            elements,
            inserted_elements_amount: 0,
        }
    }

    pub fn build(hashes: Vec<&str>, unhashed: bool) -> Self {
        let mut tree = MerkleTree::new();

        for hash in hashes {
            if unhashed {
                tree.add_unhashed(hash.to_string());
            } else {
                tree.add(hash.to_string());
            }
        }

        tree
    }

    fn hash_text(unhashed_text: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(unhashed_text);
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

    pub fn add_unhashed(&mut self, unhashed_text: String) {
        let hashed_string = MerkleTree::hash_text(&unhashed_text);

        self.add(hashed_string);
    }

    /// The logic is: first expand the tree if needed, second insert the element, and then recalculate the middle and root hashes
    pub fn add(&mut self, hashed_string: String) {
        self.expand_tree();

        self.insert_hash(hashed_string);

        self.rehash_tree(0);
    }

    /// When depth increase is needed, then insert the middle hash nodes required to calculate all the leaf hashes of the level
    fn expand_tree(&mut self) {
        if self.inserted_elements_amount == 0 {
            self.elements.insert(0, "".to_string());
        }
        // Needed this bc 1 is power of two and should not execute the logic that is inside the lower if
        if self.inserted_elements_amount == 1 {
            return;
        }
        if MerkleTree::number_is_power_of_two(self.inserted_elements_amount as f32) {
            // The need of the for is to insert the non_leaf_nodes that will be used to calculate the root hash
            for i in 0..self.inserted_elements_amount {
                // Note: self.inserted_elements_amount is a proxy of the number of copies
                self.elements
                    .insert(self.inserted_elements_amount - 1 + i, "".to_string());
                // These are empty strings because they will be calculated in the rehash_tree function from lower nodes
            }
        }
    }

    fn number_is_power_of_two(num: f32) -> bool {
        if num <= 0.0 {
            return false;
        }

        let log = num.log(2.0);
        log.fract() == 0.0
    }

    /// Decided to insert all the copies to the tree when needed to fill spaces
    fn insert_hash(&mut self, hashed_string: String) {
        let non_leaf_nodes =
            2_usize.pow(f32::log2(self.inserted_elements_amount as f32) as u32 + 1) - 1;

        let gap = non_leaf_nodes - self.inserted_elements_amount;
        let amount_of_copies = self.elements.len() - self.inserted_elements_amount - non_leaf_nodes;

        if gap > 0 && amount_of_copies == 0 {
            // When i do insert and there are spaces left
            for _ in 0..gap {
                self.elements.push(hashed_string.clone());
            }
            self.elements.push(hashed_string.clone());
        } else if gap == 0 {
            // When i replace the last copy element placed to fill the elements
            self.elements.pop();
            self.elements.push(hashed_string);
        } else if amount_of_copies > 0 {
            // When i replace copy element placed to fill the elements but it's not the last
            self.elements.pop();
            self.elements.insert(
                non_leaf_nodes + self.inserted_elements_amount,
                hashed_string,
            );
        }

        self.inserted_elements_amount += 1;
    }

    /// The logic is: First, insert the element, and then recalculate the middle hashes
    fn rehash_tree(&mut self, pos: usize) {
        // Use is_none bc cargo clippy sugered it instead of an if let
        if self.elements.get(pos).is_none() {
            return;
        }

        // Here i make the lower nodes be hashed before current node makes the hashing. Note that if the following is Null
        // the error will be catched in the if is_none at the beginning of the function
        self.rehash_tree(pos + 1);

        // This can be reasoned the following way: If have two sons, my hash is the result of hashing both. If I have only
        // one, I'll hash it with a copy of itself, and if I dont have sons (I'm a leaf node) y return my own hash
        let pos_hash = self.elements[pos].clone();
        let result = match self.elements.get(2 * pos + 1) {
            Some(hashed_left) => match self.elements.get(2 * pos + 2) {
                Some(hashed_right) => MerkleTree::combine_hashes(hashed_left, hashed_right),
                None => hashed_left.to_string(),
            },
            None => pos_hash,
        };

        self.elements[pos] = result;
    }

    /// The logic is: From the leaf, hashing with the proofs I reach my own root and compare it to the original
    pub fn verify(&self, proof: Vec<String>, leaf: String, index: &mut i32) -> bool {
        let mut hash = leaf;

        MerkleTree::generate_root(proof, &mut hash, index);

        hash == self.elements[0]
    }

    /// Here I do the combinations to reach the root
    fn generate_root(proof: Vec<String>, hash: &mut String, index: &mut i32) {
        for proof_element in proof {
            if *index % 2 == 0 {
                *hash = MerkleTree::combine_hashes(hash, &proof_element);
            } else {
                *hash = MerkleTree::combine_hashes(&proof_element, hash);
            }

            *index /= 2;
        }
    }

    /// Made a similar advance to the verify method, but here I save the sibling instead of rehashing
    pub fn generate_proof(&mut self, index: &mut usize) -> Vec<String> {
        let mut proof: Vec<String> = Vec::new();

        let non_leaf_nodes =
            2_i8.pow(f32::log2(self.inserted_elements_amount as f32) as u32) as usize - 1;
        *index += non_leaf_nodes;

        // raises a never read error, but IMO it's not a real problem
        #[allow(unused_assignments)]
        let mut even_offset = 0; // Exists for handling the climbing of the tree to the root

        while *index >= 1 {
            if *index % 2 == 0 {
                proof.push(self.elements[*index - 1].clone());
                even_offset = 1;
            } else {
                proof.push(self.elements[*index + 1].clone());
                even_offset = 0;
            }

            *index = *index / 2 - even_offset;
        }

        proof
    }

    pub fn print(&self) {
        let levels = (0..)
            .take_while(|&n| (1 << n) - 1 < self.elements.len())
            .count();
        for i in 0..levels {
            let level_nodes = 1 << i;
            let begin = (1 << i) - 1;
            let end = begin + level_nodes;

            let spaces = (2 << (levels - i - 1)) - 1;
            print!("{:width$}", "", width = spaces);

            for j in begin..end {
                if j < self.elements.len() {
                    print!("{}..  ", self.elements[j].clone().split_at(4).0);
                }
            }
            println!();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::MerkleTree;

    #[test]
    fn test_01_tree_is_created_with_valid_args() {
        // Create a MerkleTree and begins with an empty vec
        let tree = MerkleTree::new();

        assert_eq!(0, tree.elements.len());
        assert_eq!(0, tree.inserted_elements_amount);
    }

    #[test]
    fn test_02_adding_one_text_adds_the_hash_to_the_vector() {
        // Add a unhashed text to the tree, there are three elements and tree now contains the hash
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());

        let hased_string_0 = MerkleTree::hash_text("Merkle Tree");
        let hased_string_1 = MerkleTree::hash_text("Merkle Tree");
        let hashed_string_root = MerkleTree::combine_hashes(&hased_string_0, &hased_string_1);

        assert_eq!(3, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // e92a2fd865f0aada3a9b81de2ca576ae627c025dd282fc2be754f9dee4e234fd
    }

    #[test]
    fn test_03_adding_more_than_one_makes_root_a_hash_combination() {
        // Adds two unhashed texts to the tree, there are three elements in vector and tree root is result of hashing both
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());

        let hashed_string_0 = MerkleTree::hash_text("Merkle Tree");
        let hashed_string_1 = MerkleTree::hash_text("Ralph Merkle");

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(3, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // 5a13e205575dc3d9a374dfe32941511e62f8cf900fb9df59cae9c17bd8b8ce15
    }

    #[test]
    fn test_04_adding_three_elements_increases_depth_to_two() {
        // Adds three unhashed texts to the tree, there are seven elements in vector and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());
        tree.add_unhashed("Game of Life".to_string());

        let hashed_string_00 = MerkleTree::hash_text("Merkle Tree");
        let hashed_string_01 = MerkleTree::hash_text("Ralph Merkle");

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);

        let hashed_string_10 = MerkleTree::hash_text("Game of Life");
        let hashed_string_11 = MerkleTree::hash_text("Game of Life");

        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(7, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // d28d8deea9f793a014e668ea4050f34dc669cfc6084cd7bf3ba9ccdf62901cbf
    }

    #[test]
    fn test_05_adding_four_elements_doesnt_increase_depth_to_three() {
        // Adds four unhashed texts to the tree, there are seven elements in vector and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());
        tree.add_unhashed("Game of Life".to_string());
        tree.add_unhashed("John Conway".to_string());

        let hashed_string_00 = MerkleTree::hash_text("Merkle Tree");
        let hashed_string_01 = MerkleTree::hash_text("Ralph Merkle");

        let hashed_string_10 = MerkleTree::hash_text("Game of Life");
        let hashed_string_11 = MerkleTree::hash_text("John Conway");

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(7, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // 8b63c8eebf3c438a9e6aff8c860febfda5d28ab473faa6c6375a01009920b91d
    }

    #[test]
    fn test_06_adding_five_elements_increases_depth_to_three() {
        // Adds five unhashed texts to the tree, there are fifteen elements in vector and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());
        tree.add_unhashed("Game of Life".to_string());
        tree.add_unhashed("John Conway".to_string());

        tree.add_unhashed("Tetris".to_string());

        let hashed_string_000 = MerkleTree::hash_text("Merkle Tree");
        let hashed_string_001 = MerkleTree::hash_text("Ralph Merkle");

        let hashed_string_010 = MerkleTree::hash_text("Game of Life");
        let hashed_string_011 = MerkleTree::hash_text("John Conway");

        let hashed_string_00 = MerkleTree::combine_hashes(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = MerkleTree::combine_hashes(&hashed_string_010, &hashed_string_011);

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);

        let hashed_string_100 = MerkleTree::hash_text("Tetris");
        let hashed_string_101 = MerkleTree::hash_text("Tetris");
        let hashed_string_110 = MerkleTree::hash_text("Tetris");
        let hashed_string_111 = MerkleTree::hash_text("Tetris");

        let hashed_string_10 = MerkleTree::combine_hashes(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = MerkleTree::combine_hashes(&hashed_string_110, &hashed_string_111);

        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);
        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(15, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // 8b63c8eebf3c438a9e6aff8c860febfda5d28ab473faa6c6375a01009920b91d
    }

    #[test]
    fn test_07_adding_eight_elements_doesnt_increase_depth_to_four() {
        // Adds eight unhashed texts to the tree, there are fifteen elements in vector and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());
        tree.add_unhashed("Game of Life".to_string());
        tree.add_unhashed("John Conway".to_string());

        tree.add_unhashed("Tetris1".to_string());
        tree.add_unhashed("Tetris2".to_string());
        tree.add_unhashed("Tetris3".to_string());
        tree.add_unhashed("Tetris4".to_string());

        let hashed_string_000 = MerkleTree::hash_text("Merkle Tree");
        let hashed_string_001 = MerkleTree::hash_text("Ralph Merkle");
        let hashed_string_010 = MerkleTree::hash_text("Game of Life");
        let hashed_string_011 = MerkleTree::hash_text("John Conway");
        let hashed_string_100 = MerkleTree::hash_text("Tetris1");
        let hashed_string_101 = MerkleTree::hash_text("Tetris2");
        let hashed_string_110 = MerkleTree::hash_text("Tetris3");
        let hashed_string_111 = MerkleTree::hash_text("Tetris4");

        let hashed_string_00 = MerkleTree::combine_hashes(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = MerkleTree::combine_hashes(&hashed_string_010, &hashed_string_011);
        let hashed_string_10 = MerkleTree::combine_hashes(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = MerkleTree::combine_hashes(&hashed_string_110, &hashed_string_111);

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(15, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // 584d46bf1bfe774bca9d4f620d127a87a2f78a341001f5f644a2f5f153c82cad
    }

    #[test]
    fn test_08_adding_nine_elements_increases_depth_to_four() {
        // Adds nine unhashed texts to the tree, there are thirty one elements in vector and tree root is result of hashing all
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());
        tree.add_unhashed("Game of Life".to_string());
        tree.add_unhashed("John Conway".to_string());

        tree.add_unhashed("Tetris1".to_string());
        tree.add_unhashed("Tetris2".to_string());
        tree.add_unhashed("Tetris3".to_string());
        tree.add_unhashed("Tetris4".to_string());
        tree.add_unhashed("Tetris5".to_string());

        let hashed_string_0000 = MerkleTree::hash_text("Merkle Tree");
        let hashed_string_0001 = MerkleTree::hash_text("Ralph Merkle");
        let hashed_string_0010 = MerkleTree::hash_text("Game of Life");
        let hashed_string_0011 = MerkleTree::hash_text("John Conway");
        let hashed_string_0100 = MerkleTree::hash_text("Tetris1");
        let hashed_string_0101 = MerkleTree::hash_text("Tetris2");
        let hashed_string_0110 = MerkleTree::hash_text("Tetris3");
        let hashed_string_0111 = MerkleTree::hash_text("Tetris4");

        let hashed_string_1000 = MerkleTree::hash_text("Tetris5");
        let hashed_string_1001 = MerkleTree::hash_text("Tetris5");
        let hashed_string_1010 = MerkleTree::hash_text("Tetris5");
        let hashed_string_1011 = MerkleTree::hash_text("Tetris5");
        let hashed_string_1100 = MerkleTree::hash_text("Tetris5");
        let hashed_string_1101 = MerkleTree::hash_text("Tetris5");
        let hashed_string_1110 = MerkleTree::hash_text("Tetris5");
        let hashed_string_1111 = MerkleTree::hash_text("Tetris5");

        let hashed_string_000 =
            MerkleTree::combine_hashes(&hashed_string_0000, &hashed_string_0001);
        let hashed_string_001 =
            MerkleTree::combine_hashes(&hashed_string_0010, &hashed_string_0011);
        let hashed_string_010 =
            MerkleTree::combine_hashes(&hashed_string_0100, &hashed_string_0101);
        let hashed_string_011 =
            MerkleTree::combine_hashes(&hashed_string_0110, &hashed_string_0111);

        let hashed_string_100 =
            MerkleTree::combine_hashes(&hashed_string_1000, &hashed_string_1001);
        let hashed_string_101 =
            MerkleTree::combine_hashes(&hashed_string_1010, &hashed_string_1011);
        let hashed_string_110 =
            MerkleTree::combine_hashes(&hashed_string_1100, &hashed_string_1101);
        let hashed_string_111 =
            MerkleTree::combine_hashes(&hashed_string_1110, &hashed_string_1111);

        let hashed_string_00 = MerkleTree::combine_hashes(&hashed_string_000, &hashed_string_001);
        let hashed_string_01 = MerkleTree::combine_hashes(&hashed_string_010, &hashed_string_011);

        let hashed_string_10 = MerkleTree::combine_hashes(&hashed_string_100, &hashed_string_101);
        let hashed_string_11 = MerkleTree::combine_hashes(&hashed_string_110, &hashed_string_111);

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(31, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // 7d6aca7ece41a33246a1fe3d13dcf074b701aa43717a19a93047553fc38294b0
    }

    #[test]
    fn test_09_hash_function_works_correctly() {
        // Assert that hash function works correctly
        assert_eq!(
            MerkleTree::hash_text("Merkle Tree"),
            "cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952"
        );
        assert_eq!(
            MerkleTree::hash_text("Merkle Root"),
            "09b4b6987df5353bfe0055491ac474539691011d0e95ecdaf8ad06906504308b"
        );
        assert_eq!(
            MerkleTree::hash_text("Ralph Merkle"),
            "5a93dda4ddfe626b84b6ffdb6f4ee27da108a28762247359b9d25310c6f00736"
        );
    }

    #[test]
    fn test_10_combined_hash_function_works_correctly() {
        // Assert that the combine hashes function works as expected
        let hash_left = MerkleTree::hash_text("Merkle Tree");
        let hash_right = MerkleTree::hash_text("Merkle Root");
        assert_eq!(
            MerkleTree::combine_hashes(&hash_left, &hash_right),
            "c4f431efc6c50e3b703e11233dd219eaef584c24e4a4b76da22487eb74ec9258"
        );
        assert_eq!(
            MerkleTree::combine_hashes(&hash_right, &hash_left),
            "39d978a783e10f39b039ff6a022d7761f8bf74104d663717037e4825d86da10b"
        );
    }

    #[test]
    fn test_11_power_of_two_function_works_correctly() {
        assert!(MerkleTree::number_is_power_of_two(1.));
        assert!(MerkleTree::number_is_power_of_two(2.));
        assert!(MerkleTree::number_is_power_of_two(8.));
        assert!(MerkleTree::number_is_power_of_two(64.));
        assert!(MerkleTree::number_is_power_of_two(128.));
        assert!(MerkleTree::number_is_power_of_two(512.));
        assert!(MerkleTree::number_is_power_of_two(2048.));
    }

    #[test]
    fn test_12_proof_of_a_four_elements_tree_is_verified_correctly() {
        // Given a proof, a leaf of the tree, and the index of the leave, the proof verifies correctly
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());
        tree.add_unhashed("Game of Life".to_string());
        tree.add_unhashed("John Conway".to_string());

        assert!(tree.verify(
            vec![
                "5a93dda4ddfe626b84b6ffdb6f4ee27da108a28762247359b9d25310c6f00736".to_string(),
                "9630101c1c273a6c4714cc7388f35cd7f1b547bf3bc740caf3d943e33e0a9c37".to_string()
            ],
            "cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952".to_string(),
            &mut 0
        ))
    }

    #[test]
    fn test_13_proof_of_a_four_elements_tree_with_a_false_seed_doesnt_work() {
        // Given a proof, a leaf of the tree, and the index of the leave, the proof verifies correctly
        let mut tree = MerkleTree::new();
        tree.add_unhashed("Merkle Tree".to_string());
        tree.add_unhashed("Ralph Merkle".to_string());
        tree.add_unhashed("Game of Life".to_string());
        tree.add_unhashed("John Conway".to_string());

        assert!(!tree.verify(
            vec![
                "5a93dda4ddfe626b84b6ffdb6f4ee27da108a28762247359b9d25310c6f00736".to_string(),
                "9630101c1c273a6c4714cc7388f35cd7f1b547bf3bc740caf3d943e33e0a9c37".to_string()
            ],
            "not_a_seed".to_string(),
            &mut 0
        ))
    }

    #[test]
    fn test_14_build_creates_a_correct_tree() {
        // I can build a tree from an array, and it contains the elements

        let tree = MerkleTree::build(
            vec![
                "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb",
                "3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d",
                "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6",
            ],
            false,
        );

        assert!(tree.verify(
            vec![
                "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb".to_string(),
                "d50c873877f38fcbc56dbe836b9d979912efcb587ed8eea919372d403b5c2bd4".to_string()
            ],
            "3e23e8160039594a33894f6564e1b1348bbd7a0088d42c4acb73eeaed59c009d".to_string(),
            &mut 1
        ))
    }

    #[test]
    fn test_15_build_unhashed_creates_a_correct_tree() {
        // I can build a tree from an array, and it contains the elements

        let tree = MerkleTree::build(vec!["a", "b", "c", "d"], true);

        assert!(tree.verify(
            vec![
                "2e7d2c03a9507ae265ecf5b5356885a53393a2029d241394997265a1a25aefc6".to_string(),
                "62af5c3cb8da3e4f25061e829ebeea5c7513c54949115b1acc225930a90154da".to_string()
            ],
            "18ac3e7343f016890c510e93f935261169d9e3f565436429830faf0934f4f8e4".to_string(),
            &mut 3
        ))
    }

    #[test]
    fn test_16_proof_is_expected_in_a_two_depth_tree() {
        // The proof is the expected in a 2-depth tree
        let mut tree = MerkleTree::build(vec!["a", "b", "c", "d"], true);

        println!("{:?}", tree.elements);
        assert_eq!(
            vec![
                "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb".to_string(),
                "d3a0f1c792ccf7f1708d5422696263e35755a86917ea76ef9242bd4a8cf4891a".to_string()
            ],
            tree.generate_proof(&mut 1)
        );
    }

    #[test]
    fn test_17_proof_is_expected_in_a_three_depth_tree() {
        // The proof is the expected in a 3 depth tree
        let mut tree = MerkleTree::build(vec!["a", "b", "c", "d", "e", "f", "g", "h"], true);
        let mut index = 1;
        println!("{:?}", tree.elements);
        assert_eq!(
            vec![
                "ca978112ca1bbdcafac231b39a23dc4da786eff8147c4e72b9807785afee48bb".to_string(),
                "d3a0f1c792ccf7f1708d5422696263e35755a86917ea76ef9242bd4a8cf4891a".to_string(),
                "d6cf2ad3f66d0599d97346c6aad0f1081913df26d8b80e4ffa052e0a1f8391c6".to_string()
            ],
            tree.generate_proof(&mut index)
        );
    }

    #[test]
    fn test_18_tree_supports_long_unhashed_texts() {
        // Adds four unhashed long texts to the tree, there are seven elements in vector and tree root is result of hashing all

        let mut tree = MerkleTree::new();
        tree.add_unhashed("Aliquam quis semper dolor. Nam egestas pharetra enim, in aliquet leo eleifend id. Fusce lacinia quam at libero condimentum, vitae fringilla ex volutpat. Nunc sollicitudin est eu lectus mattis hendrerit. Nam sit amet tristique sapien. Pellentesque sed lorem diam. Ut eu tempor elit.".to_string());
        tree.add_unhashed("Ut augue ligula, tincidunt ut eleifend vitae, mattis nec lacus. Nunc id nunc ut diam dignissim varius. Etiam tincidunt iaculis purus et rhoncus. Curabitur eu venenatis ipsum. Nam lobortis, massa quis ultrices vulputate, magna elit posuere turpis, ut accumsan nunc dolor sed justo.".to_string());
        tree.add_unhashed("Donec blandit viverra mi. Phasellus dapibus id neque quis eleifend. In sed metus laoreet tellus egestas fermentum ac vitae metus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum eget nisl id nisl accumsan consequat vitae a leo.".to_string());
        tree.add_unhashed("Integer efficitur mollis justo in volutpat. Duis ac luctus libero. Donec scelerisque vestibulum sagittis. Mauris iaculis enim nec lectus condimentum porttitor. Fusce pharetra lobortis ipsum a vulputate.".to_string());

        let hashed_string_00 = MerkleTree::hash_text("Aliquam quis semper dolor. Nam egestas pharetra enim, in aliquet leo eleifend id. Fusce lacinia quam at libero condimentum, vitae fringilla ex volutpat. Nunc sollicitudin est eu lectus mattis hendrerit. Nam sit amet tristique sapien. Pellentesque sed lorem diam. Ut eu tempor elit.");
        let hashed_string_01 = MerkleTree::hash_text("Ut augue ligula, tincidunt ut eleifend vitae, mattis nec lacus. Nunc id nunc ut diam dignissim varius. Etiam tincidunt iaculis purus et rhoncus. Curabitur eu venenatis ipsum. Nam lobortis, massa quis ultrices vulputate, magna elit posuere turpis, ut accumsan nunc dolor sed justo.");

        let hashed_string_10 = MerkleTree::hash_text("Donec blandit viverra mi. Phasellus dapibus id neque quis eleifend. In sed metus laoreet tellus egestas fermentum ac vitae metus. Class aptent taciti sociosqu ad litora torquent per conubia nostra, per inceptos himenaeos. Vestibulum eget nisl id nisl accumsan consequat vitae a leo.");
        let hashed_string_11 = MerkleTree::hash_text("Integer efficitur mollis justo in volutpat. Duis ac luctus libero. Donec scelerisque vestibulum sagittis. Mauris iaculis enim nec lectus condimentum porttitor. Fusce pharetra lobortis ipsum a vulputate.");

        let hashed_string_0 = MerkleTree::combine_hashes(&hashed_string_00, &hashed_string_01);
        let hashed_string_1 = MerkleTree::combine_hashes(&hashed_string_10, &hashed_string_11);

        let hashed_string_root = MerkleTree::combine_hashes(&hashed_string_0, &hashed_string_1);

        assert_eq!(7, tree.elements.len());
        assert_eq!(hashed_string_root, tree.elements[0]);
        // c567f133613aac1e0f011569c65daf490adbb87a87db7246ac045b79c64d1460
    }
}
