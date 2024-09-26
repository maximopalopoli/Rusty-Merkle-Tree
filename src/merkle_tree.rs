



pub struct MerkleTree {
    tree: Vec<String>,
    depth: u8,
}

impl MerkleTree {
    fn new () -> Self {
        let tree = Vec::new();
        MerkleTree { tree, depth:1 }
    }
}


#[cfg(test)]
mod tests {
    use super::MerkleTree;

    #[test]
    fn test_01() {
        // Create a MerkleTree and begins with an empty vec and an initial depth of 1
        let tree = MerkleTree::new();
        assert_eq!(1, tree.depth);
        assert_eq!(0, tree.tree.len());
    }

}
