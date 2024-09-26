use sha2::{Digest, Sha256};




pub struct MerkleTree {
    tree: Vec<String>,
    depth: u8,
}

impl MerkleTree {
    pub fn new () -> Self {
        let tree = Vec::new();
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
        self.depth += 1;
        self.tree.push(hashed_string);
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
        assert_eq!(0, tree.tree.len());
    }

    #[test]
    fn test_02 () {
        // Add a raw text to the tree, grows depth and tree now contains the hash
        let mut tree = MerkleTree::new();
        tree.add_raw("Merkle Tree".to_string());  
        
        assert_eq!(1, tree.depth);
        assert_eq!("cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952", tree.tree[0]);
    }

}
