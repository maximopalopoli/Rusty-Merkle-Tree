pub mod merkle_tree;
use merkle_tree::MerkleTree;

fn process_comands(line: String, tree: &mut MerkleTree) {
    let args: Vec<&str> = line.split_ascii_whitespace().collect();
    println!("{args:?}");

    if args[0] == "--help" {
        println!("  build - Usage: build");
        println!("  add-raw - Usage: add-raw raw-text");
        println!("  add - Usage: add 32-bytes-hash");
        println!("  verify - Usage: verify proof1 proof2 ... proofN seed index");
    }

    if args[0] == "build" { // Shortcut to test, but could iterate the args after build
        tree.add_raw("Merkle Tree".to_string());
        tree.add_raw("Ralph Merkle".to_string());
        tree.add_raw("Game of Life".to_string());
        tree.add_raw("John Conway".to_string());
    }

    if args[0] == "add-raw" { // Usage: add-raw raw-text
        tree.add_raw(args[1].to_string());
        // example: add John-Conway
    }
    // Note: Still doesn't support blank_spaces in raw-text. TODO

    if args[0] == "add" { // Usage: add-raw raw-text
        tree.add(args[1].to_string());
        // example: add cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952
    }

    if args[0] == "verify" { // Usage: verify proof1 proof2 ... proofN seed index
        let mut proof = Vec::new();
        for i in 1..(1 + tree.depth()){
            proof.push(args[i].to_string());
        }
        let leaf = args[1+tree.depth()].to_string();

        let mut index: i32 = args[1+tree.depth()+1].to_string().parse().unwrap();

        if tree.verify(proof, leaf, &mut index) {
            println!("Proof has been verified");
        } else {
            println!("Proof has not been verified");
        }
        // example: verify 5a93dda4ddfe626b84b6ffdb6f4ee27da108a28762247359b9d25310c6f00736 9630101c1c273a6c4714cc7388f35cd7f1b547bf3bc740caf3d943e33e0a9c37 cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952 0
    }
}

fn main() {
    println!("Welcome to this Merkle Tree simulator. Type --help to list the available commands");
    let mut tree = MerkleTree::new();
    loop {
        let mut input_line = String::new();
        let bytes_read = std::io::stdin().read_line(&mut input_line).unwrap();
        println!("{bytes_read}");
        if bytes_read <= 1 {
            return;
        }
        // TODO: Error checking instead of .unwrap()
        
        process_comands(input_line, &mut tree);
    }    
}
