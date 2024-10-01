pub mod merkle_tree;
use std::num::ParseIntError;

use merkle_tree::MerkleTree;

fn process_comands(line: String, tree: &mut MerkleTree) -> Result<(), ParseIntError> {
    let args: Vec<&str> = line.split_ascii_whitespace().collect();

    match args[0] {
        "--help" => {
            println!("  build - Usage: build <hash-1> <hash-2> ... <hash-n>");
            println!("  build-raw - Usage: build-raw <raw-text-1> <raw-text-2> ... <raw-text-n>");
            println!("  add-raw - Usage: add-raw raw-text");
            println!("  add - Usage: add 32-bytes-hash");
            println!("  verify - Usage: verify proof1 proof2 ... proofN seed index");
            println!("  proof - Usage: proof index");
            println!("  print - Usage: print");
        },
        "build" => {
            // Usage: build <hash-1> <hash-2> ... <hash-n>
            let hashes: Vec<&str> = Vec::from(&args[1..]);
            *tree = MerkleTree::build(hashes, false);
        },
        "build-raw" => {
            // Usage: build <raw-text-1> <raw-text-2> ... <raw-text-n>
            let hashes: Vec<&str> = Vec::from(&args[1..]);
            *tree = MerkleTree::build(hashes, true);
        },
        "add" => {
            // Usage: add hash
            tree.add(args[1].to_string());
            // example: add cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952
        },
        "add-raw" => {
            // Usage: add-raw raw-text
            tree.add_raw(args[1].to_string());
            // example: add John-Conway
        },
        "verify" => {
            // Usage: verify proof1 proof2 ... proofN seed index
            let mut proof = Vec::new();
            for item in args.iter().skip(1).take(tree.depth()) {
                proof.push((*item).to_string());
            }
            let leaf = args[1 + tree.depth()].to_string();

            let mut index = args[1 + tree.depth() + 1].to_string().parse()?;

            if tree.verify(proof, leaf, &mut index) {
                println!("Proof has been verified");
            } else {
                println!("Proof has not been verified");
            }
            // example: verify 5a93dda4ddfe626b84b6ffdb6f4ee27da108a28762247359b9d25310c6f00736 9630101c1c273a6c4714cc7388f35cd7f1b547bf3bc740caf3d943e33e0a9c37 cbcbd2ab218ea6a894d3a93e0e83ed0cc0286597a826d3ef4ff3a360e22a7952 0
        },
        "proof" => {
            // Usage: proof <index>
            let mut index = args[1].parse::<usize>()?;
            let response = tree.generate_proof(&mut index);
            for hash in response {
                print!("{hash} ");
            }
            println!();
        },
        "print" => {
            tree.print();
        },
        _ => {
            println!("Command not recognized, type --help to see the available commands");
        }
    }
    Ok(())
}

fn main() {
    println!("Welcome to this Merkle Tree simulator. Type --help to list the available commands");
    let mut tree = MerkleTree::new();
    loop {
        let mut input_line = String::new();
        if let Ok(bytes_read) = std::io::stdin().read_line(&mut input_line) {
            if bytes_read <= 1 {
                return;
            }
        } else {
            println!("Could not receive from stdin");
            return;
        }

        if let Err(e) = process_comands(input_line, &mut tree) {
            println!("{}", e)
        }
    }
}
